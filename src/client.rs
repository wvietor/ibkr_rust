use anyhow::Context;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{tcp::OwnedWriteHalf, TcpStream},
    sync::mpsc,
};

use crate::{
    constants,
    contract::{ContractId, Security},
    decode, make_msg,
    market_data::{histogram, historical_bar, historical_ticks, live_bar, live_data, live_ticks},
    message::{InMsg, OutMsg, ToClient, ToWrapper},
    order::Executable,
    reader::Reader,
    wrapper::Wrapper,
};
use crate::{order::Order, payload::ExchangeId};

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

impl Config {
    #[inline]
    fn new(path: &str) -> anyhow::Result<Self> {
        toml::from_str(
            std::fs::read_to_string(path)
                .with_context(|| format!("Invalid config file at path {path}"))?
                .as_str(),
        )
        .with_context(|| {
            format!(
                "Invalid TOML file at path {path}.\n
        # =========================\n
        # === config.toml Usage ===\n
        # =========================\n
        address: std::net::Ipv4Addr\n
        \n
        [Ports]\n
        tws_live: u16\n
        tws_paper: u16\n
        \n
        gateway_live: u16\n
        gateway_paper: u16\n"
            )
        })
    }
}

// =======================================
// === Client Builder and Helper Types ===
// =======================================

//noinspection SpellCheckingInspection
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
/// Represents the two platforms that facilitate trading with IBKR's systems. The two hosts are
/// indistinguishable from the perspective of an API application.
pub enum Host {
    /// IBKR's flagship Trader Workstation desktop application.
    Tws,
    /// A leaner GUI that requires less performance overhead but has no monitoring of sophisticated
    /// graphical capabilities.
    Gateway,
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
    /// money) trading environment.
    /// * `host` - Specifies the platform used for communication with IBKR's trading systems.
    /// * `path` - An optional string slice that overrides the default location of "./config.toml".
    ///
    /// # Errors
    /// Returns any error encountered while reading and parsing the config file.
    pub fn from_config_file(mode: Mode, host: Host, path: Option<&str>) -> anyhow::Result<Self> {
        let config = Config::new(path.unwrap_or("./config.toml"))?;
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
    /// * `wrapper` - A type whose methods will be called upon receiving data from IBKR's trading
    /// systems.
    ///
    /// # Errors
    /// This function will error if any of the following occurs:
    /// 1) An error occurs while initiating a TCP connection on the port and address specified in
    /// either [`Builder::manual`] or in the "config.toml" file specified in
    /// [`Builder::from_config_file`].
    /// 2) An error occurs while reading or writing the handshake message that initiates a
    /// connection with IBKR's trading systems.
    ///
    /// # Returns
    /// An inactive [`Client`] that will become active upon calling [`Client::run`]
    pub async fn connect<W: 'static + Wrapper>(
        &self,
        client_id: i64,
        wrapper: W,
    ) -> anyhow::Result<Client<indicators::Inactive<W>>> {
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

        let (mut reader, mut writer) = TcpStream::connect((address, port)).await?.into_split();

        let msg = make_msg!(
            "API\0";
            format!("v{}..{}", constants::MIN_CLIENT_VERSION, constants::MAX_CLIENT_VERSION)
        );
        writer.write_all(msg.as_bytes()).await?;

        let mut buf = bytes::BytesMut::with_capacity(usize::try_from(reader.read_u32().await?)?);
        reader.read_buf(&mut buf).await?;
        let resp = buf.into_iter().map(char::from).collect::<String>();
        let mut params = resp.split('\0');

        let server_version = params
            .next()
            .ok_or_else(|| anyhow::Error::msg("Missing server version in IBKR handshake response"))?
            .parse()
            .with_context(|| "Failed to parse server version")?;
        let conn_time = chrono::NaiveDateTime::parse_and_remainder(
            params
                .next()
                .ok_or_else(|| {
                    anyhow::Error::msg("Missing connection time in IBKR handshake response")
                })?
                .trim_end_matches(|c: char| !c.is_numeric()),
            "%Y%m%d %X",
        )
        .with_context(|| "Failed to parse connection time")?
        .0;

        let (client_tx, wrapper_rx) =
            mpsc::channel::<ToWrapper>(constants::TO_WRAPPER_CHANNEL_SIZE);
        let (wrapper_tx, client_rx) = mpsc::channel::<ToClient>(constants::TO_CLIENT_CHANNEL_SIZE);

        let mut client = Client {
            mode,
            host,
            port,
            address,
            client_id,
            server_version,
            conn_time,
            writer,
            status: indicators::Inactive {
                reader,
                wrapper,
                client_tx,
                client_rx,
                wrapper_tx,
                wrapper_rx,
            },
        };
        client.start_api().await?;

        Ok(client)
    }
}

// ===============================
// === Status Trait Definition ===
// ===============================

pub(crate) mod indicators {
    use std::collections::HashSet;

    use tokio::{net::tcp::OwnedReadHalf, sync::mpsc, task::JoinHandle};

    use crate::{
        message::{ToClient, ToWrapper},
        wrapper::Wrapper,
    };

    use super::Reader;

    pub trait Status {}

    #[derive(Debug)]
    pub struct Inactive<W: Wrapper> {
        pub(crate) reader: OwnedReadHalf,
        pub(crate) wrapper: W,
        pub(crate) client_tx: mpsc::Sender<ToWrapper>,
        pub(crate) client_rx: mpsc::Receiver<ToClient>,
        pub(crate) wrapper_tx: mpsc::Sender<ToClient>,
        pub(crate) wrapper_rx: mpsc::Receiver<ToWrapper>,
    }

    impl<W: Wrapper> Status for Inactive<W> {}

    #[derive(Debug)]
    pub struct Active {
        pub(crate) r_thread: JoinHandle<Reader>,
        pub(crate) disconnect: tokio_util::sync::CancellationToken,
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
/// [`Client::run`]. This method will return an active client that can make useful queries.
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
    conn_time: chrono::NaiveDateTime,
    writer: OwnedWriteHalf,
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
    pub const fn get_conn_time(&self) -> chrono::NaiveDateTime {
        self.conn_time
    }

    #[inline]
    /// Return the version of the IBKR server with which the client is communicating.
    pub const fn get_server_version(&self) -> u32 {
        self.server_version
    }
}

impl<W: 'static + Wrapper> Client<indicators::Inactive<W>> {
    // ==========================================
    // === Methods That Initiate the API Loop ===
    // ==========================================

    async fn start_api(&mut self) -> Result<(), anyhow::Error> {
        const VERSION: u8 = 2;
        let msg = make_msg!(OutMsg::StartApi, VERSION, self.client_id, "");
        self.status
            .client_tx
            .send(ToWrapper::StartApiManagedAccts)
            .await?;
        self.status
            .client_tx
            .send(ToWrapper::StartApiNextValidId)
            .await?;
        self.writer.write_all(msg.as_bytes()).await?;
        Ok(())
    }

    // Don't worry about these allows: This function will NEVER panic (it can infinite loop though)
    #[allow(
        clippy::unwrap_used,
        clippy::missing_panics_doc,
        clippy::too_many_lines
    )]
    /// Initiates the main message loop and spawns all helper threads to manage the application.
    ///
    /// # Returns
    /// An active client that can be used to make useful queries, process market data, place
    /// orders, etc.
    pub async fn run(mut self) -> Client<indicators::Active> {
        let disconnect = tokio_util::sync::CancellationToken::new();
        let queue = Arc::new(crossbeam::queue::SegQueue::new());

        // The reader thread
        let r_queue = Arc::clone(&queue);
        let r_disconnect = disconnect.clone();
        let r_thread = tokio::spawn(async move {
            let reader = Reader::new(self.status.reader, r_queue, r_disconnect);
            reader.run().await
        });

        // The main client loop
        let c_loop_disconnect = disconnect.clone();
        tokio::spawn(async move {
            let (mut wrapper, mut tx, mut rx) = (
                self.status.wrapper,
                self.status.wrapper_tx,
                self.status.wrapper_rx,
            );
            loop {
                tokio::select! {
                    _ = c_loop_disconnect.cancelled() => {println!("Client loop: disconnecting"); break},
                    _ = async {
                        if let Some(fields) = queue.pop() {
                            let status = match fields.get(0) {
                                None => Err(anyhow::Error::msg("Empty fields received from reader")),
                                Some(s) => {
                                    match s.parse() {
                                        Ok(InMsg::TickPrice) => decode::tick_price_msg(&mut fields.into_iter(), &mut wrapper).with_context(|| "tick price msg"),
                                        Ok(InMsg::TickSize) => decode::tick_size_msg(&mut fields.into_iter(), &mut wrapper).with_context(|| "tick size msg"),
                                        Ok(InMsg::OrderStatus) => decode::order_status_msg(&mut fields.into_iter(), &mut wrapper).with_context(|| "order status msg"),
                                        Ok(InMsg::ErrMsg) => decode::err_msg_msg(&mut fields.into_iter(), &mut wrapper).with_context(|| "err msg msg"),
                                        Ok(InMsg::OpenOrder) => decode::open_order_msg(&mut fields.into_iter(), &mut wrapper).with_context(|| "open order msg"),
                                        Ok(InMsg::AcctValue) => decode::acct_value_msg(&mut fields.into_iter(), &mut wrapper).with_context(|| "acct value msg"),
                                        Ok(InMsg::PortfolioValue) => decode::portfolio_value_msg(&mut fields.into_iter(), &mut wrapper).with_context(|| "portfolio value msg"),
                                        Ok(InMsg::AcctUpdateTime) => decode::acct_update_time_msg(&mut fields.into_iter(), &mut wrapper).with_context(|| "acct update time msg"),
                                        Ok(InMsg::NextValidId) => decode::next_valid_id_msg(&mut fields.into_iter(), &mut wrapper, &mut tx, &mut rx).await.with_context(|| "next valid id msg"),
                                        Ok(InMsg::ContractData) => decode::contract_data_msg(&mut fields.into_iter(), &mut wrapper, &mut tx, &mut rx).await.with_context(|| "contract data msg"),
                                        Ok(InMsg::ExecutionData) => decode::execution_data_msg(&mut fields.into_iter(), &mut wrapper).with_context(|| "execution data msg"),
                                        Ok(InMsg::MarketDepth) => decode::market_depth_msg(&mut fields.into_iter(), &mut wrapper).with_context(|| "market depth msg"),
                                        Ok(InMsg::MarketDepthL2) => decode::market_depth_l2_msg(&mut fields.into_iter(), &mut wrapper).with_context(|| "market depth l2 msg"),
                                        Ok(InMsg::NewsBulletins) => decode::news_bulletins_msg(&mut fields.into_iter(), &mut wrapper).with_context(|| "news bulletins msg"),
                                        Ok(InMsg::ManagedAccts) => decode::managed_accts_msg(&mut fields.into_iter(), &mut wrapper, &mut tx, &mut rx).await.with_context(|| "managed accts msg"),
                                        Ok(InMsg::ReceiveFa) => decode::receive_fa_msg(&mut fields.into_iter(), &mut wrapper).with_context(|| "receive fa msg"),
                                        Ok(InMsg::HistoricalData) => decode::historical_data_msg(&mut fields.into_iter(), &mut wrapper).with_context(|| "historical data msg"),
                                        Ok(InMsg::BondContractData) => decode::bond_contract_data_msg(&mut fields.into_iter(), &mut wrapper).with_context(|| "bond contract data msg"),
                                        Ok(InMsg::ScannerParameters) => decode::scanner_parameters_msg(&mut fields.into_iter(), &mut wrapper).with_context(|| "scanner parameters msg"),
                                        Ok(InMsg::ScannerData) => decode::scanner_data_msg(&mut fields.into_iter(), &mut wrapper).with_context(|| "scanner data msg"),
                                        Ok(InMsg::TickOptionComputation) => decode::tick_option_computation_msg(&mut fields.into_iter(), &mut wrapper).with_context(|| "tick option computation msg"),
                                        Ok(InMsg::TickGeneric) => decode::tick_generic_msg(&mut fields.into_iter(), &mut wrapper).with_context(|| "tick generic msg"),
                                        Ok(InMsg::TickString) => decode::tick_string_msg(&mut fields.into_iter(), &mut wrapper).with_context(|| "tick string msg"),
                                        Ok(InMsg::TickEfp) => decode::tick_efp_msg(&mut fields.into_iter(), &mut wrapper).with_context(|| "tick efp msg"),
                                        Ok(InMsg::CurrentTime) => decode::current_time_msg(&mut fields.into_iter(), &mut wrapper).with_context(|| "current time msg"),
                                        Ok(InMsg::RealTimeBars) => decode::real_time_bars_msg(&mut fields.into_iter(), &mut wrapper).with_context(|| "real time bars msg"),
                                        Ok(InMsg::FundamentalData) => decode::fundamental_data_msg(&mut fields.into_iter(), &mut wrapper).with_context(|| "fundamental data msg"),
                                        Ok(InMsg::ContractDataEnd) => decode::contract_data_end_msg(&mut fields.into_iter(), &mut wrapper).with_context(|| "contract data end msg"),
                                        Ok(InMsg::OpenOrderEnd) => decode::open_order_end_msg(&mut fields.into_iter(), &mut wrapper).with_context(|| "open order end msg"),
                                        Ok(InMsg::AcctDownloadEnd) => decode::acct_download_end_msg(&mut fields.into_iter(), &mut wrapper).with_context(|| "acct download end msg"),
                                        Ok(InMsg::ExecutionDataEnd) => decode::execution_data_end_msg(&mut fields.into_iter(), &mut wrapper).with_context(|| "execution data end msg"),
                                        Ok(InMsg::DeltaNeutralValidation) => decode::delta_neutral_validation_msg(&mut fields.into_iter(), &mut wrapper).with_context(|| "delta neutral validation msg"),
                                        Ok(InMsg::TickSnapshotEnd) => decode::tick_snapshot_end_msg(&mut fields.into_iter(), &mut wrapper).with_context(|| "tick snapshot end msg"),
                                        Ok(InMsg::MarketDataType) => decode::market_data_type_msg(&mut fields.into_iter(), &mut wrapper).with_context(|| "market data type msg"),
                                        Ok(InMsg::CommissionReport) => decode::commission_report_msg(&mut fields.into_iter(), &mut wrapper).with_context(|| "commission report msg"),
                                        Ok(InMsg::PositionData) => decode::position_data_msg(&mut fields.into_iter(), &mut wrapper).with_context(|| "position data msg"),
                                        Ok(InMsg::PositionEnd) => decode::position_end_msg(&mut fields.into_iter(), &mut wrapper).with_context(|| "position end msg"),
                                        Ok(InMsg::AccountSummary) => decode::account_summary_msg(&mut fields.into_iter(), &mut wrapper).with_context(|| "account summary msg"),
                                        Ok(InMsg::AccountSummaryEnd) => decode::account_summary_end_msg(&mut fields.into_iter(), &mut wrapper).with_context(|| "account summary end msg"),
                                        Ok(InMsg::VerifyMessageApi) => decode::verify_message_api_msg(&mut fields.into_iter(), &mut wrapper).with_context(|| "verify message api msg"),
                                        Ok(InMsg::VerifyCompleted) => decode::verify_completed_msg(&mut fields.into_iter(), &mut wrapper).with_context(|| "verify completed msg"),
                                        Ok(InMsg::DisplayGroupList) => decode::display_group_list_msg(&mut fields.into_iter(), &mut wrapper).with_context(|| "display group list msg"),
                                        Ok(InMsg::DisplayGroupUpdated) => decode::display_group_updated_msg(&mut fields.into_iter(), &mut wrapper).with_context(|| "display group updated msg"),
                                        Ok(InMsg::VerifyAndAuthMessageApi) => decode::verify_and_auth_message_api_msg(&mut fields.into_iter(), &mut wrapper).with_context(|| "verify and auth message api msg"),
                                        Ok(InMsg::VerifyAndAuthCompleted) => decode::verify_and_auth_completed_msg(&mut fields.into_iter(), &mut wrapper).with_context(|| "verify and auth completed msg"),
                                        Ok(InMsg::PositionMulti) => decode::position_multi_msg(&mut fields.into_iter(), &mut wrapper).with_context(|| "position multi msg"),
                                        Ok(InMsg::PositionMultiEnd) => decode::position_multi_end_msg(&mut fields.into_iter(), &mut wrapper).with_context(|| "position multi end msg"),
                                        Ok(InMsg::AccountUpdateMulti) => decode::account_update_multi_msg(&mut fields.into_iter(), &mut wrapper).with_context(|| "account update multi msg"),
                                        Ok(InMsg::AccountUpdateMultiEnd) => decode::account_update_multi_end_msg(&mut fields.into_iter(), &mut wrapper).with_context(|| "account update multi end msg"),
                                        Ok(InMsg::SecurityDefinitionOptionParameter) => decode::security_definition_option_parameter_msg(&mut fields.into_iter(), &mut wrapper).with_context(|| "security definition option parameter msg"),
                                        Ok(InMsg::SecurityDefinitionOptionParameterEnd) => decode::security_definition_option_parameter_end_msg(&mut fields.into_iter(), &mut wrapper).with_context(|| "security definition option parameter end msg"),
                                        Ok(InMsg::SoftDollarTiers) => decode::soft_dollar_tiers_msg(&mut fields.into_iter(), &mut wrapper).with_context(|| "soft dollar tiers msg"),
                                        Ok(InMsg::FamilyCodes) => decode::family_codes_msg(&mut fields.into_iter(), &mut wrapper).with_context(|| "family codes msg"),
                                        Ok(InMsg::SymbolSamples) => decode::symbol_samples_msg(&mut fields.into_iter(), &mut wrapper).with_context(|| "symbol samples msg"),
                                        Ok(InMsg::MktDepthExchanges) => decode::mkt_depth_exchanges_msg(&mut fields.into_iter(), &mut wrapper).with_context(|| "mkt depth exchanges msg"),
                                        Ok(InMsg::TickReqParams) => decode::tick_req_params_msg(&mut fields.into_iter(), &mut wrapper).with_context(|| "tick req params msg"),
                                        Ok(InMsg::SmartComponents) => decode::smart_components_msg(&mut fields.into_iter(), &mut wrapper).with_context(|| "smart components msg"),
                                        Ok(InMsg::NewsArticle) => decode::news_article_msg(&mut fields.into_iter(), &mut wrapper).with_context(|| "news article msg"),
                                        Ok(InMsg::TickNews) => decode::tick_news_msg(&mut fields.into_iter(), &mut wrapper).with_context(|| "tick news msg"),
                                        Ok(InMsg::NewsProviders) => decode::news_providers_msg(&mut fields.into_iter(), &mut wrapper).with_context(|| "news providers msg"),
                                        Ok(InMsg::HistoricalNews) => decode::historical_news_msg(&mut fields.into_iter(), &mut wrapper).with_context(|| "historical news msg"),
                                        Ok(InMsg::HistoricalNewsEnd) => decode::historical_news_end_msg(&mut fields.into_iter(), &mut wrapper).with_context(|| "historical news end msg"),
                                        Ok(InMsg::HeadTimestamp) => decode::head_timestamp_msg(&mut fields.into_iter(), &mut wrapper).with_context(|| "head timestamp msg"),
                                        Ok(InMsg::HistogramData) => decode::histogram_data_msg(&mut fields.into_iter(), &mut wrapper).with_context(|| "histogram data msg"),
                                        Ok(InMsg::HistoricalDataUpdate) => decode::historical_data_update_msg(&mut fields.into_iter(), &mut wrapper).with_context(|| "historical data update msg"),
                                        Ok(InMsg::RerouteMktDataReq) => decode::reroute_mkt_data_req_msg(&mut fields.into_iter(), &mut wrapper).with_context(|| "reroute mkt data req msg"),
                                        Ok(InMsg::RerouteMktDepthReq) => decode::reroute_mkt_depth_req_msg(&mut fields.into_iter(), &mut wrapper).with_context(|| "reroute mkt depth req msg"),
                                        Ok(InMsg::MarketRule) => decode::market_rule_msg(&mut fields.into_iter(), &mut wrapper).with_context(|| "market rule msg"),
                                        Ok(InMsg::Pnl) => decode::pnl_msg(&mut fields.into_iter(), &mut wrapper).with_context(|| "pnl msg"),
                                        Ok(InMsg::PnlSingle) => decode::pnl_single_msg(&mut fields.into_iter(), &mut wrapper).with_context(|| "pnl single msg"),
                                        Ok(InMsg::HistoricalTicks) => decode::historical_ticks_midpoint_msg(&mut fields.into_iter(), &mut wrapper).with_context(|| "historical ticks msg"),
                                        Ok(InMsg::HistoricalTicksBidAsk) => decode::historical_ticks_bid_ask_msg(&mut fields.into_iter(), &mut wrapper).with_context(|| "historical ticks bid ask msg"),
                                        Ok(InMsg::HistoricalTicksLast) => decode::historical_ticks_last_msg(&mut fields.into_iter(), &mut wrapper).with_context(|| "historical ticks last msg"),
                                        Ok(InMsg::TickByTick) => decode::tick_by_tick_msg(&mut fields.into_iter(), &mut wrapper).with_context(|| "tick by tick msg"),
                                        Ok(InMsg::OrderBound) => decode::order_bound_msg(&mut fields.into_iter(), &mut wrapper).with_context(|| "order bound msg"),
                                        Ok(InMsg::CompletedOrder) => decode::completed_order_msg(&mut fields.into_iter(), &mut wrapper).with_context(|| "completed order msg"),
                                        Ok(InMsg::CompletedOrdersEnd) => decode::completed_orders_end_msg(&mut fields.into_iter(), &mut wrapper).with_context(|| "completed orders end msg"),
                                        Ok(InMsg::ReplaceFaEnd) => decode::replace_fa_end_msg(&mut fields.into_iter(), &mut wrapper).with_context(|| "replace fa end msg"),
                                        Ok(InMsg::WshMetaData) => decode::wsh_meta_data_msg(&mut fields.into_iter(), &mut wrapper).with_context(|| "wsh meta data msg"),
                                        Ok(InMsg::WshEventData) => decode::wsh_event_data_msg(&mut fields.into_iter(), &mut wrapper).with_context(|| "wsh event data msg"),
                                        Ok(InMsg::HistoricalSchedule) => decode::historical_schedule_msg(&mut fields.into_iter(), &mut wrapper).with_context(|| "historical schedule msg"),
                                        Ok(InMsg::UserInfo) => decode::user_info_msg(&mut fields.into_iter(), &mut wrapper).with_context(|| "user info msg"),
                                        Err(e) => Err(e.into()),
                                    }
                                },
                            };
                            match status {
                                Ok(()) => (),
                                Err(e) => println!("{e}")
                            }
                        }
                    } => (),
                }
            }
        });
        // Get start_api callback messages
        let (mut managed_accounts, mut valid_id) = (None, None);
        while managed_accounts.is_none() || valid_id.is_none() {
            match self.status.client_rx.recv().await {
                Some(ToClient::StartApiManagedAccts(accts)) => managed_accounts = Some(accts),
                Some(ToClient::StartApiNextValidId(id)) => valid_id = Some(id..),
                _ => (),
            }
        }

        let (managed_accounts, valid_id) = (managed_accounts.unwrap(), valid_id.unwrap());

        Client {
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
                tx: self.status.client_tx,
                rx: self.status.client_rx,
                managed_accounts,
                order_id: valid_id,
                req_id: 0_i64..,
            },
        }
    }
}

type ReqResult = Result<(), std::io::Error>;
type IdResult = Result<i64, std::io::Error>;

impl Client<indicators::Active> {
    // ====================================================
    // === Methods That Return Attributes of the Client ===
    // ====================================================

    // Don't worry about the allow: This function will NEVER panic
    #[inline]
    #[allow(clippy::missing_panics_doc, clippy::unwrap_used)]
    /// Get the next valid *order* ID, as determined by the client's internal counter
    ///
    /// # Returns
    /// The next valid order ID
    fn get_next_order_id(&mut self) -> i64 {
        self.status.order_id.next().unwrap()
    }

    // Don't worry about the allow: This function will NEVER panic
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

    /// Request the current time from the server.
    ///
    /// # Errors
    /// Returns any error encountered while writing the outgoing message
    pub async fn req_current_time(&mut self) -> ReqResult {
        const VERSION: u8 = 1;
        let msg = make_msg!(OutMsg::ReqCurrentTime, VERSION);
        self.writer.write_all(msg.as_bytes()).await
    }

    // === Historical Market Data ===

    /// Request historical bar data for a given security. See [`historical_bar`] for
    /// types and traits that are used in this funciton.
    ///
    /// # Arguments
    /// * `security` - The security for which to request data.
    /// * `end_date_time` - The last datetime for which data will be returned.
    /// * `duration` - The duration for which historical data be returned (ie. the difference
    /// between the first bar's datetime and the last bar's datetime).
    /// * `bar_size` - The size of each individual bar.
    /// * `data` - The type of data that to return (price, volume, volatility, etc.).
    /// * `regular_trading_hours_only` - When [`true`], only return bars from regular trading hours.
    ///
    /// # Errors
    /// Returns any error encountered while writing the outgoing message.
    ///
    /// # Returns
    /// Returns the unique ID associated with the request.
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
        D: historical_bar::data_types::DataType<S>,
    {
        let id = self.get_next_req_id();
        let msg = make_msg!(
            OutMsg::ReqHistoricalData,
            id,
            security,
            u8::from(false),
            end_date_time,
            bar_size,
            duration,
            u8::from(regular_trading_hours_only),
            data,
            1,
            u8::from(false),
            ""
        );
        self.writer.write_all(msg.as_bytes()).await?;
        Ok(id)
    }

    /// Request historical bar data taht remains updated for a given security.
    /// See [`historical_bar`] for types and traits that are used in this funciton.
    ///
    /// # Arguments
    /// * `security` - The security for which to request data.
    /// * `duration` - The duration for which historical data be returned (ie. the difference
    /// between the first bar's datetime and the last bar's datetime).
    /// * `bar_size` - The size of each individual bar.
    /// * `data` - The type of data that to return (price, volume, volatility, etc.).
    /// * `regular_trading_hours_only` - When [`true`], only return bars from regular trading hours.
    ///
    /// # Errors
    /// Returns any error encountered while writing the outgoing message.
    ///
    /// # Returns
    /// Returns the unique ID associated with the request.
    pub async fn req_updating_historical_bar<S, D>(
        &mut self,
        security: &S,
        duration: historical_bar::Duration,
        bar_size: historical_bar::Size,
        data: D,
        regular_trading_hours_only: bool,
    ) -> IdResult
    where
        S: Security,
        D: historical_bar::data_types::DataType<S>,
    {
        let id = self.get_next_req_id();
        let msg = make_msg!(
            OutMsg::ReqHistoricalData,
            id,
            security,
            u8::from(false),
            "",
            bar_size,
            duration,
            u8::from(regular_trading_hours_only),
            data,
            1,
            u8::from(true),
            ""
        );
        self.writer.write_all(msg.as_bytes()).await?;
        Ok(id)
    }

    /// Cancel an existing [`historical_bar`] data request.
    ///
    /// # Arguments
    /// * `req_id` - The ID of the [`historical_bar`] request to cancel.
    ///
    /// # Errors
    /// Returns any error encountered while writing the outgoing message.
    pub async fn cancel_historical_bar(&mut self, req_id: i64) -> ReqResult {
        const VERSION: u8 = 1;
        let msg = make_msg!(OutMsg::CancelHistoricalData, VERSION, req_id);
        self.writer.write_all(msg.as_bytes()).await
    }

    /// Request the earliest available data point for a given security and data type.
    ///
    /// # Arguments
    /// `security` - The security for which to make the request.
    /// `data` - The data for which to make the request.
    /// * `regular_trading_hours_only` - When [`true`], only return ticks from regular trading
    /// hours.
    ///
    /// # Errors
    /// Returns any error encountered while writing the outgoing message.
    ///
    /// # Returns
    /// Returns the unique ID associated with the request.
    pub async fn req_head_timestamp<S, D>(
        &mut self,
        security: &S,
        data: D,
        regular_trading_hours_only: bool,
    ) -> IdResult
    where
        S: Security,
        D: historical_ticks::data_types::DataType<S>,
    {
        let id = self.get_next_req_id();
        let msg = make_msg!(
            OutMsg::ReqHeadTimestamp,
            id,
            security,
            "",
            u8::from(regular_trading_hours_only),
            data,
            "1"
        );
        self.writer.write_all(msg.as_bytes()).await?;
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
        let msg = make_msg!(OutMsg::CancelHeadTimestamp, req_id);

        self.writer.write_all(msg.as_bytes()).await
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
    /// Returns the unique ID associated with the request.
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
        let msg = make_msg!(
            OutMsg::ReqHistogramData,
            id,
            security,
            "",
            u8::from(regular_trading_hours_only),
            duration
        );
        self.writer.write_all(msg.as_bytes()).await?;
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
        let msg = make_msg!(OutMsg::CancelHistogramData, req_id);
        self.writer.write_all(msg.as_bytes()).await
    }

    /// Request historical ticks for a given security. See [`historical_ticks`] for
    /// types and traits that are used in this funciton.
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
    /// Returns the unique ID associated with the request.
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
        D: historical_ticks::data_types::DataType<S>,
    {
        let id = self.get_next_req_id();
        let msg = make_msg!(
            OutMsg::ReqHistoricalTicks,
            id,
            security,
            "",
            timestamp,
            number_of_ticks,
            data,
            u8::from(regular_trading_hours_only),
            "",
            ""
        );
        self.writer.write_all(msg.as_bytes()).await?;
        Ok(id)
    }

    // === Live Market Data ===

    /// Request live data for a given security.
    ///
    /// # Arguments
    /// * `security` - The security for which to request data.
    /// * `data` - The type of data to return (`RealTimeVolume`, `MarkPrice`, etc.).
    /// * `refresh_type` - How often to refresh the data (a one-time snapshot or a continuous
    /// streaming connection)
    /// * `use_regulatory_snapshot` - When set to [`true`], return a NBBO snapshot even if no
    /// appropriate subscription exists for streaming data. Note that doing so will cost 1 cent per
    /// snapshot.
    ///
    /// # Errors
    /// Returns any error encountered while writing the outgoing message.
    ///
    /// # Returns
    /// Returns the unique ID associated with the request.
    pub async fn req_market_data<S, D>(
        &mut self,
        security: &S,
        additional_data: Vec<D>,
        refresh_type: live_data::RefreshType,
        use_regulatory_snapshot: bool,
    ) -> IdResult
    where
        S: Security,
        D: live_data::data_types::DataType<S>,
    {
        const VERSION: u8 = 11;
        let id = self.get_next_req_id();
        let msg = make_msg!(
            OutMsg::ReqMktData,
            VERSION,
            id,
            security,
            u8::from(false),
            additional_data
                .iter()
                .map(std::string::ToString::to_string)
                .collect::<Vec<String>>()
                .join(","),
            refresh_type,
            u8::from(use_regulatory_snapshot),
            ""
        );
        self.writer.write_all(msg.as_bytes()).await?;
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
        let msg = make_msg!(OutMsg::CancelMktData, VERSION, req_id);
        self.writer.write_all(msg.as_bytes()).await
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
        let msg = make_msg!(OutMsg::ReqMarketDataType, VERSION, variant);
        self.writer.write_all(msg.as_bytes()).await
    }

    /// Request real-time, 5 second bars for a given security.
    ///
    /// # Arguments
    /// * `security` - The security for which to request the bars.
    /// * `data` - The type of data to return (trades, bid, ask, midpoint).
    /// * `regular_trading_hours_only` -  When [`true`], only return ticks from regular trading
    /// hours.
    ///
    /// # Errors
    /// Returns any error encountered while writing the outgoing message.
    ///
    /// # Returns
    /// Returns the unique ID associated with the request.
    pub async fn req_real_time_bars<S, D>(
        &mut self,
        security: &S,
        data: D,
        regular_trading_hours_only: bool,
    ) -> IdResult
    where
        S: Security,
        D: live_bar::data_types::DataType<S>,
    {
        const VERSION: u8 = 3;
        let id = self.get_next_req_id();
        let msg = make_msg!(
            OutMsg::ReqRealTimeBars,
            VERSION,
            id,
            security,
            5_u32,
            data,
            u8::from(regular_trading_hours_only),
            ""
        );
        self.writer.write_all(msg.as_bytes()).await?;

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
        let msg = make_msg!(OutMsg::CancelRealTimeBars, VERSION, req_id);
        self.writer.write_all(msg.as_bytes()).await
    }

    // === Live Tick-by-Tick Data ===

    /// Request live tick-by-tick data for a given security.
    ///
    /// # Arguments
    /// * `security` - The security for which to request data.
    /// * `tick_data` - The type of data to return.
    /// * `number_of_historical_ticks` - The number of historical ticks to return before the live
    /// data.
    /// * `ignore_size` - Ignore the size parameter in the returned ticks when set to [`true`].
    ///
    /// # Errors
    /// Returns any error encountered while writing the outgoing message.
    ///
    /// # Returns
    /// Returns the unique ID associated with the request.
    pub async fn req_tick_by_tick_data<S, D>(
        &mut self,
        security: &S,
        tick_data: D,
        number_of_historical_ticks: live_ticks::NumberOfTicks,
        ignore_size: bool,
    ) -> IdResult
    where
        S: Security,
        D: live_ticks::data_types::DataType<S>,
    {
        let id = self.get_next_req_id();
        let msg = make_msg!(
            OutMsg::ReqTickByTickData,
            id,
            security,
            tick_data,
            number_of_historical_ticks,
            u8::from(ignore_size)
        );
        self.writer.write_all(msg.as_bytes()).await?;
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
        let msg = make_msg!(OutMsg::CancelTickByTickData, req_id);
        self.writer.write_all(msg.as_bytes()).await
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
    /// Returns the unique ID associated with the request.
    pub async fn req_market_depth<S>(&mut self, security: &S, number_of_rows: u32) -> IdResult
    where
        S: Security,
    {
        const VERSION: u8 = 5;
        let id = self.get_next_req_id();
        let msg = make_msg!(
            OutMsg::ReqMktDepth,
            VERSION,
            id,
            security,
            number_of_rows,
            u8::from(true),
            ""
        );
        self.writer.write_all(msg.as_bytes()).await?;

        Ok(id)
    }

    /// Request exchanges available for market depth.
    ///
    /// # Errors
    /// Returns any error encountered while writing the outgoing message.
    pub async fn req_market_depth_exchanges(&mut self) -> ReqResult {
        let msg = make_msg!(OutMsg::ReqMktDepthExchanges);
        self.writer.write_all(msg.as_bytes()).await
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
        let msg = make_msg!(OutMsg::CancelMktDepth, VERSION, req_id);
        self.writer.write_all(msg.as_bytes()).await
    }

    /// Request exchanges comprising the aggregate SMART exchange
    ///
    /// # Arguments
    /// * `exchange_id` - The identifier containing information about
    ///
    /// # Errors
    /// Returns any error encountered while writing the outgoing message.
    ///
    /// # Returns
    /// Returns the unique ID associated with the request.
    pub async fn req_smart_components(&mut self, exchange_id: ExchangeId) -> IdResult {
        let id = self.get_next_req_id();
        let msg = make_msg!(OutMsg::ReqSmartComponents, id, exchange_id);
        self.writer.write_all(msg.as_bytes()).await?;

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
    /// Returns the unique ID associated with the request.
    pub async fn req_place_order<S, E>(&mut self, order: &Order<'_, S, E>) -> IdResult
    where
        S: Security,
        E: Executable<S>,
    {
        let id = self.get_next_order_id();
        let msg = make_msg!(OutMsg::PlaceOrder, id, order.get_security(), "", "", order);
        self.writer.write_all(msg.as_bytes()).await?;
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
    /// Returns the unique ID associated with the request.
    pub async fn req_modify_order<S, E>(&mut self, order: &Order<'_, S, E>, id: i64) -> IdResult
    where
        S: Security,
        E: Executable<S>,
    {
        let msg = make_msg!(OutMsg::PlaceOrder, id, order.get_security(), "", "", order);
        self.writer.write_all(msg.as_bytes()).await?;
        Ok(id)
    }

    /// Cancel an order.
    ///
    /// # Arguments
    /// * `id` - The ID of the order to cancel.
    ///
    /// # Errors
    /// Returns any error encountered while writing the outgoing message.
    pub async fn cancel_order<S>(&mut self, id: i64) -> ReqResult
    where
        S: Security,
    {
        const VERSION: u8 = 1;
        let msg = make_msg!(OutMsg::CancelOrder, VERSION, id, "");
        self.writer.write_all(msg.as_bytes()).await
    }

    /// Cancel all currently open orders, including those placed in TWS.
    ///
    /// # Errors
    /// Returns any error encountered while writing the outgoing message.
    pub async fn cancel_all_orders(&mut self) -> ReqResult {
        const VERSION: u8 = 1;
        let msg = make_msg!(OutMsg::ReqGlobalCancel, VERSION);
        self.writer.write_all(msg.as_bytes()).await
    }

    // === Contract Creation ===

    #[inline]
    pub(crate) async fn send_contract_query(
        &mut self,
        contract_id: ContractId,
    ) -> anyhow::Result<()> {
        const VERSION: u8 = 8;
        let req_id = self.get_next_req_id();
        let msg = make_msg!(
            OutMsg::ReqContractData,
            VERSION,
            req_id,
            contract_id.0,
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            ""
        );
        self.status
            .tx
            .send(ToWrapper::ContractQuery((contract_id, req_id)))
            .await?;
        self.writer.write_all(msg.as_bytes()).await?;
        Ok(())
    }

    #[inline]
    pub(crate) async fn recv_contract_query(
        &mut self,
    ) -> anyhow::Result<crate::contract::Contract> {
        match self
            .status
            .rx
            .recv()
            .await
            .ok_or_else(|| anyhow::Error::msg("Failed to receive contract object"))?
        {
            ToClient::NewContract(c) => Ok(c),
            _ => Err(anyhow::Error::msg("No valid contract object received")),
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
    /// Returns a [`Builder`] with the same port and address as the existing client.
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
