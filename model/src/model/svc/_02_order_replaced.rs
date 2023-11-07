use crate::prelude::*;
use byteserde::prelude::*;
use byteserde_derive::{ByteDeserializeSlice, ByteSerializeStack, ByteSerializedLenOf};
use serde::{Deserialize, Serialize};

// page 12 from https://nasdaqtrader.com/content/technicalsupport/specifications/TradingProducts/Ouch5.0.pdf
// Firm
// MinQty
// MaxFloor
// PriceType
// PostOnly
// ExpireTime
// TradeNow
// HandleInst
// BBO Weight Indicator

#[derive(ByteSerializeStack, ByteDeserializeSlice, ByteSerializedLenOf, Serialize, Deserialize, PartialEq, Clone, Copy, Debug, Default)]
#[byteserde(peek(1, 1))] // peek(start, len) -> peek one byte after skipping one
pub struct OrderReplacedAppendage {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[byteserde(eq(Firm::tag_as_slice()))]
    pub firm: Option<TagValueElement<Firm>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[byteserde(eq(MinQty::tag_as_slice()))]
    pub min_qty: Option<TagValueElement<MinQty>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[byteserde(eq(MaxFloor::tag_as_slice()))]
    pub max_floor: Option<TagValueElement<MaxFloor>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[byteserde(eq(PriceType::tag_as_slice()))]
    pub price_type: Option<TagValueElement<PriceType>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[byteserde(eq(PostOnly::tag_as_slice()))]
    pub post_only: Option<TagValueElement<PostOnly>>,

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
}
impl From<(&EnterOrderAppendage, &ReplaceOrderAppendage)> for OrderReplacedAppendage {
    #[inline(always)]
    fn from(value: (&EnterOrderAppendage, &ReplaceOrderAppendage)) -> Self {
        let enter_order_appendage = value.0;
        let replace_order_appendage = value.1;
        Self {
            firm: enter_order_appendage.firm,
            min_qty: replace_order_appendage.min_qty,
            max_floor: replace_order_appendage.max_floor,
            price_type: replace_order_appendage.price_type,
            post_only: replace_order_appendage.post_only,
            expire_time: replace_order_appendage.expire_time,
            trade_now: replace_order_appendage.trade_now,
            handle_inst: replace_order_appendage.handle_inst,
            bbo_weight_indicator: None,
        }
    }
}

#[derive(ByteSerializeStack, ByteDeserializeSlice, ByteSerializedLenOf, Serialize, Deserialize, PartialEq, Clone, Debug)]
#[byteserde(endian = "be")]
#[serde(from = "OrderReplacedJsonDes")]
pub struct OrderReplaced {
    #[serde(default, skip_serializing)]
    packet_type: PacketTypeOrderReplaced,
    pub timestamp: Timestamp, // Venue assigned
    pub orig_user_ref_number: UserRefNumber,
    pub user_ref_number: UserRefNumber,
    pub side: Side, // from original order chain
    pub quantity: Quantity,
    pub symbol: Symbol, // from original order chain
    pub price: Price,
    pub time_in_force: TimeInForce,
    pub display: Display,
    pub order_reference_number: OrderReferenceNumber, // Venue assigned
    pub capacity: Capacity,                           // from original order chain
    pub int_mkt_sweep_eligibility: IntMktSweepEligibility,
    pub cross_type: CrossType,   // from original order chain
    pub order_state: OrderState, // Venue assigned
    pub clt_order_id: CltOrderId,
    #[serde(skip)]
    #[byteserde(replace( appendages.byte_len() ))]
    appendage_length: u16,
    #[byteserde(deplete(appendage_length))]
    pub appendages: OrderReplacedAppendage,
}
impl From<(&EnterOrder, &ReplaceOrder)> for OrderReplaced {
    #[inline(always)]
    fn from(value: (&EnterOrder, &ReplaceOrder)) -> Self {
        let (enter_order, replace_order) = value;
        let appendages: OrderReplacedAppendage = (&enter_order.appendages, &replace_order.appendages).into();
        OrderReplaced {
            packet_type: PacketTypeOrderReplaced::default(),

            timestamp: Timestamp::default(),                         // Venue assigned
            order_reference_number: OrderReferenceNumber::default(), // default placeholder must be replaced
            order_state: OrderState::live(),                         // Venue assigned

            orig_user_ref_number: replace_order.orig_user_ref_number,
            user_ref_number: enter_order.user_ref_number, // enter_order
            side: enter_order.side,                       // enter_order
            symbol: enter_order.symbol,                   // enter_order
            capacity: enter_order.capacity,               // enter_order
            cross_type: enter_order.cross_type,           // enter_order

            quantity: replace_order.quantity,
            price: replace_order.price,
            time_in_force: replace_order.time_in_force,
            display: replace_order.display,
            int_mkt_sweep_eligibility: replace_order.int_mkt_sweep_eligibility,

            clt_order_id: replace_order.clt_order_id,
            appendage_length: appendages.byte_len() as u16,
            appendages,
        }
    }
}

#[derive(Deserialize)]
struct OrderReplacedJsonDes {
    timestamp: Timestamp, // Venue assigned
    orig_user_ref_number: UserRefNumber,
    user_ref_number: UserRefNumber,
    side: Side, // from original order chain
    quantity: Quantity,
    symbol: Symbol, // from original order chain
    price: Price,
    time_in_force: TimeInForce,
    display: Display,
    order_reference_number: OrderReferenceNumber, // Venue assigned
    capacity: Capacity,                           // from original order chain
    int_mkt_sweep_eligibility: IntMktSweepEligibility,
    cross_type: CrossType,   // from original order chain
    order_state: OrderState, // Venue assigned
    clt_order_id: CltOrderId,
    appendages: OrderReplacedAppendage,
}
impl From<OrderReplacedJsonDes> for OrderReplaced {
    fn from(value: OrderReplacedJsonDes) -> Self {
        Self {
            packet_type: PacketTypeOrderReplaced::default(),
            timestamp: value.timestamp,
            orig_user_ref_number: value.orig_user_ref_number,
            user_ref_number: value.user_ref_number,
            side: value.side,
            quantity: value.quantity,
            symbol: value.symbol,
            price: value.price,
            time_in_force: value.time_in_force,
            display: value.display,
            order_reference_number: value.order_reference_number,
            capacity: value.capacity,
            int_mkt_sweep_eligibility: value.int_mkt_sweep_eligibility,
            cross_type: value.cross_type,
            order_state: value.order_state,
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
        let mut replace_order = ReplaceOrder::from(&enter_order);
        replace_order.quantity = Quantity::new(50);

        let msg_inp = OrderReplaced::from((&enter_order, &replace_order));

        let ser: ByteSerializerStack<128> = to_serializer_stack(&msg_inp).unwrap();
        info!("ser: {:#x}", ser);

        let msg_out: OrderReplaced = from_serializer_stack(&ser).unwrap();

        info!("msg_inp: {:?}", msg_inp);
        info!("msg_out: {:?}", msg_out);
        assert_eq!(msg_out, msg_inp);
    }

    #[test]
    fn test_msg_serde() {
        setup::log::configure_compact();
        let enter_order = EnterOrder::default();
        let mut replace_order = ReplaceOrder::from(&enter_order);
        replace_order.quantity = Quantity::new(50);

        let mut msg_inp = OrderReplaced::from((&enter_order, &replace_order));
        msg_inp.timestamp = Timestamp::from(1);

        let json_out = to_string(&msg_inp).unwrap();
        let json_exp = r#"{"timestamp":1,"orig_user_ref_number":1,"user_ref_number":1,"side":"BUY","quantity":50,"symbol":"DUMMY","price":1.2345,"time_in_force":"MARKET_HOURS","display":"VISIBLE","order_reference_number":0,"capacity":"AGENCY","int_mkt_sweep_eligibility":"ELIGIBLE","cross_type":"CONTINUOUS_MARKET","order_state":"LIVE","clt_order_id":"REPLACE_ME____","appendages":{"firm":"????","min_qty":0,"max_floor":0,"price_type":"LIMIT","post_only":"NO","expire_time":0,"trade_now":"PORT_DEFAULT","handle_inst":"NO_INSTRUCTIONS"}}"#;
        info!("json_out: {}", json_out);

        if matches!(diff(&json_out, json_exp, ","), (dist, _) if dist != 0) {
            print_diff(&json_out, json_exp, ",");
            assert_eq!(json_out, json_exp);
        }

        let msg_out: OrderReplaced = from_str(&json_out).unwrap();
        // info!("msg_out: {:?}", msg_out);
        assert_eq!(msg_out, msg_inp);
    }
}
