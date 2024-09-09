use std::fmt::Formatter;
use std::sync::Arc;

use chrono_tz::Tz;
use crossbeam::queue::SegQueue;
use serde::{Deserialize, Serialize};
use thiserror::Error;
use tokio::{io::AsyncReadExt, net::TcpStream, sync::mpsc};
use tokio::net::tcp::OwnedReadHalf;
use tokio::task::JoinHandle;

use crate::{
    account::Tag,
    comm::Writer,
    constants, decode,
    execution::Filter,
    order::{Executable, Order},
    payload::ExchangeId,
    reader::Reader,
};
use crate::contract::{ContractId, Query, Security};
use crate::decode::DecodeError;
use crate::exchange::Routing;
use crate::market_data::{
    histogram, historical_bar, historical_ticks, live_bar, live_data, live_ticks,
    updating_historical_bar,
};
use crate::message::{In, Out, ToClient, ToWrapper};
use crate::wrapper::{CancelToken, Initializer, LocalInitializer, LocalWrapper, Wrapper};

// ======================================
// === Types for Handling Config File ===
// ======================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
struct Ports {
    tws_live: u16,
    tws_paper: u16,
    gateway_live: u16,
    gateway_paper: u16,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
struct Config {
    address: std::net::Ipv4Addr,
    #[serde(alias = "Ports")]
    ports: Ports,
}

#[derive(Debug, Error)]
/// Error type representing the ways that a `config.toml` file can be invalid
pub enum ParseConfigFileError {
    #[error("Failed to read config.toml file. Cause: {0}")]
    /// The OS failed to read the file
    File(#[from] std::io::Error),
    #[error("Failed to parse config.toml file. Cause: {0}")]
    /// The required `TOMl` data was invalid or missing
    Toml(#[from] toml::de::Error),
}

impl Config {
    #[inline]
    fn new(path: impl AsRef<std::path::Path>) -> Result<Self, ParseConfigFileError> {
        Ok(toml::from_str(std::fs::read_to_string(path)?.as_str())?)
    }
}

// =======================================
// === Client Builder and Helper Types ===
// =======================================

//noinspection SpellCheckingInspection
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
/// Represents the two types of connections to IBKR's trading systems.
pub enum Mode {
    /// A live trading connection with real money.
    Live,
    /// A paper (simulated) trading connection with fake money.
    Paper,
}

/// For safety, the default [`Mode`] is a paper trading environment
///
/// # Examples
/// ```
/// # use ibapi::client::Mode;
/// let m = Mode::default();
/// assert_eq!(m, Mode::Paper);
/// ```
impl Default for Mode {
    #[inline]
    fn default() -> Self {
        Self::Paper
    }
}

impl std::fmt::Display for Mode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Paper => write!(f, "Paper"),
            Self::Live => write!(f, "Live"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
/// Represents the two platforms that facilitate trading with IBKR's systems. The two hosts are
/// indistinguishable from the perspective of an API application.
pub enum Host {
    /// IBKR's flagship Trader Workstation desktop application.
    Tws,
    /// A leaner GUI that requires less performance overhead but has no monitoring of sophisticated
    /// graphical capabilities.
    Gateway,
}

impl std::fmt::Display for Host {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Gateway => write!(f, "IB Gateway"),
            Self::Tws => write!(f, "Trader Workstation (TWS)"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Inner {
    ConfigFile {
        mode: Mode,
        host: Host,
        config: Config,
    },
    Manual {
        port: u16,
        address: std::net::Ipv4Addr,
    },
}

#[derive(Debug, Error)]
/// An error type for potential failure of the initial connection to the IBKR API
pub enum ConnectionError {
    #[error("Failed to initiate connection to IBKR API: Failed to parse server version.")]
    /// Failed to parse server version
    ServerVersion,
    #[error("Failed to initiate connection to IBKR API: Invalid timezone in connection time.")]
    /// Invalid timezone in connection time,
    TimeZone,
    #[error("Failed to initiate connection to IBKR API: Invalid datetime in connection time.")]
    /// Invalid datetime in connection time
    DateTime,
    #[error("Failed to initiate connection to IBKR API: {0}.")]
    /// IO error when attempting to initiate TCP connection
    Io(#[from] std::io::Error),
    #[error(
        "Failed to initiate connection to IBKR API: Required buffer sizes exceeds usize::MAX."
    )]
    /// Occurs if required buffer size exceeds `usize::MAX`
    InvalidBufferSize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
/// Facilitates the creation of a new connection to IBKR's trading systems.
///
/// Each connection requires a TCP port and address with which to connect to the appropriate IBKR
/// platform. This information is communicated by either: 1) Manually specifying the parameters in
/// [`Builder::manual`] or 2) Automatically looking them up in the config.toml file by specifying a
///  [`Mode`] and [`Host`] in [`Builder::from_config_file`].
pub struct Builder(Inner);

impl Builder {
    #[inline]
    /// Creates a new [`Builder`] from a mode, host, and (optionally) a path to "config.toml"
    ///
    /// # Arguments
    /// * `mode` - Specifies whether the builder will create a live (real money) or paper (fake
    ///   money) trading environment.
    /// * `host` - Specifies the platform used for communication with IBKR's trading systems.
    /// * `path` - An optional string slice that overrides the default location of "./config.toml".
    ///
    /// # Errors
    /// Returns any error encountered while reading and parsing the config file.
    pub fn from_config_file(
        mode: Mode,
        host: Host,
        path: &Option<impl AsRef<std::path::Path>>,
    ) -> Result<Self, ParseConfigFileError> {
        let path = path.as_ref().map_or(
            std::path::Path::new("./config.toml"),
            AsRef::<std::path::Path>::as_ref,
        );
        let config = Config::new(path)?;

        Ok(Self(Inner::ConfigFile { mode, host, config }))
    }

    #[must_use]
    #[inline]
    /// Creates a new [`Builder`] from a TCP port and address.
    ///
    /// # Arguments
    /// * `port` - The TCP port with which to connect to IBKR's trading systems.
    /// * `address` - The IP address with which to connect to IBKR's trading systems.
    pub fn manual(port: u16, address: Option<std::net::Ipv4Addr>) -> Self {
        Self(Inner::Manual {
            port,
            address: address.unwrap_or(std::net::Ipv4Addr::LOCALHOST),
        })
    }

    /// Initiates a connection to IBKR's trading systems and returns a [`Client`].
    ///
    /// # Arguments
    /// * `client_id` - A unique ID for IBKR's systems to distinguish between clients
    ///
    /// # Errors
    /// This function will error if any of the following occurs:
    /// 1) An error occurs while initiating a TCP connection on the port and address specified in
    ///    either [`Builder::manual`] or in the "config.toml" file specified in
    ///    [`Builder::from_config_file`].
    /// 2) An error occurs while reading or writing the handshake message that initiates a
    ///    connection with IBKR's trading systems.
    ///
    /// # Returns
    /// An inactive [`Client`] that will become active upon calling [`Client::local`] or
    /// [`Client::remote`].
    pub async fn connect(
        &self,
        client_id: i64,
    ) -> Result<Client<indicators::Inactive>, ConnectionError> {
        let (mode, host, port, address) = match self.0 {
            Inner::ConfigFile { mode, host, config } => (
                Some(mode),
                Some(host),
                match (mode, host) {
                    (Mode::Live, Host::Tws) => config.ports.tws_live,
                    (Mode::Live, Host::Gateway) => config.ports.gateway_live,
                    (Mode::Paper, Host::Tws) => config.ports.tws_paper,
                    (Mode::Paper, Host::Gateway) => config.ports.gateway_paper,
                },
                config.address,
            ),
            Inner::Manual { port, address } => (None, None, port, address),
        };

        let (mut reader, writer) = TcpStream::connect((address, port)).await?.into_split();

        let mut writer = Writer::new(writer);
        writer.add_prefix("API\0")?;
        writer.add_body(format!(
            "v{}..{}",
            constants::MIN_CLIENT_VERSION,
            constants::MAX_CLIENT_VERSION
        ))?;
        writer.send().await?;

        let mut buf = bytes::BytesMut::with_capacity(
            usize::try_from(reader.read_u32().await?)
                .map_err(|_| ConnectionError::InvalidBufferSize)?,
        );
        reader.read_buf(&mut buf).await?;
        let resp = buf.into_iter().map(char::from).collect::<String>();
        let mut params = resp.split('\0');

        let server_version = params
            .next()
            .ok_or(ConnectionError::ServerVersion)?
            .parse()
            .map_err(|_| ConnectionError::ServerVersion)?;
        let (conn_time, tz) = chrono::NaiveDateTime::parse_and_remainder(
            params.next().ok_or(ConnectionError::DateTime)?,
            "%Y%m%d %T",
        )
        .map_err(|_| ConnectionError::DateTime)?;
        let conn_time = conn_time
            .and_local_timezone(
                tz.trim()
                    .parse::<chrono_tz::Tz>()
                    .map_err(|_| ConnectionError::TimeZone)?,
            )
            .single()
            .ok_or(ConnectionError::TimeZone)?;

        let mut client = Client {
            mode,
            host,
            port,
            address,
            client_id,
            server_version,
            conn_time,
            writer,
            status: indicators::Inactive { reader },
        };
        client.start_api().await?;

        Ok(client)
    }
}

// ===============================
// === Status Trait Definition ===
// ===============================

#[allow(clippy::module_name_repetitions)]
/// An active client, which can request information from IBKR trading systems.
pub type ActiveClient = Client<indicators::Active>;

type IntoActive = (
    Client<indicators::Active>,
    mpsc::Sender<ToClient>,
    mpsc::Receiver<ToWrapper>,
    Arc<SegQueue<Vec<String>>>,
    std::collections::VecDeque<Vec<String>>,
);

type LoopParams = (
    Arc<SegQueue<Vec<String>>>,
    mpsc::Sender<ToClient>,
    mpsc::Receiver<ToWrapper>,
    std::collections::VecDeque<Vec<String>>,
);

#[inline]
#[allow(clippy::too_many_lines)]
async fn decode_msg_remote<W>(
    fields: Vec<String>,
    remote: &mut W,
    tx: &mut mpsc::Sender<ToClient>,
    rx: &mut mpsc::Receiver<ToWrapper>,
) where
    W: Wrapper,
{
    let status = match fields.first() {
        None => Err(DecodeError::MissingData {
            field_name: "In-message identifier",
        }
        .with_context("None")),
        Some(s) => match s.parse() {
            Ok(In::TickPrice) => decode::Remote::tick_price_msg(&mut fields.into_iter(), remote)
                .await
                .map_err(|e| e.with_context("tick price msg")),
            Ok(In::TickSize) => decode::Remote::tick_size_msg(&mut fields.into_iter(), remote)
                .await
                .map_err(|e| e.with_context("tick size msg")),
            Ok(In::OrderStatus) => {
                decode::Remote::order_status_msg(&mut fields.into_iter(), remote)
                    .await
                    .map_err(|e| e.with_context("order status msg"))
            }
            Ok(In::ErrMsg) => decode::Remote::err_msg_msg(&mut fields.into_iter(), remote)
                .await
                .map_err(|e| e.with_context("err msg msg")),
            Ok(In::OpenOrder) => decode::Remote::open_order_msg(&mut fields.into_iter(), remote)
                .await
                .map_err(|e| e.with_context("open order msg")),
            Ok(In::AcctValue) => decode::Remote::acct_value_msg(&mut fields.into_iter(), remote)
                .await
                .map_err(|e| e.with_context("acct value msg")),
            Ok(In::PortfolioValue) => {
                decode::Remote::portfolio_value_msg(&mut fields.into_iter(), remote)
                    .await
                    .map_err(|e| e.with_context("portfolio value msg"))
            }
            Ok(In::AcctUpdateTime) => {
                decode::Remote::acct_update_time_msg(&mut fields.into_iter(), remote)
                    .await
                    .map_err(|e| e.with_context("acct update time msg"))
            }
            Ok(In::NextValidId) => {
                decode::Remote::next_valid_id_msg(&mut fields.into_iter(), remote, tx, rx)
                    .await
                    .map_err(|e| e.with_context("next valid id msg"))
            }
            Ok(In::ContractData) => {
                decode::Remote::contract_data_msg(&mut fields.into_iter(), remote, tx, rx)
                    .await
                    .map_err(|e| e.with_context("contract data msg"))
            }
            Ok(In::ExecutionData) => {
                decode::Remote::execution_data_msg(&mut fields.into_iter(), remote)
                    .await
                    .map_err(|e| e.with_context("execution data msg"))
            }
            Ok(In::MarketDepth) => {
                decode::Remote::market_depth_msg(&mut fields.into_iter(), remote)
                    .await
                    .map_err(|e| e.with_context("market depth msg"))
            }
            Ok(In::MarketDepthL2) => {
                decode::Remote::market_depth_l2_msg(&mut fields.into_iter(), remote)
                    .await
                    .map_err(|e| e.with_context("market depth l2 msg"))
            }
            Ok(In::NewsBulletins) => {
                decode::Remote::news_bulletins_msg(&mut fields.into_iter(), remote)
                    .await
                    .map_err(|e| e.with_context("news bulletins msg"))
            }
            Ok(In::ManagedAccts) => {
                decode::Remote::managed_accts_msg(&mut fields.into_iter(), remote, tx, rx)
                    .await
                    .map_err(|e| e.with_context("managed accounts msg"))
            }
            Ok(In::ReceiveFa) => decode::Remote::receive_fa_msg(&mut fields.into_iter(), remote)
                .await
                .map_err(|e| e.with_context("receive fa msg")),
            Ok(In::HistoricalData) => {
                decode::Remote::historical_data_msg(&mut fields.into_iter(), remote)
                    .await
                    .map_err(|e| e.with_context("historical data msg"))
            }
            Ok(In::BondContractData) => {
                decode::Remote::bond_contract_data_msg(&mut fields.into_iter(), remote)
                    .await
                    .map_err(|e| e.with_context("bond contract data msg"))
            }
            Ok(In::ScannerParameters) => {
                decode::Remote::scanner_parameters_msg(&mut fields.into_iter(), remote)
                    .await
                    .map_err(|e| e.with_context("scanner parameters msg"))
            }
            Ok(In::ScannerData) => {
                decode::Remote::scanner_data_msg(&mut fields.into_iter(), remote)
                    .await
                    .map_err(|e| e.with_context("scanner data msg"))
            }
            Ok(In::TickOptionComputation) => {
                decode::Remote::tick_option_computation_msg(&mut fields.into_iter(), remote)
                    .await
                    .map_err(|e| e.with_context("tick option computation msg"))
            }
            Ok(In::TickGeneric) => {
                decode::Remote::tick_generic_msg(&mut fields.into_iter(), remote)
                    .await
                    .map_err(|e| e.with_context("tick generic msg"))
            }
            Ok(In::TickString) => decode::Remote::tick_string_msg(&mut fields.into_iter(), remote)
                .await
                .map_err(|e| e.with_context("tick string msg")),
            Ok(In::TickEfp) => decode::Remote::tick_efp_msg(&mut fields.into_iter(), remote)
                .await
                .map_err(|e| e.with_context("tick efp msg")),
            Ok(In::CurrentTime) => {
                decode::Remote::current_time_msg(&mut fields.into_iter(), remote)
                    .await
                    .map_err(|e| e.with_context("current time msg"))
            }
            Ok(In::RealTimeBars) => {
                decode::Remote::real_time_bars_msg(&mut fields.into_iter(), remote)
                    .await
                    .map_err(|e| e.with_context("real time bars msg"))
            }
            Ok(In::FundamentalData) => {
                decode::Remote::fundamental_data_msg(&mut fields.into_iter(), remote)
                    .await
                    .map_err(|e| e.with_context("fundamental data msg"))
            }
            Ok(In::ContractDataEnd) => {
                decode::Remote::contract_data_end_msg(&mut fields.into_iter(), remote)
                    .await
                    .map_err(|e| e.with_context("contract data end msg"))
            }
            Ok(In::OpenOrderEnd) => {
                decode::Remote::open_order_end_msg(&mut fields.into_iter(), remote)
                    .await
                    .map_err(|e| e.with_context("open order end msg"))
            }
            Ok(In::AcctDownloadEnd) => {
                decode::Remote::acct_download_end_msg(&mut fields.into_iter(), remote)
                    .await
                    .map_err(|e| e.with_context("acct download end msg"))
            }
            Ok(In::ExecutionDataEnd) => {
                decode::Remote::execution_data_end_msg(&mut fields.into_iter(), remote)
                    .await
                    .map_err(|e| e.with_context("execution data end msg"))
            }
            Ok(In::DeltaNeutralValidation) => {
                decode::Remote::delta_neutral_validation_msg(&mut fields.into_iter(), remote)
                    .await
                    .map_err(|e| e.with_context("delta neutral validation msg"))
            }
            Ok(In::TickSnapshotEnd) => {
                decode::Remote::tick_snapshot_end_msg(&mut fields.into_iter(), remote)
                    .await
                    .map_err(|e| e.with_context("tick snapshot end msg"))
            }
            Ok(In::MarketDataType) => {
                decode::Remote::market_data_type_msg(&mut fields.into_iter(), remote)
                    .await
                    .map_err(|e| e.with_context("market data type msg"))
            }
            Ok(In::CommissionReport) => {
                decode::Remote::commission_report_msg(&mut fields.into_iter(), remote)
                    .await
                    .map_err(|e| e.with_context("commission report msg"))
            }
            Ok(In::PositionData) => {
                decode::Remote::position_data_msg(&mut fields.into_iter(), remote)
                    .await
                    .map_err(|e| e.with_context("position data msg"))
            }
            Ok(In::PositionEnd) => {
                decode::Remote::position_end_msg(&mut fields.into_iter(), remote)
                    .await
                    .map_err(|e| e.with_context("position end msg"))
            }
            Ok(In::AccountSummary) => {
                decode::Remote::account_summary_msg(&mut fields.into_iter(), remote)
                    .await
                    .map_err(|e| e.with_context("account summary msg"))
            }
            Ok(In::AccountSummaryEnd) => {
                decode::Remote::account_summary_end_msg(&mut fields.into_iter(), remote)
                    .await
                    .map_err(|e| e.with_context("account summary end msg"))
            }
            Ok(In::VerifyMessageApi) => {
                decode::Remote::verify_message_api_msg(&mut fields.into_iter(), remote)
                    .await
                    .map_err(|e| e.with_context("verify message api msg"))
            }
            Ok(In::VerifyCompleted) => {
                decode::Remote::verify_completed_msg(&mut fields.into_iter(), remote)
                    .await
                    .map_err(|e| e.with_context("verify completed msg"))
            }
            Ok(In::DisplayGroupList) => {
                decode::Remote::display_group_list_msg(&mut fields.into_iter(), remote)
                    .await
                    .map_err(|e| e.with_context("display group list msg"))
            }
            Ok(In::DisplayGroupUpdated) => {
                decode::Remote::display_group_updated_msg(&mut fields.into_iter(), remote)
                    .await
                    .map_err(|e| e.with_context("display group updated msg"))
            }
            Ok(In::VerifyAndAuthMessageApi) => {
                decode::Remote::verify_and_auth_message_api_msg(&mut fields.into_iter(), remote)
                    .await
                    .map_err(|e| e.with_context("verify and auth message api msg"))
            }
            Ok(In::VerifyAndAuthCompleted) => {
                decode::Remote::verify_and_auth_completed_msg(&mut fields.into_iter(), remote)
                    .await
                    .map_err(|e| e.with_context("verify and auth completed msg"))
            }
            Ok(In::PositionMulti) => {
                decode::Remote::position_multi_msg(&mut fields.into_iter(), remote)
                    .await
                    .map_err(|e| e.with_context("position multi msg"))
            }
            Ok(In::PositionMultiEnd) => {
                decode::Remote::position_multi_end_msg(&mut fields.into_iter(), remote)
                    .await
                    .map_err(|e| e.with_context("position multi end msg"))
            }
            Ok(In::AccountUpdateMulti) => {
                decode::Remote::account_update_multi_msg(&mut fields.into_iter(), remote)
                    .await
                    .map_err(|e| e.with_context("account update multi msg"))
            }
            Ok(In::AccountUpdateMultiEnd) => {
                decode::Remote::account_update_multi_end_msg(&mut fields.into_iter(), remote)
                    .await
                    .map_err(|e| e.with_context("account update multi end msg"))
            }
            Ok(In::SecurityDefinitionOptionParameter) => {
                decode::Remote::security_definition_option_parameter_msg(
                    &mut fields.into_iter(),
                    remote,
                )
                .await
                .map_err(|e| e.with_context("security definition option parameter msg"))
            }
            Ok(In::SecurityDefinitionOptionParameterEnd) => {
                decode::Remote::security_definition_option_parameter_end_msg(
                    &mut fields.into_iter(),
                    remote,
                )
                .await
                .map_err(|e| e.with_context("security definition option parameter end msg"))
            }
            Ok(In::SoftDollarTiers) => {
                decode::Remote::soft_dollar_tiers_msg(&mut fields.into_iter(), remote)
                    .await
                    .map_err(|e| e.with_context("soft dollar tiers msg"))
            }
            Ok(In::FamilyCodes) => {
                decode::Remote::family_codes_msg(&mut fields.into_iter(), remote)
                    .await
                    .map_err(|e| e.with_context("family codes msg"))
            }
            Ok(In::SymbolSamples) => {
                decode::Remote::symbol_samples_msg(&mut fields.into_iter(), remote)
                    .await
                    .map_err(|e| e.with_context("symbol samples msg"))
            }
            Ok(In::MktDepthExchanges) => {
                decode::Remote::mkt_depth_exchanges_msg(&mut fields.into_iter(), remote)
                    .await
                    .map_err(|e| e.with_context("mkt depth exchanges msg"))
            }
            Ok(In::TickReqParams) => {
                decode::Remote::tick_req_params_msg(&mut fields.into_iter(), remote)
                    .await
                    .map_err(|e| e.with_context("tick req params msg"))
            }
            Ok(In::SmartComponents) => {
                decode::Remote::smart_components_msg(&mut fields.into_iter(), remote)
                    .await
                    .map_err(|e| e.with_context("smart components msg"))
            }
            Ok(In::NewsArticle) => {
                decode::Remote::news_article_msg(&mut fields.into_iter(), remote)
                    .await
                    .map_err(|e| e.with_context("news article msg"))
            }
            Ok(In::TickNews) => decode::Remote::tick_news_msg(&mut fields.into_iter(), remote)
                .await
                .map_err(|e| e.with_context("tick news msg")),
            Ok(In::NewsProviders) => {
                decode::Remote::news_providers_msg(&mut fields.into_iter(), remote)
                    .await
                    .map_err(|e| e.with_context("news providers msg"))
            }
            Ok(In::HistoricalNews) => {
                decode::Remote::historical_news_msg(&mut fields.into_iter(), remote)
                    .await
                    .map_err(|e| e.with_context("historical news msg"))
            }
            Ok(In::HistoricalNewsEnd) => {
                decode::Remote::historical_news_end_msg(&mut fields.into_iter(), remote)
                    .await
                    .map_err(|e| e.with_context("historical news end msg"))
            }
            Ok(In::HeadTimestamp) => {
                decode::Remote::head_timestamp_msg(&mut fields.into_iter(), remote)
                    .await
                    .map_err(|e| e.with_context("head timestamp msg"))
            }
            Ok(In::HistogramData) => {
                decode::Remote::histogram_data_msg(&mut fields.into_iter(), remote)
                    .await
                    .map_err(|e| e.with_context("histogram data msg"))
            }
            Ok(In::HistoricalDataUpdate) => {
                decode::Remote::historical_data_update_msg(&mut fields.into_iter(), remote)
                    .await
                    .map_err(|e| e.with_context("historical data update msg"))
            }
            Ok(In::RerouteMktDataReq) => {
                decode::Remote::reroute_mkt_data_req_msg(&mut fields.into_iter(), remote)
                    .await
                    .map_err(|e| e.with_context("reroute mkt data req msg"))
            }
            Ok(In::RerouteMktDepthReq) => {
                decode::Remote::reroute_mkt_depth_req_msg(&mut fields.into_iter(), remote)
                    .await
                    .map_err(|e| e.with_context("reroute mkt depth req msg"))
            }
            Ok(In::MarketRule) => decode::Remote::market_rule_msg(&mut fields.into_iter(), remote)
                .await
                .map_err(|e| e.with_context("market rule msg")),
            Ok(In::Pnl) => decode::Remote::pnl_msg(&mut fields.into_iter(), remote)
                .await
                .map_err(|e| e.with_context("pnl msg")),
            Ok(In::PnlSingle) => decode::Remote::pnl_single_msg(&mut fields.into_iter(), remote)
                .await
                .map_err(|e| e.with_context("pnl single msg")),
            Ok(In::HistoricalTicks) => {
                decode::Remote::historical_ticks_midpoint_msg(&mut fields.into_iter(), remote)
                    .await
                    .map_err(|e| e.with_context("historical ticks msg"))
            }
            Ok(In::HistoricalTicksBidAsk) => {
                decode::Remote::historical_ticks_bid_ask_msg(&mut fields.into_iter(), remote)
                    .await
                    .map_err(|e| e.with_context("historical ticks bid ask msg"))
            }
            Ok(In::HistoricalTicksLast) => {
                decode::Remote::historical_ticks_last_msg(&mut fields.into_iter(), remote)
                    .await
                    .map_err(|e| e.with_context("historical ticks last msg"))
            }
            Ok(In::TickByTick) => decode::Remote::tick_by_tick_msg(&mut fields.into_iter(), remote)
                .await
                .map_err(|e| e.with_context("tick by tick msg")),
            Ok(In::OrderBound) => decode::Remote::order_bound_msg(&mut fields.into_iter(), remote)
                .await
                .map_err(|e| e.with_context("order bound msg")),
            Ok(In::CompletedOrder) => {
                decode::Remote::completed_order_msg(&mut fields.into_iter(), remote)
                    .await
                    .map_err(|e| e.with_context("completed order msg"))
            }
            Ok(In::CompletedOrdersEnd) => {
                decode::Remote::completed_orders_end_msg(&mut fields.into_iter(), remote)
                    .await
                    .map_err(|e| e.with_context("completed orders end msg"))
            }
            Ok(In::ReplaceFaEnd) => {
                decode::Remote::replace_fa_end_msg(&mut fields.into_iter(), remote)
                    .await
                    .map_err(|e| e.with_context("replace fa end msg"))
            }
            Ok(In::WshMetaData) => {
                decode::Remote::wsh_meta_data_msg(&mut fields.into_iter(), remote)
                    .await
                    .map_err(|e| e.with_context("wsh meta data msg"))
            }
            Ok(In::WshEventData) => {
                decode::Remote::wsh_event_data_msg(&mut fields.into_iter(), remote)
                    .await
                    .map_err(|e| e.with_context("wsh event data msg"))
            }
            Ok(In::HistoricalSchedule) => {
                decode::Remote::historical_schedule_msg(&mut fields.into_iter(), remote)
                    .await
                    .map_err(|e| e.with_context("historical schedule msg"))
            }
            Ok(In::UserInfo) => decode::Remote::user_info_msg(&mut fields.into_iter(), remote)
                .await
                .map_err(|e| e.with_context("user info msg")),
            Err(e) => Err(DecodeError::Other(e.0).with_context("invalid in msg")),
        },
    };
    match status {
        Ok(()) => (),
        Err(e) => {
            tokio::task::yield_now().await;
            println!("\x1B[31m{e}\x1B[0m");
        }
    }
}

#[inline]
#[allow(clippy::too_many_lines)]
async fn decode_msg_local<W>(
    fields: Vec<String>,
    local: &mut W,
    tx: &mut mpsc::Sender<ToClient>,
    rx: &mut mpsc::Receiver<ToWrapper>,
) where
    W: LocalWrapper,
{
    let status = match fields.first() {
        None => Err(DecodeError::MissingData {
            field_name: "In-message identifier",
        }
        .with_context("None")),
        Some(s) => match s.parse() {
            Ok(In::TickPrice) => decode::Local::tick_price_msg(&mut fields.into_iter(), local)
                .await
                .map_err(|e| e.with_context("tick price msg")),
            Ok(In::TickSize) => decode::Local::tick_size_msg(&mut fields.into_iter(), local)
                .await
                .map_err(|e| e.with_context("tick size msg")),
            Ok(In::OrderStatus) => decode::Local::order_status_msg(&mut fields.into_iter(), local)
                .await
                .map_err(|e| e.with_context("order status msg")),
            Ok(In::ErrMsg) => decode::Local::err_msg_msg(&mut fields.into_iter(), local)
                .await
                .map_err(|e| e.with_context("err msg msg")),
            Ok(In::OpenOrder) => decode::Local::open_order_msg(&mut fields.into_iter(), local)
                .await
                .map_err(|e| e.with_context("open order msg")),
            Ok(In::AcctValue) => decode::Local::acct_value_msg(&mut fields.into_iter(), local)
                .await
                .map_err(|e| e.with_context("acct value msg")),
            Ok(In::PortfolioValue) => {
                decode::Local::portfolio_value_msg(&mut fields.into_iter(), local)
                    .await
                    .map_err(|e| e.with_context("portfolio value msg"))
            }
            Ok(In::AcctUpdateTime) => {
                decode::Local::acct_update_time_msg(&mut fields.into_iter(), local)
                    .await
                    .map_err(|e| e.with_context("acct update time msg"))
            }
            Ok(In::NextValidId) => {
                decode::Local::next_valid_id_msg(&mut fields.into_iter(), local, tx, rx)
                    .await
                    .map_err(|e| e.with_context("next valid id msg"))
            }
            Ok(In::ContractData) => {
                decode::Local::contract_data_msg(&mut fields.into_iter(), local, tx, rx)
                    .await
                    .map_err(|e| e.with_context("contract data msg"))
            }
            Ok(In::ExecutionData) => {
                decode::Local::execution_data_msg(&mut fields.into_iter(), local)
                    .await
                    .map_err(|e| e.with_context("execution data msg"))
            }
            Ok(In::MarketDepth) => decode::Local::market_depth_msg(&mut fields.into_iter(), local)
                .await
                .map_err(|e| e.with_context("market depth msg")),
            Ok(In::MarketDepthL2) => {
                decode::Local::market_depth_l2_msg(&mut fields.into_iter(), local)
                    .await
                    .map_err(|e| e.with_context("market depth l2 msg"))
            }
            Ok(In::NewsBulletins) => {
                decode::Local::news_bulletins_msg(&mut fields.into_iter(), local)
                    .await
                    .map_err(|e| e.with_context("news bulletins msg"))
            }
            Ok(In::ManagedAccts) => {
                decode::Local::managed_accts_msg(&mut fields.into_iter(), local, tx, rx)
                    .await
                    .map_err(|e| e.with_context("managed accounts msg"))
            }
            Ok(In::ReceiveFa) => decode::Local::receive_fa_msg(&mut fields.into_iter(), local)
                .await
                .map_err(|e| e.with_context("receive fa msg")),
            Ok(In::HistoricalData) => {
                decode::Local::historical_data_msg(&mut fields.into_iter(), local)
                    .await
                    .map_err(|e| e.with_context("historical data msg"))
            }
            Ok(In::BondContractData) => {
                decode::Local::bond_contract_data_msg(&mut fields.into_iter(), local)
                    .await
                    .map_err(|e| e.with_context("bond contract data msg"))
            }
            Ok(In::ScannerParameters) => {
                decode::Local::scanner_parameters_msg(&mut fields.into_iter(), local)
                    .await
                    .map_err(|e| e.with_context("scanner parameters msg"))
            }
            Ok(In::ScannerData) => decode::Local::scanner_data_msg(&mut fields.into_iter(), local)
                .await
                .map_err(|e| e.with_context("scanner data msg")),
            Ok(In::TickOptionComputation) => {
                decode::Local::tick_option_computation_msg(&mut fields.into_iter(), local)
                    .await
                    .map_err(|e| e.with_context("tick option computation msg"))
            }
            Ok(In::TickGeneric) => decode::Local::tick_generic_msg(&mut fields.into_iter(), local)
                .await
                .map_err(|e| e.with_context("tick generic msg")),
            Ok(In::TickString) => decode::Local::tick_string_msg(&mut fields.into_iter(), local)
                .await
                .map_err(|e| e.with_context("tick string msg")),
            Ok(In::TickEfp) => decode::Local::tick_efp_msg(&mut fields.into_iter(), local)
                .await
                .map_err(|e| e.with_context("tick efp msg")),
            Ok(In::CurrentTime) => decode::Local::current_time_msg(&mut fields.into_iter(), local)
                .await
                .map_err(|e| e.with_context("current time msg")),
            Ok(In::RealTimeBars) => {
                decode::Local::real_time_bars_msg(&mut fields.into_iter(), local)
                    .await
                    .map_err(|e| e.with_context("real time bars msg"))
            }
            Ok(In::FundamentalData) => {
                decode::Local::fundamental_data_msg(&mut fields.into_iter(), local)
                    .await
                    .map_err(|e| e.with_context("fundamental data msg"))
            }
            Ok(In::ContractDataEnd) => {
                decode::Local::contract_data_end_msg(&mut fields.into_iter(), local)
                    .await
                    .map_err(|e| e.with_context("contract data end msg"))
            }
            Ok(In::OpenOrderEnd) => {
                decode::Local::open_order_end_msg(&mut fields.into_iter(), local)
                    .await
                    .map_err(|e| e.with_context("open order end msg"))
            }
            Ok(In::AcctDownloadEnd) => {
                decode::Local::acct_download_end_msg(&mut fields.into_iter(), local)
                    .await
                    .map_err(|e| e.with_context("acct download end msg"))
            }
            Ok(In::ExecutionDataEnd) => {
                decode::Local::execution_data_end_msg(&mut fields.into_iter(), local)
                    .await
                    .map_err(|e| e.with_context("execution data end msg"))
            }
            Ok(In::DeltaNeutralValidation) => {
                decode::Local::delta_neutral_validation_msg(&mut fields.into_iter(), local)
                    .await
                    .map_err(|e| e.with_context("delta neutral validation msg"))
            }
            Ok(In::TickSnapshotEnd) => {
                decode::Local::tick_snapshot_end_msg(&mut fields.into_iter(), local)
                    .await
                    .map_err(|e| e.with_context("tick snapshot end msg"))
            }
            Ok(In::MarketDataType) => {
                decode::Local::market_data_type_msg(&mut fields.into_iter(), local)
                    .await
                    .map_err(|e| e.with_context("market data type msg"))
            }
            Ok(In::CommissionReport) => {
                decode::Local::commission_report_msg(&mut fields.into_iter(), local)
                    .await
                    .map_err(|e| e.with_context("commission report msg"))
            }
            Ok(In::PositionData) => {
                decode::Local::position_data_msg(&mut fields.into_iter(), local)
                    .await
                    .map_err(|e| e.with_context("position data msg"))
            }
            Ok(In::PositionEnd) => decode::Local::position_end_msg(&mut fields.into_iter(), local)
                .await
                .map_err(|e| e.with_context("position end msg")),
            Ok(In::AccountSummary) => {
                decode::Local::account_summary_msg(&mut fields.into_iter(), local)
                    .await
                    .map_err(|e| e.with_context("account summary msg"))
            }
            Ok(In::AccountSummaryEnd) => {
                decode::Local::account_summary_end_msg(&mut fields.into_iter(), local)
                    .await
                    .map_err(|e| e.with_context("account summary end msg"))
            }
            Ok(In::VerifyMessageApi) => {
                decode::Local::verify_message_api_msg(&mut fields.into_iter(), local)
                    .await
                    .map_err(|e| e.with_context("verify message api msg"))
            }
            Ok(In::VerifyCompleted) => {
                decode::Local::verify_completed_msg(&mut fields.into_iter(), local)
                    .await
                    .map_err(|e| e.with_context("verify completed msg"))
            }
            Ok(In::DisplayGroupList) => {
                decode::Local::display_group_list_msg(&mut fields.into_iter(), local)
                    .await
                    .map_err(|e| e.with_context("display group list msg"))
            }
            Ok(In::DisplayGroupUpdated) => {
                decode::Local::display_group_updated_msg(&mut fields.into_iter(), local)
                    .await
                    .map_err(|e| e.with_context("display group updated msg"))
            }
            Ok(In::VerifyAndAuthMessageApi) => {
                decode::Local::verify_and_auth_message_api_msg(&mut fields.into_iter(), local)
                    .await
                    .map_err(|e| e.with_context("verify and auth message api msg"))
            }
            Ok(In::VerifyAndAuthCompleted) => {
                decode::Local::verify_and_auth_completed_msg(&mut fields.into_iter(), local)
                    .await
                    .map_err(|e| e.with_context("verify and auth completed msg"))
            }
            Ok(In::PositionMulti) => {
                decode::Local::position_multi_msg(&mut fields.into_iter(), local)
                    .await
                    .map_err(|e| e.with_context("position multi msg"))
            }
            Ok(In::PositionMultiEnd) => {
                decode::Local::position_multi_end_msg(&mut fields.into_iter(), local)
                    .await
                    .map_err(|e| e.with_context("position multi end msg"))
            }
            Ok(In::AccountUpdateMulti) => {
                decode::Local::account_update_multi_msg(&mut fields.into_iter(), local)
                    .await
                    .map_err(|e| e.with_context("account update multi msg"))
            }
            Ok(In::AccountUpdateMultiEnd) => {
                decode::Local::account_update_multi_end_msg(&mut fields.into_iter(), local)
                    .await
                    .map_err(|e| e.with_context("account update multi end msg"))
            }
            Ok(In::SecurityDefinitionOptionParameter) => {
                decode::Local::security_definition_option_parameter_msg(
                    &mut fields.into_iter(),
                    local,
                )
                .await
                .map_err(|e| e.with_context("security definition option parameter msg"))
            }
            Ok(In::SecurityDefinitionOptionParameterEnd) => {
                decode::Local::security_definition_option_parameter_end_msg(
                    &mut fields.into_iter(),
                    local,
                )
                .await
                .map_err(|e| e.with_context("security definition option parameter end msg"))
            }
            Ok(In::SoftDollarTiers) => {
                decode::Local::soft_dollar_tiers_msg(&mut fields.into_iter(), local)
                    .await
                    .map_err(|e| e.with_context("soft dollar tiers msg"))
            }
            Ok(In::FamilyCodes) => decode::Local::family_codes_msg(&mut fields.into_iter(), local)
                .await
                .map_err(|e| e.with_context("family codes msg")),
            Ok(In::SymbolSamples) => {
                decode::Local::symbol_samples_msg(&mut fields.into_iter(), local)
                    .await
                    .map_err(|e| e.with_context("symbol samples msg"))
            }
            Ok(In::MktDepthExchanges) => {
                decode::Local::mkt_depth_exchanges_msg(&mut fields.into_iter(), local)
                    .await
                    .map_err(|e| e.with_context("mkt depth exchanges msg"))
            }
            Ok(In::TickReqParams) => {
                decode::Local::tick_req_params_msg(&mut fields.into_iter(), local)
                    .await
                    .map_err(|e| e.with_context("tick req params msg"))
            }
            Ok(In::SmartComponents) => {
                decode::Local::smart_components_msg(&mut fields.into_iter(), local)
                    .await
                    .map_err(|e| e.with_context("smart components msg"))
            }
            Ok(In::NewsArticle) => decode::Local::news_article_msg(&mut fields.into_iter(), local)
                .await
                .map_err(|e| e.with_context("news article msg")),
            Ok(In::TickNews) => decode::Local::tick_news_msg(&mut fields.into_iter(), local)
                .await
                .map_err(|e| e.with_context("tick news msg")),
            Ok(In::NewsProviders) => {
                decode::Local::news_providers_msg(&mut fields.into_iter(), local)
                    .await
                    .map_err(|e| e.with_context("news providers msg"))
            }
            Ok(In::HistoricalNews) => {
                decode::Local::historical_news_msg(&mut fields.into_iter(), local)
                    .await
                    .map_err(|e| e.with_context("historical news msg"))
            }
            Ok(In::HistoricalNewsEnd) => {
                decode::Local::historical_news_end_msg(&mut fields.into_iter(), local)
                    .await
                    .map_err(|e| e.with_context("historical news end msg"))
            }
            Ok(In::HeadTimestamp) => {
                decode::Local::head_timestamp_msg(&mut fields.into_iter(), local)
                    .await
                    .map_err(|e| e.with_context("head timestamp msg"))
            }
            Ok(In::HistogramData) => {
                decode::Local::histogram_data_msg(&mut fields.into_iter(), local)
                    .await
                    .map_err(|e| e.with_context("histogram data msg"))
            }
            Ok(In::HistoricalDataUpdate) => {
                decode::Local::historical_data_update_msg(&mut fields.into_iter(), local)
                    .await
                    .map_err(|e| e.with_context("historical data update msg"))
            }
            Ok(In::RerouteMktDataReq) => {
                decode::Local::reroute_mkt_data_req_msg(&mut fields.into_iter(), local)
                    .await
                    .map_err(|e| e.with_context("reroute mkt data req msg"))
            }
            Ok(In::RerouteMktDepthReq) => {
                decode::Local::reroute_mkt_depth_req_msg(&mut fields.into_iter(), local)
                    .await
                    .map_err(|e| e.with_context("reroute mkt depth req msg"))
            }
            Ok(In::MarketRule) => decode::Local::market_rule_msg(&mut fields.into_iter(), local)
                .await
                .map_err(|e| e.with_context("market rule msg")),
            Ok(In::Pnl) => decode::Local::pnl_msg(&mut fields.into_iter(), local)
                .await
                .map_err(|e| e.with_context("pnl msg")),
            Ok(In::PnlSingle) => decode::Local::pnl_single_msg(&mut fields.into_iter(), local)
                .await
                .map_err(|e| e.with_context("pnl single msg")),
            Ok(In::HistoricalTicks) => {
                decode::Local::historical_ticks_midpoint_msg(&mut fields.into_iter(), local)
                    .await
                    .map_err(|e| e.with_context("historical ticks msg"))
            }
            Ok(In::HistoricalTicksBidAsk) => {
                decode::Local::historical_ticks_bid_ask_msg(&mut fields.into_iter(), local)
                    .await
                    .map_err(|e| e.with_context("historical ticks bid ask msg"))
            }
            Ok(In::HistoricalTicksLast) => {
                decode::Local::historical_ticks_last_msg(&mut fields.into_iter(), local)
                    .await
                    .map_err(|e| e.with_context("historical ticks last msg"))
            }
            Ok(In::TickByTick) => decode::Local::tick_by_tick_msg(&mut fields.into_iter(), local)
                .await
                .map_err(|e| e.with_context("tick by tick msg")),
            Ok(In::OrderBound) => decode::Local::order_bound_msg(&mut fields.into_iter(), local)
                .await
                .map_err(|e| e.with_context("order bound msg")),
            Ok(In::CompletedOrder) => {
                decode::Local::completed_order_msg(&mut fields.into_iter(), local)
                    .await
                    .map_err(|e| e.with_context("completed order msg"))
            }
            Ok(In::CompletedOrdersEnd) => {
                decode::Local::completed_orders_end_msg(&mut fields.into_iter(), local)
                    .await
                    .map_err(|e| e.with_context("completed orders end msg"))
            }
            Ok(In::ReplaceFaEnd) => {
                decode::Local::replace_fa_end_msg(&mut fields.into_iter(), local)
                    .await
                    .map_err(|e| e.with_context("replace fa end msg"))
            }
            Ok(In::WshMetaData) => decode::Local::wsh_meta_data_msg(&mut fields.into_iter(), local)
                .await
                .map_err(|e| e.with_context("wsh meta data msg")),
            Ok(In::WshEventData) => {
                decode::Local::wsh_event_data_msg(&mut fields.into_iter(), local)
                    .await
                    .map_err(|e| e.with_context("wsh event data msg"))
            }
            Ok(In::HistoricalSchedule) => {
                decode::Local::historical_schedule_msg(&mut fields.into_iter(), local)
                    .await
                    .map_err(|e| e.with_context("historical schedule msg"))
            }
            Ok(In::UserInfo) => decode::Local::user_info_msg(&mut fields.into_iter(), local)
                .await
                .map_err(|e| e.with_context("user info msg")),
            Err(e) => Err(DecodeError::Other(e.0).with_context("invalid in msg")),
        },
    };
    match status {
        Ok(()) => (),
        Err(e) => {
            tokio::task::yield_now().await;
            println!("\x1B[31m{e}\x1B[0m");
        }
    }
}

pub(crate) mod indicators {
    use std::collections::HashSet;

    use tokio::{net::tcp::OwnedReadHalf, sync::mpsc, task::JoinHandle};

    use crate::message::{ToClient, ToWrapper};

    use super::Reader;

    pub trait Status {}

    pub struct Inactive {
        pub(crate) reader: OwnedReadHalf,
    }

    impl Status for Inactive {}

    #[derive(Debug)]
    pub struct Active {
        pub(crate) r_thread: JoinHandle<Reader>,
        pub(crate) disconnect: super::CancelToken,
        pub(crate) tx: mpsc::Sender<ToWrapper>,
        pub(crate) rx: mpsc::Receiver<ToClient>,
        pub(crate) managed_accounts: HashSet<String>,
        pub(crate) order_id: core::ops::RangeFrom<i64>,
        pub(crate) req_id: core::ops::RangeFrom<i64>,
    }

    impl Status for Active {}
}

// =============================
// === Client Implementation ===
// =============================

#[derive(Debug)]
/// The principal client that handles all outgoing messages to the IBKR trading systems. It also
/// manages messages that are received from the "reader thread". Before any useful functionality is
/// available, an inactive client (which is created from [`Builder::connect`]) must call
/// [`Client::local`] or [`Client::remote`]. This method will return an active client that can make useful queries.
///
/// In general, [`Client`] has two types of methods: "req" methods and "get" methods.
///
/// "Req" methods require an active connection to the IBKR trading systems, and each method
/// corresponds to a single outgoing message. Note that all "req" methods are async and
/// therefore must be awaited before any useful message is sent.
///
/// "Get" methods can be called regardless of whether the client is active or inactive. These
/// methods return useful attributes of the client or other locally managed data.
pub struct Client<C: indicators::Status> {
    mode: Option<Mode>,
    host: Option<Host>,
    port: u16,
    address: std::net::Ipv4Addr,
    client_id: i64,
    server_version: u32,
    conn_time: chrono::DateTime<Tz>,
    writer: Writer,
    status: C,
}

impl<S: indicators::Status> Client<S> {
    // ====================================================
    // === Methods That Return Attributes of the Client ===
    // ====================================================

    #[inline]
    /// Return the client's mode, if it was created with [`Builder::from_config_file`].
    ///
    /// # Returns
    /// The client's [`Mode`], if it exists; otherwise, [`None`].
    pub const fn get_mode(&self) -> Option<Mode> {
        self.mode
    }

    #[inline]
    /// Return the client's host, if it was created with [`Builder::from_config_file`].
    ///
    /// # Returns
    /// The client's [`Host`], if it exists; otherwise, [`None`].
    pub const fn get_host(&self) -> Option<Host> {
        self.host
    }

    #[inline]
    /// Return the client's port
    pub const fn get_port(&self) -> u16 {
        self.port
    }

    #[inline]
    /// Return the client's address
    pub const fn get_address(&self) -> std::net::Ipv4Addr {
        self.address
    }

    #[inline]
    /// Return the client's ID, which is used by the IBKR trading systems to distinguish it from
    /// other connections.
    pub const fn get_client_id(&self) -> i64 {
        self.client_id
    }

    #[inline]
    /// Return the time at which the client successfully connected.
    pub const fn get_conn_time(&self) -> chrono::DateTime<Tz> {
        self.conn_time
    }

    #[inline]
    /// Return the version of the IBKR server with which the client is communicating.
    pub const fn get_server_version(&self) -> u32 {
        self.server_version
    }
}

#[inline]
fn spawn_reader_thread(
    rdr: OwnedReadHalf,
) -> (CancelToken, Arc<SegQueue<Vec<String>>>, JoinHandle<Reader>) {
    let disconnect = CancelToken::new();
    let queue = Arc::new(SegQueue::new());

    let r_queue = Arc::clone(&queue);
    let r_disconnect = disconnect.clone();
    let r_thread = tokio::spawn(async move {
        let reader = Reader::new(rdr, r_queue, r_disconnect);
        reader.run().await
    });
    (disconnect, queue, r_thread)
}

#[inline]
fn spawn_temp_contract_thread(
    cancel_token: CancelToken,
    queue: Arc<SegQueue<Vec<String>>>,
    mut backlog: std::collections::VecDeque<Vec<String>>,
    mut tx: mpsc::Sender<ToClient>,
    mut rx: mpsc::Receiver<ToWrapper>,
) -> JoinHandle<LoopParams> {
    tokio::spawn(async move {
        loop {
            tokio::select! {
                () = cancel_token.cancelled() => { break (queue, tx, rx, backlog); },
                () = async {
                    let _ = if let Some(fields) = queue.pop() {
                        match fields.first().and_then(|t| t.parse().ok()) {
                            Some(In::ContractData) => decode::decode_contract_no_wrapper(&mut fields.into_iter(), &mut tx, &mut rx).await.map_err(|e| e.with_context("contract data msg")),
                            Some(_) => { backlog.push_back(fields); Ok(()) },
                            None => Ok(()),
                        }
                    } else { Ok(()) };
                } => ()
            }
            tokio::task::yield_now().await;
        }
    })
}

impl Client<indicators::Inactive> {
    // ==========================================
    // === Methods That Initiate the API Loop ===
    // ==========================================

    async fn start_api(&mut self) -> Result<(), std::io::Error> {
        const VERSION: u8 = 2;

        self.writer
            .add_body((Out::StartApi, VERSION, self.client_id, None::<()>))?;
        self.writer.send().await?;
        Ok(())
    }

    async fn into_active(self) -> IntoActive {
        let (disconnect, queue, r_thread) = spawn_reader_thread(self.status.reader);

        let mut backlog = std::collections::VecDeque::new();
        let (mut managed_accounts, mut valid_id) = (None, None);
        while managed_accounts.is_none() || valid_id.is_none() {
            if let Some(fields) = queue.pop() {
                match fields.first().and_then(|t| t.parse().ok()) {
                    Some(In::ManagedAccts) => {
                        managed_accounts = Some(
                            fields
                                .into_iter()
                                .skip(2)
                                .filter(|v| v.as_str() != "")
                                .collect::<std::collections::HashSet<String>>(),
                        );
                    }
                    Some(In::NextValidId) => {
                        valid_id = decode::nth(&mut fields.into_iter(), 2, "valid_id")
                            .ok()
                            .and_then(|t| t.parse::<i64>().ok());
                    }
                    Some(_) => backlog.push_back(fields),
                    None => (),
                }
            }
            tokio::task::yield_now().await;
        }
        let (Some(managed_accounts), Some(valid_id)) = (managed_accounts, valid_id) else {
            unreachable!(
                "The loop should only exit if a valid set of accounts and id are received."
            )
        };
        let (client_tx, wrapper_rx) =
            mpsc::channel::<ToWrapper>(constants::TO_WRAPPER_CHANNEL_SIZE);
        let (wrapper_tx, client_rx) = mpsc::channel::<ToClient>(constants::TO_CLIENT_CHANNEL_SIZE);

        let client = Client {
            mode: self.mode,
            host: self.host,
            port: self.port,
            address: self.address,
            client_id: self.client_id,
            server_version: self.server_version,
            conn_time: self.conn_time,
            writer: self.writer,
            status: indicators::Active {
                r_thread,
                disconnect,
                tx: client_tx,
                rx: client_rx,
                managed_accounts,
                order_id: valid_id..,
                req_id: 0_i64..,
            },
        };
        (client, wrapper_tx, wrapper_rx, queue, backlog)
    }

    /// Initiates the main message loop and spawns all helper threads to manage the application.
    ///
    /// # Arguments
    /// * `init` - A [`LocalInitializer`], which defines how incoming data from the IBKR trading systems
    ///   should be handled.
    /// * `disconnect_token` - If provided, the client will disconnect when this token is cancelled.
    ///
    /// # Returns
    /// Does not return until the loop is terminated from within a [`LocalWrapper`] wrapper method using the [`CancelToken`]
    /// provided to the [`LocalInitializer`] in the [`LocalInitializer::build`] method or the `disconnect_token` passed
    /// to this method.
    ///
    /// # Errors
    /// Returns any error that occurs in the loop initialization or in the disconnection process.
    pub async fn local<I: LocalInitializer>(
        self,
        init: I,
        disconnect_token: Option<CancelToken>,
    ) -> Result<Builder, std::io::Error> {
        let (mut client, tx, rx, queue, backlog) = self.into_active().await;
        let temp = CancelToken::new();
        let con_fut = spawn_temp_contract_thread(temp.clone(), queue, backlog, tx, rx);

        let disconnect_token = disconnect_token.unwrap_or_default();
        let mut wrapper =
            LocalInitializer::build(init, &mut client, disconnect_token.clone()).await;
        temp.cancel();
        drop(temp);
        let (queue, mut tx, mut rx, mut backlog) = con_fut.await?;
        while let Some(fields) = backlog.pop_front() {
            decode_msg_local(fields, &mut wrapper, &mut tx, &mut rx).await;
        }
        drop(backlog);
        loop {
            tokio::select! {
                () = disconnect_token.cancelled() => {
                    println!("Client loop: disconnecting");
                    break
                },
                () = async {
                    if let Some(fields) = queue.pop() {
                        decode_msg_local(fields, &mut wrapper, &mut tx, &mut rx).await;
                    } else {
                        tokio::task::yield_now().await;
                    }
                    crate::wrapper::LocalRecurring::cycle(&mut wrapper).await;
                } => (),
            }
        }
        drop(wrapper);
        client.disconnect().await
    }

    /// Initiates the main message loop and spawns all helper threads to manage the application.
    ///
    /// # Arguments
    /// * `init` - An [`Initializer`], which defines how incoming data from the IBKR trading systems
    ///   should be handled.
    ///
    /// # Returns
    /// A [`CancelToken`] that can be used to terminate the main loop and disconnect the client.
    pub async fn remote<I: Initializer + 'static>(self, init: I) -> CancelToken {
        let (mut client, tx, rx, queue, backlog) = self.into_active().await;

        let temp = CancelToken::new();
        let con_fut = spawn_temp_contract_thread(temp.clone(), queue, backlog, tx, rx);

        let break_loop = CancelToken::new();
        let break_loop_inner = break_loop.clone();

        tokio::spawn(async move {
            let mut wrapper = Initializer::build(init, &mut client, break_loop_inner.clone()).await;
            temp.cancel();
            drop(temp);
            let (queue, mut tx, mut rx, mut backlog) = con_fut.await?;
            while let Some(fields) = backlog.pop_front() {
                decode_msg_remote(fields, &mut wrapper, &mut tx, &mut rx).await;
            }
            drop(backlog);
            loop {
                tokio::select! {
                    () = break_loop_inner.cancelled() => {
                        println!("Client loop: disconnecting");
                        break
                    },
                    () = async {
                        if let Some(fields) = queue.pop() {
                            decode_msg_remote(fields, &mut wrapper, &mut tx, &mut rx).await;
                        } else {
                            tokio::task::yield_now().await;
                        }
                        crate::wrapper::Recurring::cycle(&mut wrapper).await;
                    } => (),
                }
            }
            drop(wrapper);
            client.disconnect().await
        });

        break_loop
    }

    /// Initiates the main message loop and spawns all helper threads to manage the application.
    ///
    /// # Arguments
    /// * `wrapper` - A [`Wrapper`] that defines how incoming data from the IBKR trading systems should be handled.
    ///
    /// # Returns
    /// An active [`Client`] that can be used to make API requests.
    pub async fn disaggregated<W: Wrapper + Send + 'static>(
        self,
        mut wrapper: W,
    ) -> Client<indicators::Active> {
        let (client, mut tx, mut rx, queue, mut backlog) = self.into_active().await;
        let c_loop_disconnect = client.status.disconnect.clone();

        while let Some(fields) = backlog.pop_front() {
            decode_msg_remote(fields, &mut wrapper, &mut tx, &mut rx).await;
        }
        drop(backlog);
        tokio::spawn(async move {
            let (mut tx, mut rx, mut wrapper) = (tx, rx, wrapper);
            loop {
                tokio::select! {
                    () = c_loop_disconnect.cancelled() => {println!("Client loop: disconnecting"); break},
                    () = async {
                        if let Some(fields) = queue.pop() {
                            decode_msg_remote(fields, &mut wrapper, &mut tx, &mut rx).await;
                        } else {
                            tokio::task::yield_now().await;
                        }
                    } => (),
                }
            }
        });

        client
    }
}

type ReqResult = Result<(), std::io::Error>;
type IdResult = Result<i64, std::io::Error>;

impl Client<indicators::Active> {
    // ====================================================
    // === Methods That Return Attributes of the Client ===
    // ====================================================

    // Don't worry about the "allow": This function will NEVER panic
    #[inline]
    #[allow(clippy::missing_panics_doc, clippy::unwrap_used)]
    /// Get the next valid *order* ID, as determined by the client's internal counter
    ///
    /// # Returns
    /// The next valid order ID
    fn get_next_order_id(&mut self) -> i64 {
        self.status.order_id.next().unwrap()
    }

    // Don't worry about the "allow": This function will NEVER panic
    #[inline]
    #[allow(clippy::missing_panics_doc, clippy::unwrap_used)]
    /// Get the next valid *request* ID, as determined by the client's internal counter
    ///
    /// # Returns
    /// The next valid request ID
    fn get_next_req_id(&mut self) -> i64 {
        self.status.req_id.next().unwrap()
    }

    #[inline]
    #[must_use]
    /// Get the set of accounts managed by the client
    ///
    /// # Returns
    /// A reference to the set of the client's managed accounts
    pub const fn get_managed_accounts(&self) -> &std::collections::HashSet<String> {
        &self.status.managed_accounts
    }

    // ===================================
    // === Methods That Make API Calls ===
    // ===================================

    // === General Functions ===

    /// Request the current time from the server.
    ///
    /// # Errors
    /// Returns any error encountered while writing the outgoing message.
    pub async fn req_current_time(&mut self) -> ReqResult {
        const VERSION: u8 = 1;

        self.writer.add_body((Out::ReqCurrentTime, VERSION))?;
        self.writer.send().await
    }

    /// Requests the accounts to which the logged user has access to.
    ///
    /// # Errors
    /// Returns any error encountered while writing the outgoing message.
    pub async fn req_managed_accounts(&mut self) -> ReqResult {
        const VERSION: u8 = 1;

        self.writer.add_body((Out::ReqManagedAccts, VERSION))?;
        self.writer.send().await
    }

    /// Creates a subscription to the TWS through which account and portfolio information is
    /// delivered. This information is the exact same as the one displayed within the TWS' Account
    /// Window.
    ///
    /// # Arguments
    /// * `account_number` - The account number for which to subscribe to account data (optional for
    ///   single account structures)
    ///
    /// # Errors
    /// Returns any error encountered while writing the outgoing message. Additionally, returns an
    /// error if a provided `account_number` is not in the client's managed accounts.
    pub async fn req_account_updates(&mut self, account_number: Option<String>) -> ReqResult {
        const VERSION: u8 = 2;
        if let Some(acct_num) = &account_number {
            check_valid_account(self, acct_num)?;
        }

        self.writer
            .add_body((Out::ReqAcctData, VERSION, 1, account_number))?;
        self.writer.send().await
    }

    /// Cancels an existing subscription to receive account updates.
    ///
    /// # Arguments
    /// * `account_number` - The account number for which to subscribe to account data (optional for
    ///   single account structures)
    ///
    /// # Errors
    /// Returns any error encountered while writing the outgoing message. Additionally, returns an
    /// error if a provided `account_number` is not in the client's managed accounts.
    pub async fn cancel_account_updates(&mut self, account_number: Option<String>) -> ReqResult {
        const VERSION: u8 = 2;
        if let Some(acct_num) = &account_number {
            check_valid_account(self, acct_num)?;
        }

        self.writer
            .add_body((Out::ReqAcctData, VERSION, 0, account_number))?;
        self.writer.send().await
    }

    /// Subscribes to position updates for all accessible accounts. All positions sent initially,
    /// and then only updates as positions change.
    ///
    /// # Errors
    /// Returns any error encountered while writing the outgoing message.
    pub async fn req_positions(&mut self) -> ReqResult {
        const VERSION: u8 = 1;

        self.writer.add_body((Out::ReqPositions, VERSION))?;
        self.writer.send().await
    }

    /// Cancels a previous position subscription request made with [`Client::req_positions`].
    ///
    /// # Errors
    /// Returns any error encountered while writing the outgoing message.
    pub async fn cancel_positions(&mut self) -> ReqResult {
        const VERSION: u8 = 1;

        self.writer.add_body((Out::CancelPositions, VERSION))?;
        self.writer.send().await
    }

    /// Creates subscription for real time daily P&L and unrealized P&L updates.
    ///
    /// # Arguments
    /// * `account_number` - The account number with which to create the subscription.
    ///
    /// # Errors
    /// Returns any error encountered while writing the outgoing message. Additionally, returns an
    /// error if a provided `account_number` is not in the client's managed accounts.
    ///
    /// # Returns
    /// The unique ID associated with the request.
    pub async fn req_pnl(&mut self, account_number: &String) -> IdResult {
        let req_id = self.get_next_req_id();
        check_valid_account(self, account_number)?;

        self.writer
            .add_body((Out::ReqPnl, req_id, account_number, None::<()>))?;
        self.writer.send().await?;
        Ok(req_id)
    }

    /// Cancel subscription for real-time updates created by [`Client::req_pnl`]
    ///
    /// # Arguments
    /// * `req_id` - The ID of the [`Client::req_pnl`] subscription to cancel.
    ///
    /// # Errors
    /// Returns any error encountered while writing the outgoing message.
    pub async fn cancel_pnl(&mut self, req_id: i64) -> ReqResult {
        self.writer.add_body((Out::CancelPnl, req_id))?;
        self.writer.send().await
    }

    /// Creates subscription for real time daily P&L and unrealized P&L updates, but only for a
    /// specific position.
    ///
    /// # Arguments
    /// * `account_number` - The account number with which to create the subscription.
    /// * `contract_id` - The contract ID to create a subscription to changes for a specific
    ///   security
    ///
    /// # Errors
    /// Returns any error encountered while writing the outgoing message. Additionally, returns an
    /// error if a provided `account_number` is not in the client's managed accounts.
    ///
    /// # Returns
    /// The unique ID associated with the request.
    pub async fn req_single_position_pnl(
        &mut self,
        account_number: &String,
        contract_id: ContractId,
    ) -> IdResult {
        let req_id = self.get_next_req_id();
        check_valid_account(self, account_number)?;

        self.writer.add_body((
            Out::ReqPnlSingle,
            req_id,
            account_number,
            None::<()>,
            contract_id,
        ))?;
        self.writer.send().await?;
        Ok(req_id)
    }

    /// Cancel subscription for real-time updates created by [`Client::req_single_position_pnl`]
    ///
    /// # Arguments
    /// * `req_id` - The ID of the [`Client::req_pnl`] subscription to cancel.
    ///
    /// # Errors
    /// Returns any error encountered while writing the outgoing message.
    pub async fn cancel_pnl_single(&mut self, req_id: i64) -> ReqResult {
        self.writer.add_body((Out::CancelPnl, req_id))?;
        self.writer.send().await
    }

    /// Request completed orders.
    ///
    /// # Arguments
    /// * `api_only` - When true, only orders placed from the API returned.
    ///
    /// # Errors
    /// Returns any error encountered while writing the outgoing message.
    pub async fn req_completed_orders(&mut self, api_only: bool) -> ReqResult {
        self.writer.add_body((Out::ReqCompletedOrders, api_only))?;
        self.writer.send().await
    }

    /// Request summary information about a specific account, creating a subscription to the same
    /// information as is shown in the TWS Account Summary tab.
    ///
    /// # Arguments
    /// * `tags` - The list of data tags to include in the subscription.
    ///
    /// # Returns
    /// The unique ID associated with the request.
    ///
    /// # Errors
    /// Returns any error encountered while writing the outgoing message.
    pub async fn req_account_summary(&mut self, tags: &Vec<Tag>) -> IdResult {
        const VERSION: u8 = 1;
        let req_id = self.get_next_req_id();

        self.writer
            .add_body((Out::ReqAccountSummary, VERSION, req_id, "All", tags))?;
        self.writer.send().await?;
        Ok(req_id)
    }

    /// Cancel an existing account summary subscription created by [`Client::req_account_summary`].
    ///
    /// # Arguments
    /// * `req_id` - The ID of the subscription to cancel.
    ///
    /// # Errors
    /// Returns any error encountered while writing the outgoing message.
    pub async fn cancel_account_summary(&mut self, req_id: i64) -> ReqResult {
        const VERSION: u8 = 1;

        self.writer
            .add_body((Out::CancelAccountSummary, VERSION, req_id))?;
        self.writer.send().await
    }

    /// Request user info details for the user associated with the calling client.
    ///
    /// # Returns
    /// The unique ID associated with the request.
    ///
    /// # Errors
    /// Returns any error encountered while writing the outgoing message.
    pub async fn req_user_info(&mut self) -> IdResult {
        let req_id = self.get_next_req_id();

        self.writer.add_body((Out::ReqUserInfo, req_id))?;
        self.writer.send().await?;
        Ok(req_id)
    }

    // === Historical Market Data ===

    /// Request historical bar data for a given security. See [`historical_bar`] for
    /// types and traits that are used in this function.
    ///
    /// # Arguments
    /// * `security` - The security for which to request data.
    /// * `end_date_time` - The last datetime for which data will be returned.
    /// * `duration` - The duration for which historical data be returned (i.e. the difference
    ///   between the first bar's datetime and the last bar's datetime).
    /// * `bar_size` - The size of each individual bar.
    /// * `data` - The type of data that to return (price, volume, volatility, etc.).
    /// * `regular_trading_hours_only` - When [`true`], only return bars from regular trading hours.
    ///
    /// # Errors
    /// Returns any error encountered while writing the outgoing message.
    ///
    /// # Returns
    /// The unique ID associated with the request.
    pub async fn req_historical_bar<S, D>(
        &mut self,
        security: &S,
        end_date_time: historical_bar::EndDateTime,
        duration: historical_bar::Duration,
        bar_size: historical_bar::Size,
        data: D,
        regular_trading_hours_only: bool,
    ) -> IdResult
    where
        S: Security,
        D: historical_bar::DataType<S>,
    {
        let id = self.get_next_req_id();

        self.writer.add_body((
            Out::ReqHistoricalData,
            id,
            security.as_out_msg(),
            false,
            end_date_time,
            bar_size,
            duration,
            regular_trading_hours_only,
            data,
            1,
            false,
            None::<()>,
        ))?;
        self.writer.send().await?;
        Ok(id)
    }

    /// Request historical bar data that remains updated for a given security.
    /// See [`historical_bar`] for types and traits that are used in this function.
    ///
    /// # Arguments
    /// * `security` - The security for which to request data.
    /// * `duration` - The duration for which historical data be returned (i.e. the difference
    ///   between the first bar's datetime and the last bar's datetime).
    /// * `bar_size` - The size of each individual bar.
    /// * `data` - The type of data that to return (price, volume, volatility, etc.).
    /// * `regular_trading_hours_only` - When [`true`], only return bars from regular trading hours.
    ///
    /// # Errors
    /// Returns any error encountered while writing the outgoing message.
    ///
    /// # Returns
    /// The unique ID associated with the request.
    pub async fn req_updating_historical_bar<S, D>(
        &mut self,
        security: &S,
        duration: updating_historical_bar::Duration,
        bar_size: updating_historical_bar::Size,
        data: D,
        regular_trading_hours_only: bool,
    ) -> IdResult
    where
        S: Security,
        D: updating_historical_bar::DataType<S>,
    {
        let id = self.get_next_req_id();

        self.writer.add_body((
            Out::ReqHistoricalData,
            id,
            security.as_out_msg(),
            false,
            None::<()>,
            bar_size,
            duration,
            regular_trading_hours_only,
            data,
            1,
            true,
            None::<()>,
        ))?;
        self.writer.send().await?;
        Ok(id)
    }

    /// Cancel an existing [`historical_bar`] data request.
    ///
    /// # Arguments
    /// * `req_id` - The ID of the [`historical_bar`] request to cancel.
    ///
    /// # Errors
    /// Returns any error encountered while writing the outgoing message.
    pub async fn cancel_updating_historical_bar(&mut self, req_id: i64) -> ReqResult {
        const VERSION: u8 = 1;

        self.writer
            .add_body((Out::CancelHistoricalData, VERSION, req_id))?;
        self.writer.send().await
    }

    /// Request the earliest available data point for a given security and data type.
    ///
    /// # Arguments
    /// `security` - The security for which to make the request.
    /// `data` - The data for which to make the request.
    /// * `regular_trading_hours_only` - When [`true`], only return ticks from regular trading
    ///   hours.
    ///
    /// # Errors
    /// Returns any error encountered while writing the outgoing message.
    ///
    /// # Returns
    /// The unique ID associated with the request.
    pub async fn req_head_timestamp<S, D>(
        &mut self,
        security: &S,
        data: D,
        regular_trading_hours_only: bool,
    ) -> IdResult
    where
        S: Security,
        D: historical_ticks::DataType<S>,
    {
        let id = self.get_next_req_id();

        self.writer.add_body((
            Out::ReqHeadTimestamp,
            id,
            security.as_out_msg(),
            None::<()>,
            regular_trading_hours_only,
            data,
            2,
        ))?;
        self.writer.send().await?;
        Ok(id)
    }

    /// Cancel an existing [`Client::req_head_timestamp`] data request.
    ///
    /// # Arguments
    /// * `req_id` - The ID of the [`Client::req_head_timestamp`] request to cancel.
    ///
    /// # Errors
    /// Returns any error encountered while writing the outgoing message.
    pub async fn cancel_head_timestamp(&mut self, req_id: i64) -> ReqResult {
        self.writer.add_body((Out::CancelHeadTimestamp, req_id))?;
        self.writer.send().await
    }

    /// Request a histogram of historical data.
    ///
    /// # Arguments
    /// * `security` - The security for which to request histogram data.
    /// * `regular_trading_hours_only` - When [`true`], only return ticks from regular trading hours.
    /// * `duration` - The duration of data to return.
    ///
    /// # Errors
    /// Returns any error encountered while writing the outgoing message.
    ///
    /// # Returns
    /// The unique ID associated with the request.
    pub async fn req_histogram_data<S>(
        &mut self,
        security: &S,
        regular_trading_hours_only: bool,
        duration: histogram::Duration,
    ) -> IdResult
    where
        S: Security,
    {
        let id = self.get_next_req_id();

        self.writer.add_body((
            Out::ReqHistogramData,
            id,
            security.as_out_msg(),
            None::<()>,
            regular_trading_hours_only,
            duration,
        ))?;
        self.writer.send().await?;
        Ok(id)
    }

    /// Cancel an existing [`histogram`] data request.
    ///
    /// # Arguments
    /// * `req_id` - The ID of the [`histogram`] data request to cancel.
    ///
    /// # Errors
    /// Returns any error encountered while writing the outgoing message.
    pub async fn cancel_histogram_data(&mut self, req_id: i64) -> ReqResult {
        self.writer.add_body((Out::CancelHistogramData, req_id))?;
        self.writer.send().await
    }

    /// Request historical ticks for a given security. See [`historical_ticks`] for
    /// types and traits that are used in this function.
    ///
    /// # Arguments
    /// * `security` - The security for which to request data.
    /// * `timestamp` - The first/last datetime for which data will be returned.
    /// * `number_of_ticks` - The number of ticks to return.
    /// * `data` - The type of data to return (Trades, `BidAsk`, etc.).
    /// * `regular_trading_hours_only` - When [`true`], only return ticks from regular trading hours.
    ///
    /// # Errors
    /// Returns any error encountered while writing the outgoing message.
    ///
    /// # Returns
    /// The unique ID associated with the request.
    pub async fn req_historical_ticks<S, D>(
        &mut self,
        security: &S,
        timestamp: historical_ticks::TimeStamp,
        number_of_ticks: historical_ticks::NumberOfTicks,
        data: D,
        regular_trading_hours_only: bool,
    ) -> IdResult
    where
        S: Security,
        D: historical_ticks::DataType<S>,
    {
        let id = self.get_next_req_id();

        self.writer.add_body((
            Out::ReqHistoricalTicks,
            id,
            security.as_out_msg(),
            None::<()>,
            timestamp,
            number_of_ticks,
            data,
            regular_trading_hours_only,
            None::<()>,
            None::<()>,
        ))?;
        self.writer.send().await?;
        Ok(id)
    }

    // === Live Market Data ===

    /// Request live data for a given security.
    ///
    /// # Arguments
    /// * `security` - The security for which to request data.
    /// * `data` - The type of data to return (`RealTimeVolume`, `MarkPrice`, etc.).
    /// * `refresh_type` - How often to refresh the data (a one-time snapshot or a continuous
    ///   streaming connection)
    /// * `use_regulatory_snapshot` - When set to [`true`], return a NBBO snapshot even if no
    ///   appropriate subscription exists for streaming data. Note that doing so will cost 1 cent per
    ///   snapshot.
    ///
    /// # Errors
    /// Returns any error encountered while writing the outgoing message.
    ///
    /// # Returns
    /// The unique ID associated with the request.
    pub async fn req_market_data<S, D>(
        &mut self,
        security: &S,
        additional_data: Vec<D>,
        refresh_type: live_data::RefreshType,
        use_regulatory_snapshot: bool,
    ) -> IdResult
    where
        S: Security,
        D: live_data::DataType<S>,
    {
        const VERSION: u8 = 11;
        let id = self.get_next_req_id();

        self.writer.add_body((
            Out::ReqMktData,
            VERSION,
            id,
            security.as_out_msg(),
            false,
            additional_data,
            refresh_type,
            use_regulatory_snapshot,
            None::<()>,
        ))?;
        self.writer.send().await?;
        Ok(id)
    }

    /// Cancel an open streaming data connection with a given `req_id`.
    ///
    /// # Arguments
    /// * `req_id` - The ID associated with the market data request to cancel.
    ///
    /// # Errors
    /// Returns any error encountered while writing the outgoing message.
    pub async fn cancel_market_data(&mut self, req_id: i64) -> ReqResult {
        const VERSION: u8 = 2;

        self.writer
            .add_body((Out::CancelMktData, VERSION, req_id))?;
        self.writer.send().await
    }

    /// Set the market data variant for all succeeding `Client::req_market_data` requests.
    ///
    /// # Arguments
    /// * `variant` - The variant to set.
    ///
    /// # Errors
    /// Returns any error encountered while writing the outgoing message.
    pub async fn req_market_data_type(&mut self, variant: live_data::Class) -> ReqResult {
        const VERSION: u8 = 1;

        self.writer
            .add_body((Out::ReqMarketDataType, VERSION, variant))?;
        self.writer.send().await
    }

    /// Request real-time, 5 second bars for a given security.
    ///
    /// # Arguments
    /// * `security` - The security for which to request the bars.
    /// * `data` - The type of data to return (trades, bid, ask, midpoint).
    /// * `regular_trading_hours_only` -  When [`true`], only return ticks from regular trading
    ///   hours.
    ///
    /// # Errors
    /// Returns any error encountered while writing the outgoing message.
    ///
    /// # Returns
    /// The unique ID associated with the request.
    pub async fn req_real_time_bars<S, D>(
        &mut self,
        security: &S,
        data: D,
        regular_trading_hours_only: bool,
    ) -> IdResult
    where
        S: Security,
        D: live_bar::DataType<S>,
    {
        const VERSION: u8 = 3;
        let id = self.get_next_req_id();

        self.writer.add_body((
            Out::ReqRealTimeBars,
            VERSION,
            id,
            security.as_out_msg(),
            5_u32,
            data,
            regular_trading_hours_only,
            None::<()>,
        ))?;
        self.writer.send().await?;
        Ok(id)
    }

    /// Cancel an existing real-time bars subscription.
    ///
    /// # Arguments
    /// `req_id` - The ID associated with the bar subscription to cancel.
    ///
    /// # Errors
    /// Returns any error encountered while writing the outgoing message.
    pub async fn cancel_real_time_bars(&mut self, req_id: i64) -> ReqResult {
        const VERSION: u8 = 1;

        self.writer
            .add_body((Out::CancelRealTimeBars, VERSION, req_id))?;
        self.writer.send().await
    }

    // === Live Tick-by-Tick Data ===

    /// Request live tick-by-tick data for a given security.
    ///
    /// # Arguments
    /// * `security` - The security for which to request data.
    /// * `tick_data` - The type of data to return.
    /// * `number_of_historical_ticks` - The number of historical ticks to return before the live
    ///   data.
    /// * `ignore_size` - Ignore the size parameter in the returned ticks when set to [`true`].
    ///
    /// # Errors
    /// Returns any error encountered while writing the outgoing message.
    ///
    /// # Returns
    /// The unique ID associated with the request.
    pub async fn req_tick_by_tick_data<S, D>(
        &mut self,
        security: &S,
        tick_data: D,
        number_of_historical_ticks: live_ticks::NumberOfTicks,
        ignore_size: bool,
    ) -> IdResult
    where
        S: Security,
        D: live_ticks::DataType<S>,
    {
        let id = self.get_next_req_id();

        self.writer.add_body((
            Out::ReqTickByTickData,
            id,
            security.as_out_msg(),
            tick_data,
            number_of_historical_ticks,
            ignore_size,
        ))?;
        self.writer.send().await?;
        Ok(id)
    }

    /// Cancel an existing tick-by-tick data subscription.
    ///
    /// # Arguments
    /// * `req_id` - The request ID of the subscription to cancel.
    ///
    /// # Errors
    /// Returns any error encountered while writing the outgoing message.
    pub async fn cancel_tick_by_tick_data(&mut self, req_id: i64) -> ReqResult {
        self.writer.add_body((Out::CancelTickByTickData, req_id))?;
        self.writer.send().await
    }

    // === Market Depth ===

    /// Request market depth data for a given security.
    ///
    /// # Arguments
    /// * `security` - The security for which to return the market depth data.
    /// * `number_of_rows` - The maximum number of rows in the returned limit order book.
    ///
    /// # Errors
    /// Returns any error encountered while writing the outgoing message.
    ///
    /// # Returns
    /// The unique ID associated with the request.
    pub async fn req_market_depth<S>(&mut self, security: &S, number_of_rows: u32) -> IdResult
    where
        S: Security,
    {
        const VERSION: u8 = 5;
        let id = self.get_next_req_id();

        self.writer.add_body((
            Out::ReqMktDepth,
            VERSION,
            id,
            security.as_out_msg(),
            number_of_rows,
            true,
            None::<()>,
        ))?;
        self.writer.send().await?;
        Ok(id)
    }

    /// Request exchanges available for market depth.
    ///
    /// # Errors
    /// Returns any error encountered while writing the outgoing message.
    pub async fn req_market_depth_exchanges(&mut self) -> ReqResult {
        self.writer.add_body(Out::ReqMktDepthExchanges)?;
        self.writer.send().await
    }

    /// Cancel a market depth subscription for a given `req_id`.
    ///
    /// # Arguments
    /// * `req_id` - The request ID for which to cancel a market depth subscription.
    ///
    /// # Errors
    /// Returns any error encountered while writing the outgoing message.
    pub async fn cancel_market_depth(&mut self, req_id: i64) -> ReqResult {
        const VERSION: u8 = 1;

        self.writer
            .add_body((Out::CancelMktDepth, VERSION, req_id))?;
        self.writer.send().await
    }

    /// Request exchanges comprising the aggregate SMART exchange
    ///
    /// # Arguments
    /// * `exchange_id` - The identifier containing information about the component exchanges, which
    ///   is attained from an initial market data callback.
    ///
    /// # Errors
    /// Returns any error encountered while writing the outgoing message.
    ///
    /// # Returns
    /// The unique ID associated with the request.
    pub async fn req_smart_components(&mut self, exchange_id: ExchangeId) -> IdResult {
        let id = self.get_next_req_id();

        self.writer
            .add_body((Out::ReqSmartComponents, id, exchange_id))?;
        self.writer.send().await?;
        Ok(id)
    }

    // === Orders and order management ===

    /// Place an order.
    ///
    /// # Arguments
    /// * `security` - The security on which to place the order.
    /// * `order` - The order to execute.
    ///
    /// # Errors
    /// Returns any error encountered while writing the outgoing message.
    ///
    /// # Returns
    /// The unique ID associated with the request.
    pub async fn req_place_order<S, E>(&mut self, order: &Order<'_, S, E>) -> IdResult
    where
        S: Security,
        E: Executable<S>,
    {
        let id = self.get_next_order_id();

        self.writer.add_body((
            Out::PlaceOrder,
            id,
            order.get_security().as_out_msg(),
            None::<()>,
            None::<()>,
            order,
        ))?;
        self.writer.send().await?;
        Ok(id)
    }

    /// Modify an order.
    ///
    /// # Arguments
    /// * `security` - The security on which the original order was placed.
    /// * `order` - The original order.
    /// * `id` - The original order's ID.
    ///
    /// # Errors
    /// Returns any error encountered while writing the outgoing message.
    ///
    /// # Returns
    /// The unique ID associated with the request.
    pub async fn req_modify_order<S, E>(&mut self, order: &Order<'_, S, E>, id: i64) -> IdResult
    where
        S: Security,
        E: Executable<S>,
    {
        self.writer.add_body((
            Out::PlaceOrder,
            id,
            order.get_security().as_out_msg(),
            None::<()>,
            None::<()>,
            order,
        ))?;
        self.writer.send().await?;
        Ok(id)
    }

    /// Cancel an order.
    ///
    /// # Arguments
    /// * `id` - The ID of the order to cancel.
    ///
    /// # Errors
    /// Returns any error encountered while writing the outgoing message.
    pub async fn cancel_order(&mut self, id: i64) -> ReqResult {
        const VERSION: u8 = 1;

        self.writer
            .add_body((Out::CancelOrder, VERSION, id, None::<()>))?;
        self.writer.send().await
    }

    /// Cancel all currently open orders, including those placed in TWS.
    ///
    /// # Errors
    /// Returns any error encountered while writing the outgoing message.
    pub async fn cancel_all_orders(&mut self) -> ReqResult {
        const VERSION: u8 = 1;

        self.writer.add_body((Out::ReqGlobalCancel, VERSION))?;
        self.writer.send().await
    }

    /// Request all the open orders placed from all API clients and from TWS.
    ///
    /// Note that this will request all the orders associated with a given IBKR account and
    /// therefore will contain orders placed by another [`Client`].
    ///
    /// # Errors
    /// Returns any error encountered while writing the outgoing message.
    pub async fn req_all_open_orders(&mut self) -> ReqResult {
        const VERSION: u8 = 1;

        self.writer.add_body((Out::ReqAllOpenOrders, VERSION))?;
        self.writer.send().await
    }

    /// Request that all newly created TWS orders will be implicitly associated with the calling
    /// client. Therefore, the API will receive updates about TWS orders.
    ///
    /// Note! This can only be called from a client with ID 0.
    ///
    /// # Errors
    /// Returns any error encountered while writing the outgoing message. Also returns an error if
    /// the calling client does not have ID 0.
    pub async fn req_auto_open_orders(&mut self) -> ReqResult {
        const VERSION: u8 = 1;

        self.writer
            .add_body((Out::ReqAutoOpenOrders, VERSION, true))?;
        self.writer.send().await
    }

    /// Request the open orders that were placed from the calling client.
    ///
    /// A Note that a client with an ID of 0 will also receive updates about orders placed with TWS.
    ///
    /// # Errors
    /// Returns any error encountered while writing the outgoing message.
    pub async fn req_open_orders(&mut self) -> ReqResult {
        const VERSION: u8 = 1;

        self.writer.add_body((Out::ReqOpenOrders, VERSION))?;
        self.writer.send().await
    }

    // === Executions ===

    /// Request execution all execution reports that fit the criteria specified in the `filter`.
    ///
    /// In order to view executions beyond the past 24 hours, open the Trade Log in TWS and, while
    /// the Trade Log is displayed, request the executions again from the API.
    ///
    /// # Arguments
    /// `filter` - The conditions with which to determine whether an execution will be returned.
    ///
    /// # Errors
    /// Returns any error encountered while writing the outgoing message.
    pub async fn req_executions(&mut self, filter: Filter) -> IdResult {
        const VERSION: u8 = 3;
        let req_id = self.get_next_req_id();

        self.writer
            .add_body((Out::ReqExecutions, VERSION, req_id, filter))?;
        self.writer.send().await?;
        Ok(req_id)
    }

    // === Contract Creation ===

    #[inline]
    pub(crate) async fn send_contract_query(&mut self, query: Query) -> Result<(), std::io::Error> {
        const VERSION: u8 = 8;
        let req_id = self.get_next_req_id();
        self.status
            .tx
            .send(ToWrapper::ContractQuery((query, req_id)))
            .await
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::BrokenPipe, e))?;

        match query {
            Query::IbContractId(contract_id, routing) => {
                self.writer.add_body((
                    Out::ReqContractData,
                    VERSION,
                    req_id,
                    contract_id,
                    [None::<()>; 6],
                    routing,
                    [None::<()>; 8],
                ))?;
            }
            Query::Figi(figi) => {
                self.writer.add_body((
                    Out::ReqContractData,
                    VERSION,
                    req_id,
                    [None::<()>; 7],
                    Routing::Smart,
                    [None::<()>; 5],
                    "FIGI",
                    figi,
                    None::<()>,
                ))?;
            }
        }

        self.writer.send().await?;
        Ok(())
    }

    #[inline]
    pub(crate) async fn recv_contract_query(&mut self) -> Option<crate::contract::Contract> {
        if let Some(ToClient::NewContract(c)) = self.status.rx.recv().await {
            Some(c)
        } else {
            None
        }
    }

    // === Disconnect ==

    #[inline]
    /// Terminate the connection with the IBKR trading systems and return a [`Builder`] that can
    /// be used to reconnect if necessary.
    ///
    /// # Errors
    /// Returns any error encountered while flushing and shutting down the outgoing buffer.
    ///
    /// # Returns
    /// A [`Builder`] with the same port and address as the existing client.
    pub async fn disconnect(mut self) -> Result<Builder, std::io::Error> {
        self.writer.flush().await?;
        self.writer.shutdown().await?;
        self.status.disconnect.cancel();
        self.status.r_thread.await?;
        Ok(Builder(Inner::Manual {
            port: self.port,
            address: self.address,
        }))
    }
}

#[inline]
fn check_valid_account(
    client: &Client<indicators::Active>,
    account_number: &str,
) -> Result<(), std::io::Error> {
    if client.status.managed_accounts.contains(account_number) {
        Ok(())
    } else {
        Err(std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            "Invalid account number provided to req_account_updates",
        ))
    }
}
