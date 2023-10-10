use crate::prelude::*;
use byteserde::prelude::*;
use byteserde_derive::{ByteDeserializeSlice, ByteSerializeStack, ByteSerializedLenOf};

// page 5 from https://nasdaqtrader.com/content/technicalsupport/specifications/TradingProducts/Ouch5.0.pdf
// Firm
// MinQty
// CustomerType
// MaxFloor
// PriceType
// PegOffset
// DiscretionPrice
// DiscretionPriceType
// DiscretionPegOffset
// PostOnly
// RandomReserves
// Route
// ExpireTime
// TradeNow
// HandleInst
// GroupID
// SharesLocated

#[derive(ByteSerializeStack, ByteDeserializeSlice, ByteSerializedLenOf, PartialEq, Clone, Copy, Debug, Default)]
#[byteserde(peek(1, 1))] // peek(start, len) -> peek one byte after skipping one
pub struct EnterOrderAppendage {
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

    #[byteserde(eq(ExpireTime::tag_as_slice()))]
    pub expire_time: Option<TagValueElement<ExpireTime>>,

    #[byteserde(eq(TradeNow::tag_as_slice()))]
    pub trade_now: Option<TagValueElement<TradeNow>>,

    #[byteserde(eq(HandleInst::tag_as_slice()))]
    pub handle_inst: Option<TagValueElement<HandleInst>>,

    #[byteserde(eq(GroupId::tag_as_slice()))]
    pub group_id: Option<TagValueElement<GroupId>>,

    #[byteserde(eq(SharesLocated::tag_as_slice()))]
    pub shares_located: Option<TagValueElement<SharesLocated>>,
}

#[derive(ByteSerializeStack, ByteDeserializeSlice, ByteSerializedLenOf, PartialEq, Clone, Debug)]
#[byteserde(endian = "be")]
pub struct EnterOrder {
    packet_type: PacketTypeEnterOrder,
    pub user_ref_number: UserRefNumber,
    pub side: Side,
    pub quantity: Quantity,
    pub symbol: Symbol,
    pub price: Price,
    pub time_in_force: TimeInForce,
    pub display: Display,
    pub capacity: Capacity,
    pub int_mkt_sweep_eligibility: IntMktSweepEligibility,
    pub cross_type: CrossType,
    pub clt_order_id: CltOrderId,
    #[byteserde(replace( appendages.byte_len() ))]
    appendage_length: u16,
    #[byteserde(deplete(appendage_length))]
    pub appendages: EnterOrderAppendage,
}
impl EnterOrder {
    #[allow(clippy::too_many_arguments)]
    #[inline]
    pub fn new(
        user_ref_number: UserRefNumber,
        quantity: Quantity,
        symbol: Symbol,
        price: Price,
        time_in_force: TimeInForce,
        display: Display,
        capacity: Capacity,
        int_mkt_sweep_eligibility: IntMktSweepEligibility,
        cross_type: CrossType,
        clt_order_id: CltOrderId,
        appendages: EnterOrderAppendage,
    ) -> Self {
        Self {
            packet_type: PacketTypeEnterOrder::default(),
            user_ref_number,
            side: Side::buy(),
            quantity,
            symbol,
            price,
            time_in_force,
            display,
            capacity,
            int_mkt_sweep_eligibility,
            cross_type,
            clt_order_id,
            appendage_length: appendages.byte_len() as u16,
            appendages,
        }
    }
}

impl Default for EnterOrder {
    #[inline(always)]
    fn default() -> Self {
        let appendages = EnterOrderAppendage {
            firm: Some(TagValueElement::new(Firm::new(*b"ABCD"))),
            min_qty: Some(TagValueElement::new(MinQty::new(100))),
            ..Default::default()
        };
        Self {
            packet_type: PacketTypeEnterOrder::default(),
            user_ref_number: UserRefNumberGenerator::default().next().unwrap(),
            side: Side::buy(),
            quantity: Quantity::from(100),
            symbol: Symbol::from(b"DUMMY".as_slice()),
            price: Price::from(1.2345),
            time_in_force: TimeInForce::market_hours(),
            display: Display::visible(),
            capacity: Capacity::agency(),
            int_mkt_sweep_eligibility: IntMktSweepEligibility::eligible(),
            cross_type: CrossType::continuous_market(),
            clt_order_id: CltOrderIdIterator::default().next().unwrap(),
            appendage_length: appendages.byte_len() as u16,
            appendages,
        }
    }
}

impl CancelableOrder for EnterOrder {
    fn user_ref_number(&self) -> UserRefNumber {
        self.user_ref_number
    }
    fn quantity(&self) -> Quantity {
        self.quantity
    }
    fn cl_ord_id(&self) -> CltOrderId {
        self.clt_order_id
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
        let msg_inp = EnterOrder::default();

        let ser: ByteSerializerStack<128> = to_serializer_stack(&msg_inp).unwrap();
        info!("ser: {:#x}", ser);

        let msg_out: EnterOrder = from_serializer_stack(&ser).unwrap();

        info!("msg_inp: {:?}", msg_inp);
        info!("msg_out: {:?}", msg_out);
        assert_eq!(msg_out, msg_inp);
    }
}
