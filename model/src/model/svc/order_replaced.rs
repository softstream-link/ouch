use crate::prelude::*;
use byteserde::prelude::*;
use byteserde_derive::{ByteDeserializeSlice, ByteSerializeStack, ByteSerializedLenOf};

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

#[derive(ByteSerializeStack, ByteDeserializeSlice, ByteSerializedLenOf, PartialEq, Clone, Copy, Debug, Default)]
#[byteserde(peek(1, 1))] // peek(start, len) -> peek one byte after skipping one
pub struct OrderReplacedAppendage {
    #[byteserde(eq(Firm::tag_as_slice()))]
    pub firm: Option<TagValueElement<Firm>>,

    #[byteserde(eq(MinQty::tag_as_slice()))]
    pub min_qty: Option<TagValueElement<MinQty>>,

    #[byteserde(eq(MaxFloor::tag_as_slice()))]
    pub max_floor: Option<TagValueElement<MaxFloor>>,

    #[byteserde(eq(PriceType::tag_as_slice()))]
    pub price_type: Option<TagValueElement<PriceType>>,

    #[byteserde(eq(PostOnly::tag_as_slice()))]
    pub post_only: Option<TagValueElement<PostOnly>>,

    #[byteserde(eq(ExpireTimeSec::tag_as_slice()))]
    pub expire_time: Option<TagValueElement<ExpireTimeSec>>,

    #[byteserde(eq(TradeNow::tag_as_slice()))]
    pub trade_now: Option<TagValueElement<TradeNow>>,

    #[byteserde(eq(HandleInst::tag_as_slice()))]
    pub handle_inst: Option<TagValueElement<HandleInst>>,

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

#[derive(ByteSerializeStack, ByteDeserializeSlice, ByteSerializedLenOf, PartialEq, Clone, Debug)]
#[byteserde(endian = "be")]
pub struct OrderReplaced {
    packet_type: PacketTypeOrderReplaced,

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

    capacity: Capacity, // from original order chain
    int_mkt_sweep_eligibility: IntMktSweepEligibility,
    cross_type: CrossType, // from original order chain

    order_state: OrderState, // Venue assigned

    clt_order_id: CltOrderId,
    #[byteserde(replace( appendages.byte_len() ))]
    appendage_length: u16,
    #[byteserde(deplete(appendage_length))]
    appendages: OrderReplacedAppendage,
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

#[cfg(test)]
mod test {
    use super::*;
    use links_core::unittest::setup;

    use log::info;

    #[test]
    fn test_msg() {
        setup::log::configure();
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
}
