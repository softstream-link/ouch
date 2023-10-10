use crate::prelude::*;
use byteserde::prelude::*;
use byteserde_derive::{ByteDeserializeSlice, ByteSerializeStack, ByteSerializedLenOf};

use super::enter_order::EnterOrderAppendage;

// page 7 from https://nasdaqtrader.com/content/technicalsupport/specifications/TradingProducts/Ouch5.0.pdf
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
// ExpireTime
// TradeNow
// HandleInst
// GroupID
// SharesLocated

#[derive(ByteSerializeStack, ByteDeserializeSlice, ByteSerializedLenOf, PartialEq, Clone, Copy, Debug, Default)]
#[byteserde(peek(1, 1))] // peek(start, len) -> peek one byte after skipping one
pub struct ReplaceOrderAppendage {
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

impl From<&EnterOrderAppendage> for ReplaceOrderAppendage {
    #[inline(always)]
    fn from(appendages: &EnterOrderAppendage) -> Self {
        ReplaceOrderAppendage {
            min_qty: appendages.min_qty,
            customer_type: appendages.customer_type,
            max_floor: appendages.max_floor,
            price_type: appendages.price_type,
            peg_offset: appendages.peg_offset,
            discretion_price: appendages.discretion_price,
            discretion_price_type: appendages.discretion_price_type,
            discretion_peg_offset: appendages.discretion_peg_offset,
            post_only: appendages.post_only,
            random_reserves: appendages.random_reserves,
            expire_time: appendages.expire_time,
            trade_now: appendages.trade_now,
            handle_inst: appendages.handle_inst,
            group_id: appendages.group_id,
            shares_located: appendages.shares_located,
        }
    }
}

#[derive(ByteSerializeStack, ByteDeserializeSlice, ByteSerializedLenOf, PartialEq, Clone, Debug)]
#[byteserde(endian = "be")]
pub struct ReplaceOrder {
    packet_type: PacketTypeReplaceOrder,
    pub orig_user_ref_number: UserRefNumber,
    pub user_ref_number: UserRefNumber,
    pub quantity: Quantity,
    pub price: Price,
    pub time_in_force: TimeInForce,
    pub display: Display,
    pub int_mkt_sweep_eligibility: IntMktSweepEligibility,
    pub clt_order_id: CltOrderId,
    #[byteserde(replace( appendages.byte_len() ))]
    appendage_length: u16,
    #[byteserde(deplete(appendage_length))]
    pub appendages: ReplaceOrderAppendage,
}
impl CancelableOrder for ReplaceOrder {
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
impl From<&EnterOrder> for ReplaceOrder {
    fn from(enter_order: &EnterOrder) -> Self {
        let appendages: ReplaceOrderAppendage = (&enter_order.appendages).into();
        Self {
            packet_type: PacketTypeReplaceOrder::default(),
            orig_user_ref_number: enter_order.user_ref_number,
            user_ref_number: UserRefNumber::default(), // default place holder, has to be replaced
            quantity: enter_order.quantity,
            price: enter_order.price,
            time_in_force: enter_order.time_in_force,
            display: enter_order.display,
            int_mkt_sweep_eligibility: enter_order.int_mkt_sweep_eligibility,
            clt_order_id: CltOrderId::default(), // default place holder, has to be replaced
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
        let msg_inp = ReplaceOrder::from(&EnterOrder::default());

        let ser: ByteSerializerStack<128> = to_serializer_stack(&msg_inp).unwrap();
        info!("ser: {:#x}", ser);

        let msg_out: ReplaceOrder = from_serializer_stack(&ser).unwrap();

        info!("msg_inp: {:?}", msg_inp);
        info!("msg_out: {:?}", msg_out);
        assert_eq!(msg_out, msg_inp);
    }
}
