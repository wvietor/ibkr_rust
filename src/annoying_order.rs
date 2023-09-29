(
Out::PlaceOrder,
order_id,
contract,
None::<()>,
None::<()>,
order
)


order:
(
    "BUY"/"SELL",
    total_quantity,
    order_type,
    limit_price, // Default = ""
    aux_price, //  Default = ""
    time_in_force, // Default = ""
    oca_group, //  Default = "" (One cancels all group)
    account, //  Default = "" (IB account)
    None::<()>, //  Default = "", valid={O: Open, C: Close} (institutional only)
    origin, // Default = 0, valid={0: Customer, 1: Firm}
    order_ref, // Default = "",
    transmit, // Default = true
    parent_id, // Default = 0, Parent order Id, to associate Auto STP or TRAIL orders with the original order.
    block_order, // Default = false
    sweep_to_fill, // Default = false
    display_size, // Default = 0
    trigger_method, // Default=0, valid = {0: Default, 1: Double_Bid_Ask, 2: Last, 3: Double_Last, 4: Bid_Ask, 7: Last_or_Bid_Ask, 8: Mid-point}
    outside_rth, // Default = false
    hidden, // Default = false (only applies for Nasdaq)
    None::<()>, // Deprecated field
    discretionary_amount, // Default = 0
    good_after_time, // Default = "", Format: 20060505 08:00:00 {time zone}
    good_til_date, // Default = "", Format: 20060505 08:00:00 {time zone}
    None::<()>, // Default = "" (institutional only)
    None::<()>, // Default = ""(institutional only)
    None::<()>, // Default = ""(institutional only)
    model_code, // Default =""
    0, // Default=0 (institutional only)
    None::<()>, // Default ="" (institutional only)
    -1, // Default = -1 (institutional only)
    oca_type, // Default = 0, valid={1: CANCEL_WITH_BLOCK, 2: REDUCE_WITH_BLOCK, 3: REDUCE_NON_BLOCK}
    rule_80a, // Default = "", valid={'I': Individual, 'A': Agency, 'W': AgentOtherMember, 'J': IndividualPTIA, 'U': AgencyPTIA, 'M': AgentOtherMemberPTIA, 'K': IndividualPT, 'Y': AgencyPT, 'N': AgentOtherMemberPT}
    None::<()>, // Default= "", (institutional only)
    all_or_none, // Default=false
    min_qty, // Default = ""
    percent_offset, // Default = ""
    false, // Deprecated fields
    false, // Deprecated fields
    None::<()>, // Default = ""
    auction_strategy, // Default = 0, valid={1: Match, 2: Improvement, 3: Transparent}
    starting_price, // Default =""
    stock_ref_price, // Default = ""
    delta, // Default = ""
    stock_range_lower, // Default =""
    stock_range_upper, // Default = ""
    override_percentage_constraints, // Default = false
    volatility, // Defualt = ""
    volatility_type, // Default = "", valid={1: daily, 2: annual}
    delta_neutral_order_type, // Default = ""
    delta_neutral_aux_price, // Default = ""
/*{ if delta_neutral_order_type
flds += [make_field( order.deltaNeutralConId),
make_field( order.deltaNeutralSettlingFirm),
make_field( order.deltaNeutralClearingAccount),
make_field( order.deltaNeutralClearingIntent)]

if self.serverVersion() >= MIN_SERVER_VER_DELTA_NEUTRAL_OPEN_CLOSE and order.deltaNeutralOrderType:
flds += [make_field( order.deltaNeutralOpenClose),
make_field( order.deltaNeutralShortSale),
make_field( order.deltaNeutralShortSaleSlot),
make_field( order.deltaNeutralDesignatedLocation)]
}*/
    continuous_update, // Default = false
    reference_price, // Default = "", valid={1: Average, 2: BidOrAsk}
    trail_stop_price, // Default = "",
    trailing_percent, // Default =""
    scale_init_level_size, // Default = ""
    scale_subs_level_size, // Default = ""
    scale_price_increment, // Default = ""
/*{ if self.serverVersion() >= MIN_SERVER_VER_SCALE_ORDERS3 \
    and order.scalePriceIncrement != UNSET_DOUBLE \
    and order.scalePriceIncrement > 0.0:

    flds += [make_field_handle_empty( order.scalePriceAdjustValue),
        make_field_handle_empty( order.scalePriceAdjustInterval),
        make_field_handle_empty( order.scaleProfitOffset),
        make_field( order.scaleAutoReset),
        make_field_handle_empty( order.scaleInitPosition),
        make_field_handle_empty( order.scaleInitFillQty),
        make_field( order.scaleRandomPercent)]
} */
    scale_table, // Defualt = ""
    active_start_time, // Default =""
    active_stop_time, // Default = ""
    hedge_type, // Default = "", valid = {'D': delta, 'B': beta, 'F': FX, 'P': pair}
/* { if order.hedgeType:
flds.append(make_field( order.hedgeParam))
}*/
    opt_out_smart_routing, // Default = false
    clearing_account, // Default = ""
    clearing_intent, // Default = "", valid = {"": Default, "IB": IB, "Away": Away, "PTA" (PostTrade)}
    not_held, // Default = false
    false, // (see below)
/*{if contract.deltaNeutralContract:
flds += [make_field(True),
make_field(contract.deltaNeutralContract.conId),
make_field(contract.deltaNeutralContract.delta),
make_field(contract.deltaNeutralContract.price)]
else:
flds.append(make_field(False))
}*/
    algo_strategy, // Default = ""
/*{if order.algoStrategy:
algoParamsCount = len(order.algoParams) if order.algoParams else 0
flds.append(make_field(algoParamsCount))
if algoParamsCount > 0:
for algoParam in order.algoParams:
flds += [make_field(algoParam.tag),
make_field(algoParam.value)]

} */
    algo_id, // default = ""
    what_if, // default = false
    misc_options, // default = "", valid = Vec<tagValue> not comma separated though
    solicited, // default = false
    randomize_size, // default = false
    randomize_price, // default = false
/*{if isPegBenchOrder(order.orderType):
flds += [make_field(order.referenceContractId),
make_field(order.isPeggedChangeAmountDecrease),
make_field(order.peggedChangeAmount),
make_field(order.referenceChangeAmount),
make_field(order.referenceExchangeId)]
} */
    len_order_conditions, // clearrly, default=0
/*{if len(order.conditions) > 0:
for cond in order.conditions:
flds.append(make_field(cond.type()))
flds += cond.make_fields()

flds += [make_field(order.conditionsIgnoreRth),
make_field(order.conditionsCancelOrder)]
} */
    adjusted_order_type, // default = ""
    trigger_price, // default = ""
    limit_price_offst, //default = ""
    adjusted_stop_price, // default = ""
    adjusted_stop_limit_price, //default = ""
    adjusted_trailing_amount, // default =""
    adjustable_trailing_unit, //default = 0
    ext_operator, // default = ""
    soft_dollar_tier_name, // default = ""
    soft_dollar_tier_val, // default = ""
    cash_qty, // default = 1.7976931348623157e+308 (max double)
    mifid2_decision_maker, // default = ""
    mifid2_decision_algo, // default = ""
    mifid2_execution_trader, // default = ""
    mifid2_execution_algo, // default = ""
    dont_use_auto_price_for_hedge, // default = false
    is_oms_container, // default = false
    discretionary_up_to_limit_price, // default =False
    use_price_mgmt_algo, // default = "", valid = {true, false, ""}
    duration, // default = ""
    post_to_ats, // default = ""
    auto_cancel_parent, // default= false
    advanced_error_override, // default = ""
    manual_order_time, // default = ""
/*{  sendMidOffsets = False
    if contract.exchange == "IBKRATS":
        flds.append(make_field_handle_empty(order.minTradeQty))
    if isPegBestOrder(order.orderType):
        flds.append(make_field_handle_empty(order.minCompeteSize))
        flds.append(make_field_handle_empty(order.competeAgainstBestOffset))
        if order.competeAgainstBestOffset == COMPETE_AGAINST_BEST_OFFSET_UP_TO_MID:
            sendMidOffsets = True
    elif isPegMidOrder(order.orderType):
            sendMidOffsets = True
    if sendMidOffsets:
        flds.append(make_field_handle_empty(order.midOffsetAtWhole))
        flds.append(make_field_handle_empty(order.midOffsetAtHalf))
}*/
)



