use super::_01_enter_order::EnterOrderAppendage;
use crate::prelude::*;
use byteserde::prelude::*;
use byteserde_derive::{ByteDeserializeSlice, ByteSerializeStack, ByteSerializedLenOf};
use serde::{Deserialize, Serialize};

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

#[derive(ByteSerializeStack, ByteDeserializeSlice, ByteSerializedLenOf, Serialize, Deserialize, PartialEq, Clone, Copy, Debug, Default)]
#[byteserde(peek(1, 1))] // peek(start, len) -> peek one byte after skipping one
pub struct ReplaceOrderAppendage {
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
    #[byteserde(eq(ExpireTimeSec::tag_as_slice()))]
    pub expire_time: Option<TagValueElement<ExpireTimeSec>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[byteserde(eq(TradeNow::tag_as_slice()))]
    pub trade_now: Option<TagValueElement<TradeNow>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[byteserde(eq(HandleInst::tag_as_slice()))]
    pub handle_inst: Option<TagValueElement<HandleInst>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[byteserde(eq(GroupId::tag_as_slice()))]
    pub group_id: Option<TagValueElement<GroupId>>,

    #[serde(skip_serializing_if = "Option::is_none")]
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

#[derive(ByteSerializeStack, ByteDeserializeSlice, ByteSerializedLenOf, Serialize, Deserialize, PartialEq, Clone, Debug)]
#[byteserde(endian = "be")]
#[serde(from = "ReplaceOrderJsonDesShadow")]
pub struct ReplaceOrder {
    #[serde(default, skip_serializing)]
    packet_type: PacketTypeReplaceOrder,
    pub orig_user_ref_number: UserRefNumber,
    pub user_ref_number: UserRefNumber,
    pub quantity: Quantity,
    pub price: Price,
    pub time_in_force: TimeInForce,
    pub display: Display,
    pub int_mkt_sweep_eligibility: IntMktSweepEligibility,
    pub clt_order_id: CltOrderId,
    #[serde(skip)]
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
    /// All fields will be copied from [EnterOrder] except for [ReplaceOrder::user_ref_number] and
    /// [ReplaceOrder::clt_order_id] for which a default value will be used.
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

#[derive(Deserialize)]
struct ReplaceOrderJsonDesShadow {
    orig_user_ref_number: UserRefNumber,
    user_ref_number: UserRefNumber,
    quantity: Quantity,
    price: Price,
    time_in_force: TimeInForce,
    display: Display,
    int_mkt_sweep_eligibility: IntMktSweepEligibility,
    clt_order_id: CltOrderId,
    appendages: ReplaceOrderAppendage,
}
impl From<ReplaceOrderJsonDesShadow> for ReplaceOrder {
    fn from(shadow: ReplaceOrderJsonDesShadow) -> Self {
        Self {
            packet_type: PacketTypeReplaceOrder::default(),
            orig_user_ref_number: shadow.orig_user_ref_number,
            user_ref_number: shadow.user_ref_number,
            quantity: shadow.quantity,
            price: shadow.price,
            time_in_force: shadow.time_in_force,
            display: shadow.display,
            int_mkt_sweep_eligibility: shadow.int_mkt_sweep_eligibility,
            clt_order_id: shadow.clt_order_id,
            appendage_length: shadow.appendages.byte_len() as u16,
            appendages: shadow.appendages,
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
        let msg_inp = ReplaceOrder::from(&EnterOrder::default());

        let ser: ByteSerializerStack<128> = to_serializer_stack(&msg_inp).unwrap();
        info!("ser: {:#x}", ser);

        let msg_out: ReplaceOrder = from_serializer_stack(&ser).unwrap();

        info!("msg_inp: {:?}", msg_inp);
        info!("msg_out: {:?}", msg_out);
        assert_eq!(msg_out, msg_inp);
    }

    #[test]
    fn test_msg_serde() {
        setup::log::configure_compact();

        let msg_inp = ReplaceOrder::from(&EnterOrder::default());
        // info!("msg_inp: {:?}", msg_inp);

        let json_out = to_string(&msg_inp).unwrap();
        info!("json_out: {}", json_out);

        let json_exp = r#"{"orig_user_ref_number":1,"user_ref_number":0,"quantity":100,"price":1.2345,"time_in_force":"MARKET_HOURS","display":"VISIBLE","int_mkt_sweep_eligibility":"ELIGIBLE","clt_order_id":"REPLACE_ME____","appendages":{"min_qty":0,"customer_type":" ","max_floor":0,"price_type":"L","peg_offset":-1.1234,"discretion_price":0.0,"discretion_price_type":"L","discretion_peg_offset":-1.1234,"post_only":"N","random_reserves":0,"expire_time":0,"trade_now":" ","handle_inst":" ","group_id":0,"shares_located":"N"}}"#;
        let (dist, _) = diff(&json_out, json_exp, "\n"); // pretty print the diff
        if dist != 0 {
            print_diff(&json_out, json_exp, "\n")
        }

        let msg_out: ReplaceOrder = from_str(&json_out).unwrap();

        // info!("msg_out: {:?}", msg_out);
        assert_eq!(msg_out, msg_inp);
    }
}
