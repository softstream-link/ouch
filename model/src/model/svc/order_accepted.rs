use crate::{model::clt::_01_enter_order::EnterOrderAppendage, prelude::*};
use byteserde::prelude::*;
use byteserde_derive::{ByteDeserializeSlice, ByteSerializeStack, ByteSerializedLenOf};

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

#[derive(ByteSerializeStack, ByteDeserializeSlice, ByteSerializedLenOf, PartialEq, Clone, Copy, Debug, Default)]
#[byteserde(peek(1, 1))] // peek(start, len) -> peek one byte after skipping one
pub struct OrderAcceptedAppendage {
    #[byteserde(eq(Firm::tag_as_slice()))]
    pub firm: Option<TagValueElement<Firm>>,

    #[byteserde(eq(MinQty::tag_as_slice()))]
    pub min_qty: Option<TagValueElement<MinQty>>,

    #[byteserde(eq(CustomerType::tag_as_slice()))]
    pub customer_type: Option<TagValueElement<CustomerType>>,

    #[byteserde(eq(MaxFloor::tag_as_slice()))]
    pub max_floor: Option<TagValueElement<MaxFloor>>,

    #[byteserde(eq(PriceType::tag_as_slice()))]
    pub price_type: Option<TagValueElement<PriceType>>,

    #[byteserde(eq(PegOffset::tag_as_slice()))]
    pub peg_offset: Option<TagValueElement<PegOffset>>,

    #[byteserde(eq(DiscretionPrice::tag_as_slice()))]
    pub discretion_price: Option<TagValueElement<DiscretionPrice>>,

    #[byteserde(eq(DiscretionPriceType::tag_as_slice()))]
    pub discretion_price_type: Option<TagValueElement<DiscretionPriceType>>,

    #[byteserde(eq(DiscretionPegOffset::tag_as_slice()))]
    pub discretion_peg_offset: Option<TagValueElement<DiscretionPegOffset>>,

    #[byteserde(eq(PostOnly::tag_as_slice()))]
    pub post_only: Option<TagValueElement<PostOnly>>,

    #[byteserde(eq(RandomReserves::tag_as_slice()))]
    pub random_reserves: Option<TagValueElement<RandomReserves>>,

    #[byteserde(eq(Route::tag_as_slice()))]
    pub route: Option<TagValueElement<Route>>,

    #[byteserde(eq(ExpireTimeSec::tag_as_slice()))]
    pub expire_time: Option<TagValueElement<ExpireTimeSec>>,

    #[byteserde(eq(TradeNow::tag_as_slice()))]
    pub trade_now: Option<TagValueElement<TradeNow>>,

    #[byteserde(eq(HandleInst::tag_as_slice()))]
    pub handle_inst: Option<TagValueElement<HandleInst>>,

    #[byteserde(eq(BBOWeightIndicator::tag_as_slice()))]
    pub bbo_weight_indicator: Option<TagValueElement<BBOWeightIndicator>>,

    #[byteserde(eq(GroupId::tag_as_slice()))]
    pub group_id: Option<TagValueElement<GroupId>>,

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

#[derive(ByteSerializeStack, ByteDeserializeSlice, ByteSerializedLenOf, PartialEq, Clone, Debug)]
#[byteserde(endian = "be")]
pub struct OrderAccepted {
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
    #[byteserde(replace( appendages.byte_len() ))]
    appendage_length: u16,
    #[byteserde(deplete(appendage_length))]
    pub appendages: OrderAcceptedAppendage,
}

impl From<&EnterOrder> for OrderAccepted {
    #[inline(always)]
    fn from(enter_order: &EnterOrder) -> Self {
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

            order_reference_number: OrderReferenceNumber::default(), // Venue assigned

            capacity: enter_order.capacity,
            int_mkt_sweep_eligibility: enter_order.int_mkt_sweep_eligibility,
            cross_type: enter_order.cross_type,

            order_state: OrderState::live(), // Venue assigned

            clt_order_id: enter_order.clt_order_id,
            appendage_length: enter_order.appendages.byte_len() as u16,
            appendages: (&enter_order.appendages).into(),
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
        let msg_inp = OrderAccepted::from(&enter_order);

        let ser: ByteSerializerStack<128> = to_serializer_stack(&msg_inp).unwrap();
        info!("ser: {:#x}", ser);

        let msg_out: OrderAccepted = from_serializer_stack(&ser).unwrap();

        info!("msg_inp: {:?}", msg_inp);
        info!("msg_out: {:?}", msg_out);
        assert_eq!(msg_out, msg_inp);
    }
}
