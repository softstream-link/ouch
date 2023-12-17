use crate::prelude::*;
use byteserde::prelude::*;
use byteserde_derive::{ByteDeserializeSlice, ByteSerializeStack, ByteSerializedLenOf};
use serde::{Deserialize, Serialize};

use super::_04_modify_order::ModifiableOrder;

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

#[derive(ByteSerializeStack, ByteDeserializeSlice, ByteSerializedLenOf, Serialize, Deserialize, PartialEq, Clone, Copy, Debug, Default)]
#[byteserde(peek(1, 1))] // peek(start, len) -> peek one byte after skipping one
pub struct EnterOrderAppendage {
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
    #[byteserde(eq(GroupId::tag_as_slice()))]
    pub group_id: Option<TagValueElement<GroupId>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[byteserde(eq(SharesLocated::tag_as_slice()))]
    pub shares_located: Option<TagValueElement<SharesLocated>>,
}

#[derive(ByteSerializeStack, ByteDeserializeSlice, ByteSerializedLenOf, Serialize, Deserialize, PartialEq, Clone, Debug)]
#[byteserde(endian = "be")]
#[serde(from = "EnterOrderJsonDesShadow")]
pub struct EnterOrder {
    #[serde(default, skip_serializing)]
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
    #[serde(skip)]
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
    /// Buy 100 shares of DUMMY at 1.2345
    #[inline(always)]
    fn default() -> Self {
        let appendages = EnterOrderAppendage {
            firm: Some(b"????".into()),
            min_qty: Some(0.into()),
            customer_type: Some(CustomerType::default().into()),
            max_floor: Some(0.into()),
            price_type: Some(PriceType::default().into()),
            peg_offset: Some((-1.1234).into()),
            discretion_price: Some(0.into()),
            discretion_price_type: Some(DiscretionPriceType::default().into()),
            discretion_peg_offset: Some((-1.1234).into()),
            post_only: Some(PostOnly::default().into()),
            random_reserves: Some(RandomReserves::default().into()),
            route: Some(b"????".into()),
            expire_time: Some(ExpireTime::default().into()),
            trade_now: Some(TradeNow::default().into()),
            handle_inst: Some(HandleInst::default().into()),
            group_id: Some(GroupId::default().into()),
            shares_located: Some(SharesLocated::default().into()),
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
impl ModifiableOrder for EnterOrder {
    fn quantity(&self) -> Quantity {
        self.quantity
    }
    fn side(&self) -> Side {
        self.side
    }
    fn user_ref_number(&self) -> UserRefNumber {
        self.user_ref_number
    }
}

#[derive(Deserialize)]
struct EnterOrderJsonDesShadow {
    user_ref_number: UserRefNumber,
    side: Side,
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
}
impl From<EnterOrderJsonDesShadow> for EnterOrder {
    fn from(shadow: EnterOrderJsonDesShadow) -> Self {
        Self {
            packet_type: PacketTypeEnterOrder::default(),
            user_ref_number: shadow.user_ref_number,
            side: shadow.side,
            quantity: shadow.quantity,
            symbol: shadow.symbol,
            price: shadow.price,
            time_in_force: shadow.time_in_force,
            display: shadow.display,
            capacity: shadow.capacity,
            int_mkt_sweep_eligibility: shadow.int_mkt_sweep_eligibility,
            cross_type: shadow.cross_type,
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
        setup::log::configure_compact(log::LevelFilter::Info);
        let msg_inp = EnterOrder::default();

        let ser: ByteSerializerStack<256> = to_serializer_stack(&msg_inp).unwrap();
        info!("ser: {:#x}", ser);

        let msg_out: EnterOrder = from_serializer_stack(&ser).unwrap();

        info!("msg_inp: {:?}", msg_inp);
        info!("msg_out: {:?}", msg_out);
        assert_eq!(msg_out, msg_inp);
    }

    #[test]
    fn test_msg_serde() {
        setup::log::configure_compact(log::LevelFilter::Info);
        let msg_inp = EnterOrder::default();
        // info!("msg_inp: {:?}", msg_inp);

        let json_out = to_string(&msg_inp).unwrap();
        let json_exp = r#"{"user_ref_number":1,"side":"BUY","quantity":100,"symbol":"DUMMY","price":1.2345,"time_in_force":"MARKET_HOURS","display":"VISIBLE","capacity":"AGENCY","int_mkt_sweep_eligibility":"ELIGIBLE","cross_type":"CONTINUOUS_MARKET","clt_order_id":"1","appendages":{"firm":"????","min_qty":0,"customer_type":"PORT_DEFAULT","max_floor":0,"price_type":"LIMIT","peg_offset":-1.1234,"discretion_price":0.0,"discretion_price_type":"LIMIT","discretion_peg_offset":-1.1234,"post_only":"NO","random_reserves":0,"route":"????","expire_time":0,"trade_now":"PORT_DEFAULT","handle_inst":"NO_INSTRUCTIONS","group_id":0,"shares_located":"NO"}}"#;
        info!("json_out: {}", json_out);

        if matches!(diff(&json_out, json_exp, ","), (dist, _) if dist != 0) {
            print_diff(&json_out, json_exp, ",");
            assert_eq!(json_out, json_exp);
        }

        let msg_out: EnterOrder = from_str(&json_out).unwrap();
        // info!("msg_out: {:?}", msg_out);
        assert_eq!(msg_out, msg_inp);
    }
}
