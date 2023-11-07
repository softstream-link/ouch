use crate::{model::clt::_01_enter_order::EnterOrderAppendage, prelude::*};
use byteserde::prelude::*;
use byteserde_derive::{ByteDeserializeSlice, ByteSerializeStack, ByteSerializedLenOf};
use serde::{Deserialize, Serialize};

// page 10 from https://nasdaqtrader.com/content/technicalsupport/specifications/TradingProducts/Ouch5.0.pdf
// Firm
// MinQty
// CustomerType
// MaxFloor
// PriceType
// PegOffset
// Discretion
// DiscretionPrice
// DiscretionPegType
// DiscretionPegOffset
// PostOnly
// RandomReserves
// Route
// ExpireTime
// TradeNow
// HandleInst
// BBO Weight Indicator
// GroupID
// SharesLocated

#[derive(ByteSerializeStack, ByteDeserializeSlice, ByteSerializedLenOf, Serialize, Deserialize, PartialEq, Clone, Copy, Debug, Default)]
#[byteserde(peek(1, 1))] // peek(start, len) -> peek one byte after skipping one
pub struct OrderAcceptedAppendage {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[byteserde(eq(Firm::tag_as_slice()))]
    pub firm: Option<TagValueElement<Firm>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[byteserde(eq(MinQty::tag_as_slice()))]
    pub min_qty: Option<TagValueElement<MinQty>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[byteserde(eq(CustomerType::tag_as_slice()))]
    pub customer_type: Option<TagValueElement<CustomerType>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[byteserde(eq(MaxFloor::tag_as_slice()))]
    pub max_floor: Option<TagValueElement<MaxFloor>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[byteserde(eq(PriceType::tag_as_slice()))]
    pub price_type: Option<TagValueElement<PriceType>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[byteserde(eq(PegOffset::tag_as_slice()))]
    pub peg_offset: Option<TagValueElement<PegOffset>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[byteserde(eq(DiscretionPrice::tag_as_slice()))]
    pub discretion_price: Option<TagValueElement<DiscretionPrice>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[byteserde(eq(DiscretionPriceType::tag_as_slice()))]
    pub discretion_price_type: Option<TagValueElement<DiscretionPriceType>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[byteserde(eq(DiscretionPegOffset::tag_as_slice()))]
    pub discretion_peg_offset: Option<TagValueElement<DiscretionPegOffset>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[byteserde(eq(PostOnly::tag_as_slice()))]
    pub post_only: Option<TagValueElement<PostOnly>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[byteserde(eq(RandomReserves::tag_as_slice()))]
    pub random_reserves: Option<TagValueElement<RandomReserves>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[byteserde(eq(Route::tag_as_slice()))]
    pub route: Option<TagValueElement<Route>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[byteserde(eq(ExpireTime::tag_as_slice()))]
    pub expire_time: Option<TagValueElement<ExpireTime>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[byteserde(eq(TradeNow::tag_as_slice()))]
    pub trade_now: Option<TagValueElement<TradeNow>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[byteserde(eq(HandleInst::tag_as_slice()))]
    pub handle_inst: Option<TagValueElement<HandleInst>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[byteserde(eq(BBOWeightIndicator::tag_as_slice()))]
    pub bbo_weight_indicator: Option<TagValueElement<BBOWeightIndicator>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[byteserde(eq(GroupId::tag_as_slice()))]
    pub group_id: Option<TagValueElement<GroupId>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[byteserde(eq(SharesLocated::tag_as_slice()))]
    pub shares_located: Option<TagValueElement<SharesLocated>>,
}
impl From<&EnterOrderAppendage> for OrderAcceptedAppendage {
    #[inline(always)]
    fn from(value: &EnterOrderAppendage) -> Self {
        OrderAcceptedAppendage {
            firm: value.firm,
            min_qty: value.min_qty,
            customer_type: value.customer_type,
            max_floor: value.max_floor,
            price_type: value.price_type,
            peg_offset: value.peg_offset,
            discretion_price: value.discretion_price,
            discretion_price_type: value.discretion_price_type,
            discretion_peg_offset: value.discretion_peg_offset,
            post_only: value.post_only,
            random_reserves: value.random_reserves,
            route: value.route,
            expire_time: value.expire_time,
            trade_now: value.trade_now,
            handle_inst: value.handle_inst,
            bbo_weight_indicator: None,
            group_id: value.group_id,
            shares_located: value.shares_located,
        }
    }
}

#[derive(ByteSerializeStack, ByteDeserializeSlice, ByteSerializedLenOf, Serialize, Deserialize, PartialEq, Clone, Debug)]
#[byteserde(endian = "be")]
#[serde(from = "OrderAcceptedJsonDes")]
pub struct OrderAccepted {
    #[serde(default, skip_serializing)]
    packet_type: PacketTypeOrderAccepted,
    pub timestamp: Timestamp, // Venue assigned
    pub user_ref_number: UserRefNumber,
    pub side: Side,
    pub quantity: Quantity,
    pub symbol: Symbol,
    pub price: Price,
    pub time_in_force: TimeInForce,
    pub display: Display,
    pub order_reference_number: OrderReferenceNumber, // Venue assigned
    pub capacity: Capacity,
    pub int_mkt_sweep_eligibility: IntMktSweepEligibility,
    pub cross_type: CrossType,
    pub order_state: OrderState, // Venue assigned
    pub clt_order_id: CltOrderId,
    #[serde(skip)]
    #[byteserde(replace( appendages.byte_len() ))]
    appendage_length: u16,
    #[byteserde(deplete(appendage_length))]
    pub appendages: OrderAcceptedAppendage,
}
impl From<(&EnterOrder, OrderReferenceNumber, OrderState)> for OrderAccepted {
    #[inline(always)]
    fn from(value: (&EnterOrder, OrderReferenceNumber, OrderState)) -> Self {
        let (enter_order, order_reference_number, order_state) = value;
        Self {
            packet_type: PacketTypeOrderAccepted::default(),
            timestamp: Timestamp::default(), // Venue assigned
            user_ref_number: enter_order.user_ref_number,
            side: enter_order.side,
            quantity: enter_order.quantity,
            symbol: enter_order.symbol,
            price: enter_order.price,
            time_in_force: enter_order.time_in_force,
            display: enter_order.display,
            order_reference_number: order_reference_number, // Venue assigned
            capacity: enter_order.capacity,
            int_mkt_sweep_eligibility: enter_order.int_mkt_sweep_eligibility,
            cross_type: enter_order.cross_type,
            order_state: order_state, // Venue assigned
            clt_order_id: enter_order.clt_order_id,
            appendage_length: enter_order.appendages.byte_len() as u16,
            appendages: (&enter_order.appendages).into(),
        }
    }
}

#[derive(Deserialize)]
struct OrderAcceptedJsonDes {
    timestamp: Timestamp, // Venue assigned
    user_ref_number: UserRefNumber,
    side: Side,
    quantity: Quantity,
    symbol: Symbol,
    price: Price,
    time_in_force: TimeInForce,
    display: Display,
    order_reference_number: OrderReferenceNumber, // Venue assigned
    capacity: Capacity,
    int_mkt_sweep_eligibility: IntMktSweepEligibility,
    cross_type: CrossType,
    order_state: OrderState, // Venue assigned
    clt_order_id: CltOrderId,
    appendages: OrderAcceptedAppendage,
}
impl From<OrderAcceptedJsonDes> for OrderAccepted {
    fn from(value: OrderAcceptedJsonDes) -> Self {
        Self {
            packet_type: PacketTypeOrderAccepted::default(),
            timestamp: value.timestamp, // Venue assigned
            user_ref_number: value.user_ref_number,
            side: value.side,
            quantity: value.quantity,
            symbol: value.symbol,
            price: value.price,
            time_in_force: value.time_in_force,
            display: value.display,
            order_reference_number: value.order_reference_number, // Venue assigned
            capacity: value.capacity,
            int_mkt_sweep_eligibility: value.int_mkt_sweep_eligibility,
            cross_type: value.cross_type,
            order_state: value.order_state, // Venue assigned
            clt_order_id: value.clt_order_id,
            appendage_length: value.appendages.byte_len() as u16,
            appendages: value.appendages,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use links_core::unittest::setup;

    use log::info;
    use serde_json::{from_str, to_string};
    use text_diff::{diff, print_diff};

    #[test]
    fn test_msg_byteserde() {
        setup::log::configure_compact();

        let enter_order = EnterOrder::default();
        let msg_inp = OrderAccepted::from((&enter_order, OrderReferenceNumber::new(1), OrderState::live()));

        let ser: ByteSerializerStack<256> = to_serializer_stack(&msg_inp).unwrap();
        info!("ser: {:#x}", ser);

        let msg_out: OrderAccepted = from_serializer_stack(&ser).unwrap();

        info!("msg_inp: {:?}", msg_inp);
        info!("msg_out: {:?}", msg_out);
        assert_eq!(msg_out, msg_inp);
    }

    #[test]
    fn test_msg_serde() {
        setup::log::configure_compact();

        let enter_order = EnterOrder::default();
        let mut msg_inp = OrderAccepted::from((&enter_order, OrderReferenceNumber::new(1), OrderState::live()));
        msg_inp.timestamp = Timestamp::new(1);
        // info!("msg_inp: {:?}", msg_inp);

        let json_out = to_string(&msg_inp).unwrap();
        let json_exp = r#"{"timestamp":1,"user_ref_number":1,"side":"BUY","quantity":100,"symbol":"DUMMY","price":1.2345,"time_in_force":"MARKET_HOURS","display":"VISIBLE","order_reference_number":1,"capacity":"AGENCY","int_mkt_sweep_eligibility":"ELIGIBLE","cross_type":"CONTINUOUS_MARKET","order_state":"LIVE","clt_order_id":"1","appendages":{"firm":"????","min_qty":0,"customer_type":"PORT_DEFAULT","max_floor":0,"price_type":"LIMIT","peg_offset":-1.1234,"discretion_price":0.0,"discretion_price_type":"LIMIT","discretion_peg_offset":-1.1234,"post_only":"NO","random_reserves":0,"route":"????","expire_time":0,"trade_now":"PORT_DEFAULT","handle_inst":"NO_INSTRUCTIONS","group_id":0,"shares_located":"NO"}}"#;
        info!("json_out: {}", json_out);

        if matches!(diff(&json_out, json_exp, ","), (dist, _) if dist != 0) {
            print_diff(&json_out, json_exp, ",");
            assert_eq!(json_out, json_exp);
        }

        let msg_out: OrderAccepted = from_str(&json_out).unwrap();
        // info!("msg_out: {:?}", msg_out);
        assert_eq!(msg_out, msg_inp);
    }
}
