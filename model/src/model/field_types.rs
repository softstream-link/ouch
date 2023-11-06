pub use super::optional_filed_types::*;

pub use aiq_strategy::AiqStrategy;
pub use broken_trade_reason::BrokenTradeReason;
pub use cancel_reason::CancelReason;
pub use cancel_reason_aiq::CancelReasonAiq;
pub use capacity::Capacity;
pub use clt_order_id::*;
pub use cross_type::CrossType;
pub use display::Display;
pub use event_code::EventCode;
pub use int_mkt_sweep_eligibility::IntMktSweepEligibility;
pub use liquidity_flag::LiquidityFlag;
pub use match_number::MatchNumber;
pub use order_reference_number::OrderReferenceNumber;
pub use order_reject_reason::RejectReason;
pub use order_restated_reason::RestatedReason;
pub use order_state::OrderState;
pub use ouch_packet_types::*;
pub use price::Price;
pub use qty::*;
pub use side::Side;
pub use string_ascii_fixed::*;
pub use time_in_force::TimeInForce;
pub use timestamp::Timestamp;
pub use user_ref::*;

use byteserde_derive::{ByteDeserializeSlice, ByteSerializeStack, ByteSerializedLenOf, ByteSerializedSizeOf};
use byteserde_types::{char_ascii, const_char_ascii, string_ascii_fixed, u16_tuple, u32_tuple, u64_tuple};
use serde::{Deserialize, Serialize};

// const char ascii
pub mod ouch_packet_types {
    use super::*;
    // TODO how will packet_type field play between soupbintcp and ouch?
    // inbound
    const_char_ascii!(PacketTypeEnterOrder, b'O', #[derive(ByteSerializeStack, ByteSerializedSizeOf, ByteSerializedLenOf, PartialEq, Clone, Copy)]);
    const_char_ascii!(PacketTypeReplaceOrder, b'U', #[derive(ByteSerializeStack, ByteSerializedSizeOf, ByteSerializedLenOf, PartialEq, Clone, Copy)]);
    const_char_ascii!(PacketTypeCancelOrder, b'X', #[derive(ByteSerializeStack, ByteSerializedSizeOf, ByteSerializedLenOf, PartialEq, Clone, Copy)]);
    const_char_ascii!(PacketTypeModifyOrder, b'M', #[derive(ByteSerializeStack, ByteSerializedSizeOf, ByteSerializedLenOf, PartialEq, Clone, Copy)]);
    const_char_ascii!(PacketTypeAccountQueryRequest, b'Q', #[derive(ByteSerializeStack, ByteSerializedSizeOf, ByteSerializedLenOf, PartialEq, Clone, Copy)]);

    // outbound
    const_char_ascii!(PacketTypeSystemEvent, b'S', #[derive(ByteSerializeStack, ByteSerializedSizeOf, ByteSerializedLenOf, PartialEq, Clone, Copy)]);
    const_char_ascii!(PacketTypeOrderAccepted, b'A', #[derive(ByteSerializeStack, ByteSerializedSizeOf, ByteSerializedLenOf, PartialEq, Clone, Copy)]);
    const_char_ascii!(PacketTypeOrderReplaced, b'U', #[derive(ByteSerializeStack, ByteSerializedSizeOf, ByteSerializedLenOf, PartialEq, Clone, Copy)]);
    const_char_ascii!(PacketTypeOrderCanceled, b'C', #[derive(ByteSerializeStack, ByteSerializedSizeOf, ByteSerializedLenOf, PartialEq, Clone, Copy)]);
    const_char_ascii!(PacketTypeOrderAiqCanceled, b'D', #[derive(ByteSerializeStack, ByteSerializedSizeOf, ByteSerializedLenOf, PartialEq, Clone, Copy)]);
    const_char_ascii!(PacketTypeOrderExecuted, b'E', #[derive(ByteSerializeStack, ByteSerializedSizeOf, ByteSerializedLenOf, PartialEq, Clone, Copy)]);
    const_char_ascii!(PacketTypeBrokenTrade, b'B', #[derive(ByteSerializeStack, ByteSerializedSizeOf, ByteSerializedLenOf, PartialEq, Clone, Copy)]);
    const_char_ascii!(PacketTypeOrderRejected, b'J', #[derive(ByteSerializeStack, ByteSerializedSizeOf, ByteSerializedLenOf, PartialEq, Clone, Copy)]);
    const_char_ascii!(PacketTypeCancelPending, b'P', #[derive(ByteSerializeStack, ByteSerializedSizeOf, ByteSerializedLenOf, PartialEq, Clone, Copy)]);
    const_char_ascii!(PacketTypeCancelReject, b'I', #[derive(ByteSerializeStack, ByteSerializedSizeOf, ByteSerializedLenOf, PartialEq, Clone, Copy)]);
    const_char_ascii!(PacketTypePriorityUpdate, b'T', #[derive(ByteSerializeStack, ByteSerializedSizeOf, ByteSerializedLenOf, PartialEq, Clone, Copy)]);
    const_char_ascii!(PacketTypeOrderModified, b'M', #[derive(ByteSerializeStack, ByteSerializedSizeOf, ByteSerializedLenOf, PartialEq, Clone, Copy)]);
    const_char_ascii!(PacketTypeOrderRestated, b'R', #[derive(ByteSerializeStack, ByteSerializedSizeOf, ByteSerializedLenOf, PartialEq, Clone, Copy)]);
    const_char_ascii!(PacketTypeAccountQueryResponse, b'Q', #[derive(ByteSerializeStack, ByteSerializedSizeOf, ByteSerializedLenOf, PartialEq, Clone, Copy)]);
}
// fixed ascii strings
pub mod string_ascii_fixed {
    use super::*;
    string_ascii_fixed!(Symbol, 9, b' ', false, #[derive(ByteSerializeStack, ByteDeserializeSlice, ByteSerializedSizeOf, ByteSerializedLenOf, PartialEq, Clone, Copy)]);
}

pub mod clt_order_id {
    use super::*;
    string_ascii_fixed!(CltOrderId, 14, b' ', false, #[derive(ByteSerializeStack, ByteDeserializeSlice, ByteSerializedSizeOf, ByteSerializedLenOf, PartialEq, Clone, Copy)]);
    impl Default for CltOrderId {
        fn default() -> Self {
            Self::new(b"REPLACE_ME____".to_owned())
        }
    }
    impl From<u64> for CltOrderId {
        fn from(id: u64) -> Self {
            Self::from(format!("{}", id).as_str().as_bytes())
        }
    }

    #[derive(Default)]
    pub struct CltOrderIdIterator {
        last: u64,
    }
    impl Iterator for CltOrderIdIterator {
        type Item = CltOrderId;
        fn next(&mut self) -> Option<Self::Item> {
            self.last += 1;
            Some(CltOrderId::from(self.last))
        }
    }
    #[cfg(test)]

    mod test {
        use links_core::unittest::setup;
        use log::info;

        use super::*;

        #[test]
        fn test_clt_order_id_iterator() {
            setup::log::configure();
            let mut iter = CltOrderIdIterator { last: 0 };
            let next = iter.next().unwrap();
            info!("next: {:?}", next);
            assert_eq!(next, CltOrderId::from(1));
            let next = iter.next().unwrap();
            info!("next: {:?}", next);
            assert_eq!(next, CltOrderId::from(2));
            let next = iter.next().unwrap();
            info!("next: {:?}", next);
            assert_eq!(next, CltOrderId::from(3));
        }
    }
}

// char ascii
pub mod side {
    use super::*;
    use links_core::core::macros::short_type_name;
    use serde::{Deserializer, Serializer};
    char_ascii!(Side, #[derive(ByteSerializeStack, ByteDeserializeSlice, ByteSerializedSizeOf, ByteSerializedLenOf, PartialEq, Clone, Copy)]);
    #[rustfmt::skip]
    impl Side {
        pub fn buy() -> Self { Side(b'B') }
        pub fn sell() -> Self { Side(b'S') }
        pub fn sell_short() -> Self { Side(b'T') }
        pub fn sell_short_exempt() -> Self { Side(b'U') }
        pub fn is_buy(&self) -> bool { self.0 == b'B' }
        pub fn is_sell(&self) -> bool { self.0 == b'S' }
        pub fn is_sell_short(&self) -> bool { self.0 == b'T' }
        pub fn is_sell_short_exempt(&self) -> bool { self.0 == b'U' }
    }
    impl Serialize for Side {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: Serializer {
            if self.is_buy() {
                serializer.serialize_str("BUY")
            } else if self.is_sell() {
                serializer.serialize_str("SELL")
            } else if self.is_sell_short() {
                serializer.serialize_str("SELL_SHORT")
            } else if self.is_sell_short_exempt() {
                serializer.serialize_str("SELL_SHORT_EXCEPT")
            } else {
                serializer.serialize_str("UNKNOWN")
            }
        }
    }
    impl<'de> Deserialize<'de> for Side {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where D: Deserializer<'de> {
            let value = String::deserialize(deserializer)?.to_uppercase();
            match value.as_str() {
                "BUY" | "B" => Ok(Self::buy()),
                "SELL" | "S" => Ok(Self::sell()),
                "SELL_SHORT" | "T" => Ok(Self::sell_short()),
                "SELL_SHORT_EXCEPT" | "U" => Ok(Self::sell_short_exempt()),
                _ => panic!("Unknown value for {}: {}", short_type_name::<Self>(), value),
            }
        }
    }
}

pub mod time_in_force {
    use super::*;
    char_ascii!(TimeInForce, true, #[derive(ByteSerializeStack, ByteDeserializeSlice, ByteSerializedSizeOf, ByteSerializedLenOf, PartialEq, Clone, Copy)]);
    #[rustfmt::skip]
    impl TimeInForce{
        pub fn market_hours() -> Self { TimeInForce(b'0') }
        pub fn immediate_or_cancel() -> Self { TimeInForce(b'3') }
        pub fn good_till_extended_hours() -> Self { TimeInForce(b'5') }
        pub fn good_till_triggered() -> Self { TimeInForce(b'6') }
        pub fn after_hours() -> Self { TimeInForce(b'E') }
        pub fn is_market_hours(tif: &TimeInForce) -> bool { Self::market_hours() == *tif }
        pub fn is_immediate_or_cancel(tif: &TimeInForce) -> bool { Self::immediate_or_cancel() == *tif }
        pub fn is_good_till_extended_hours(tif: &TimeInForce) -> bool { Self::good_till_extended_hours() == *tif }
        pub fn is_good_till_triggered(tif: &TimeInForce) -> bool { Self::good_till_triggered() == *tif }
        pub fn is_after_hours(tif: &TimeInForce) -> bool { Self::after_hours() == *tif }
    }
}

pub mod display {
    use super::*;
    char_ascii!(Display, true, #[derive(ByteSerializeStack, ByteDeserializeSlice, ByteSerializedSizeOf, ByteSerializedLenOf, PartialEq, Clone, Copy)]);
    #[rustfmt::skip]
    impl Display {
        pub fn visible() -> Self { Display(b'Y') }
        pub fn hidden() -> Self { Display(b'N') }
        pub fn attributable() -> Self { Display(b'A') }
        pub fn conformant() -> Self { Display(b'Z') }
        pub fn is_visible(display: &Display) -> bool { Self::visible() == *display }
        pub fn is_hidden(display: &Display) -> bool { Self::hidden() == *display }
        pub fn is_attributable(display: &Display) -> bool { Self::attributable() == *display }
        pub fn is_conformant(display: &Display) -> bool { Self::conformant() == *display }
    }
}

pub mod capacity {
    use super::*;
    char_ascii!(Capacity, true, #[derive(ByteSerializeStack, ByteDeserializeSlice, ByteSerializedSizeOf, ByteSerializedLenOf, PartialEq, Clone, Copy)]);
    #[rustfmt::skip]
    impl Capacity{
        pub fn agency() -> Self { Capacity(b'A') }
        pub fn principal() -> Self { Capacity(b'P') }
        pub fn riskless_principal() -> Self { Capacity(b'R') }
        pub fn other() -> Self { Capacity(b'O') }
        pub fn is_agency(capacity: &Capacity) -> bool { Self::agency() == *capacity }
        pub fn is_principal(capacity: &Capacity) -> bool { Self::principal() == *capacity }
        pub fn is_riskless_principal(capacity: &Capacity) -> bool { Self::riskless_principal() == *capacity }
        pub fn is_other(capacity: &Capacity) -> bool { Self::other() == *capacity }
    }
}

pub mod int_mkt_sweep_eligibility {
    use super::*;
    char_ascii!(IntMktSweepEligibility, true, #[derive(ByteSerializeStack, ByteDeserializeSlice, ByteSerializedSizeOf, ByteSerializedLenOf, PartialEq, Clone, Copy)]);
    #[rustfmt::skip]
    impl IntMktSweepEligibility{
        pub fn eligible() -> Self { IntMktSweepEligibility(b'Y') }
        pub fn not_eligible() -> Self { IntMktSweepEligibility(b'N') }
        pub fn is_eligible(eligibility: &IntMktSweepEligibility) -> bool { Self::eligible() == *eligibility }
        pub fn is_not_eligible(eligibility: &IntMktSweepEligibility) -> bool { Self::not_eligible() == *eligibility }
    }
}

pub mod cross_type {
    use super::*;
    char_ascii!(CrossType, true, #[derive(ByteSerializeStack, ByteDeserializeSlice, ByteSerializedSizeOf, ByteSerializedLenOf, PartialEq, Clone, Copy)]);
    #[rustfmt::skip]
    impl CrossType{
        pub fn continuous_market() -> Self { CrossType(b'N') }
        pub fn opening_cross() -> Self { CrossType(b'O') }
        pub fn closing_cross() -> Self { CrossType(b'C') }
        pub fn halt_ipo() -> Self { CrossType(b'H') }
        pub fn supplemental() -> Self { CrossType(b'S') }
        pub fn retail() -> Self { CrossType(b'R') }
        pub fn extended_life() -> Self { CrossType(b'E') }
        pub fn after_hours_close() -> Self { CrossType(b'A') }
        pub fn is_continuous_market(cross_type: &CrossType) -> bool { Self::continuous_market() == *cross_type }
        pub fn is_opening_cross(cross_type: &CrossType) -> bool { Self::opening_cross() == *cross_type }
        pub fn is_closing_cross(cross_type: &CrossType) -> bool { Self::closing_cross() == *cross_type }
        pub fn is_halt_ipo(cross_type: &CrossType) -> bool { Self::halt_ipo() == *cross_type }
        pub fn is_supplemental(cross_type: &CrossType) -> bool { Self::supplemental() == *cross_type }
        pub fn is_retail(cross_type: &CrossType) -> bool { Self::retail() == *cross_type }
        pub fn is_extended_life(cross_type: &CrossType) -> bool { Self::extended_life() == *cross_type }
        pub fn is_after_hours_close(cross_type: &CrossType) -> bool { Self::after_hours_close() == *cross_type }
    }
}

pub mod event_code {
    use super::*;
    char_ascii!(EventCode, #[derive(ByteSerializeStack, ByteDeserializeSlice, ByteSerializedSizeOf, ByteSerializedLenOf, PartialEq, Clone, Copy)]);
    #[rustfmt::skip]
    impl EventCode{
        pub fn start_of_day() -> Self { EventCode(b'S') }
        pub fn end_of_day() -> Self { EventCode(b'E') }
        pub fn is_startofday(side: &EventCode) -> bool { Self::start_of_day() == *side }
        pub fn is_endofday(side: &EventCode) -> bool { Self::end_of_day() == *side }
    }
}

pub mod order_state {
    use super::*;
    char_ascii!(OrderState, #[derive(ByteSerializeStack, ByteDeserializeSlice, ByteSerializedSizeOf, ByteSerializedLenOf, PartialEq, Clone, Copy)]);
    #[rustfmt::skip]
    impl OrderState{
        pub fn live() -> Self { OrderState(b'L') }
        pub fn dead() -> Self { OrderState(b'D') }
        pub fn is_live(side: &OrderState) -> bool { Self::live() == *side }
        pub fn is_dead(side: &OrderState) -> bool { Self::dead() == *side }
    }
}
// numerics
pub mod qty {
    use super::*;
    u32_tuple!(Quantity, "be", #[derive(ByteSerializeStack, ByteDeserializeSlice, ByteSerializedSizeOf, ByteSerializedLenOf, Serialize, Deserialize, PartialEq, Clone, Copy, Debug, Default)]);
}

pub mod user_ref {
    use super::*;

    u32_tuple!(UserRefNumber, "be", #[derive(ByteSerializeStack, ByteDeserializeSlice, ByteSerializedSizeOf, ByteSerializedLenOf, Serialize, Deserialize, PartialEq, Clone, Copy, Debug, Default)]);
    #[derive(Default)]
    pub struct UserRefNumberGenerator {
        last: u32,
    }
    impl Iterator for UserRefNumberGenerator {
        type Item = UserRefNumber;
        fn next(&mut self) -> Option<Self::Item> {
            if self.last == u32::MAX {
                None
            } else {
                self.last += 1;
                Some(UserRefNumber::new(self.last))
            }
        }
    }
    #[cfg(test)]
    mod test {
        use super::*;
        use links_core::unittest::setup;
        use log::info;

        #[test]
        fn test_user_ref_number_iterator() {
            setup::log::configure();

            let mut iter = UserRefNumberGenerator::default();
            let next = iter.next().unwrap();
            info!("next: {:?}", next);
            assert_eq!(next, UserRefNumber::new(1));
            let next = iter.next().unwrap();
            info!("next: {:?}", next);
            assert_eq!(next, UserRefNumber::new(2));
        }
    }
}

pub mod price {
    use super::*;
    use links_core::core::macros::short_type_name;
    use std::fmt::Debug;

    u64_tuple!(Price, "be", #[derive(ByteSerializeStack, ByteDeserializeSlice, ByteSerializedSizeOf, ByteSerializedLenOf, PartialEq, Clone, Copy, Default)]);
    pub const PRICE_SCALE: f32 = 10000.0;
    impl From<f32> for Price {
        fn from(f: f32) -> Self {
            debug_assert!(f >= 0.0, "from: {} must be positive to create struct of {} type", f, short_type_name::<Price>());
            Price((f * PRICE_SCALE) as u64)
        }
    }
    impl From<&Price> for f32 {
        fn from(p: &Price) -> Self {
            p.0 as f32 / PRICE_SCALE
        }
    }
    impl Serialize for Price {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: serde::Serializer {
            serializer.serialize_f32(f32::from(self))
        }
    }
    impl<'de> Deserialize<'de> for Price {
        fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
            let f = f32::deserialize(deserializer)?;
            Ok(f.into())
        }
    }
    impl Debug for Price {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.debug_tuple(short_type_name::<Price>()).field(&f32::from(self)).finish()
        }
    }

    #[cfg(test)]
    mod test {
        use crate::prelude::*;
        use links_core::unittest::setup;
        use log::info;
        use serde_json::{from_str, to_string};

        #[test]
        fn test_price() {
            setup::log::configure_compact();
            let msg_in = Price::from(1.1234);
            info!("msg_in: {:?}", msg_in);

            let json_out = to_string(&msg_in).unwrap();
            info!("json_out: {}", json_out);

            let msg_out: Price = from_str(&json_out).unwrap();
            info!("msg_out: {:?}", msg_out);
            assert_eq!(msg_in, msg_out);
        }
        #[test]
        #[should_panic]
        fn test_price_fail() {
            let _ = Price::from(-1.1234);
        }
    }
}

pub mod timestamp {
    use super::*;
    use chrono::{DateTime, Local, NaiveDateTime, Utc};

    #[rustfmt::skip]
    u64_tuple!(Timestamp, "be", #[derive(ByteSerializeStack, ByteDeserializeSlice, ByteSerializedSizeOf, ByteSerializedLenOf, PartialEq, Debug, Clone, Copy)]);
    impl From<DateTime<Local>> for Timestamp {
        /// Converts into nanoseconds from last midnight of a given [`DateTime<Local>`] and into a [Timestamp]
        fn from(dt: DateTime<Local>) -> Self {
            let naive_now = dt.naive_local();
            Timestamp::from(naive_now)
        }
    }
    impl From<DateTime<Utc>> for Timestamp {
        fn from(dt: DateTime<Utc>) -> Self {
            let naive_now = dt.naive_utc();
            Timestamp::from(naive_now)
        }
    }
    impl From<NaiveDateTime> for Timestamp {
        fn from(dt: NaiveDateTime) -> Self {
            let last_midnight = dt.date().and_hms_opt(0, 0, 0).unwrap();
            let duration = dt.signed_duration_since(last_midnight).to_std().unwrap();
            let nanos_since_last_midnight = duration.as_nanos() as u64;
            Timestamp(nanos_since_last_midnight)
        }
    }
    impl Default for Timestamp {
        fn default() -> Self {
            Timestamp::from(Local::now())
        }
    }

    #[cfg(test)]

    mod test {
        use links_core::unittest::setup;
        use log::info;

        use super::*;
        #[test]
        fn test_timestamp() {
            setup::log::configure();

            // default
            let timestamp = Timestamp::default();
            info!("default timestamp: {:?}", timestamp);

            // from an arbitrary date
            let nanos_shift = 1000;
            let nanos_shift_past_midnight = Local::now().date_naive().and_hms_nano_opt(0, 0, 0, nanos_shift).unwrap();

            info!("one_th_nano_past_midnight: {:?}", nanos_shift_past_midnight);
            let timestamp = Timestamp::from(nanos_shift_past_midnight);
            info!("nanos_shift: {}, timestamp: {:?}", nanos_shift, timestamp);
            assert_eq!(timestamp, Timestamp(nanos_shift as u64));
        }
    }
}

pub mod order_reference_number {
    use super::*;
    #[rustfmt::skip]
    u64_tuple!(OrderReferenceNumber, "be", #[derive(ByteSerializeStack, ByteDeserializeSlice, ByteSerializedSizeOf, ByteSerializedLenOf, PartialEq, Clone, Copy, Debug, Default)]);

    #[derive(Default)]
    pub struct OrderReferenceNumberIterator {
        last: u64,
    }
    impl Iterator for OrderReferenceNumberIterator {
        type Item = OrderReferenceNumber;
        fn next(&mut self) -> Option<Self::Item> {
            if self.last == u64::MAX {
                None
            } else {
                self.last += 1;
                Some(OrderReferenceNumber::new(self.last))
            }
        }
    }

    #[cfg(test)]

    mod test {
        use log::info;

        use super::*;
        use links_core::unittest::setup;
        #[test]
        fn test_order_ref_number_iterator() {
            setup::log::configure();

            let mut iter = OrderReferenceNumberIterator::default();
            let next = iter.next().unwrap();
            info!("next: {:?}", next);
            assert_eq!(next, OrderReferenceNumber::new(1));
            let next = iter.next().unwrap();
            info!("next: {:?}", next);
            assert_eq!(next, OrderReferenceNumber::new(2));
        }
    }
}

pub mod cancel_reason {
    use super::*;

    #[rustfmt::skip]
    char_ascii!(CancelReason, #[derive(ByteSerializeStack, ByteDeserializeSlice, ByteSerializedSizeOf, ByteSerializedLenOf, PartialEq, Clone, Copy)]);
    #[rustfmt::skip]
    impl CancelReason {
        pub fn reg_restriction() -> Self{ CancelReason(b'D') }
        pub fn closed() -> Self{ CancelReason(b'E') }
        pub fn post_only_cancel_nms() -> Self{ CancelReason(b'F') }
        pub fn post_only_cancel_displayed() -> Self{ CancelReason(b'G') }
        pub fn halted() -> Self{ CancelReason(b'H') }
        pub fn immediate_or_cancel() -> Self{ CancelReason(b'I') }
        pub fn market_collars() -> Self{ CancelReason(b'K') }
        pub fn self_match_prevention() -> Self{ CancelReason(b'Q') }
        pub fn supervisory() -> Self{ CancelReason(b'S') }
        pub fn timeout() -> Self{ CancelReason(b'T') }
        pub fn user_requested() -> Self{ CancelReason(b'U') }
        pub fn open_protection() -> Self{ CancelReason(b'X') }
        pub fn system_cancel() -> Self{ CancelReason(b'Z') }
        pub fn exceeds_allowable_shares() -> Self{ CancelReason(b'e') }
        pub fn is_reg_restriction(reason: &CancelReason) -> bool{ Self::reg_restriction() == *reason }
        pub fn is_closed(reason: &CancelReason) -> bool{ Self::closed() == *reason }
        pub fn is_post_only_cancel_nms(reason: &CancelReason) -> bool{ Self::post_only_cancel_nms() == *reason }
        pub fn is_post_only_cancel_displayed(reason: &CancelReason) -> bool{ Self::post_only_cancel_displayed() == *reason }
        pub fn is_halted(reason: &CancelReason) -> bool{ Self::halted() == *reason }
        pub fn is_immediate_or_cancel(reason: &CancelReason) -> bool{ Self::immediate_or_cancel() == *reason }
        pub fn is_market_collars(reason: &CancelReason) -> bool{ Self::market_collars() == *reason }
        pub fn is_self_match_prevention(reason: &CancelReason) -> bool{ Self::self_match_prevention() == *reason }
        pub fn is_supervisory(reason: &CancelReason) -> bool{ Self::supervisory() == *reason }
        pub fn is_timeout(reason: &CancelReason) -> bool{ Self::timeout() == *reason }
        pub fn is_user_requested(reason: &CancelReason) -> bool{ Self::user_requested() == *reason }
        pub fn is_open_protection(reason: &CancelReason) -> bool{ Self::open_protection() == *reason }
        pub fn is_system_cancel(reason: &CancelReason) -> bool{ Self::system_cancel() == *reason }
        pub fn is_exceeds_allowable_shares(reason: &CancelReason) -> bool{ Self::exceeds_allowable_shares() == *reason }
    }
}

pub mod cancel_reason_aiq {
    use super::*;

    #[rustfmt::skip]
    const_char_ascii!(CancelReasonAiq, b'Q', #[derive(ByteSerializeStack, ByteSerializedSizeOf, ByteSerializedLenOf, PartialEq, Clone, Copy)]);
}

pub mod liquidity_flag {
    use super::*;

    #[rustfmt::skip]
    char_ascii!(LiquidityFlag, #[derive(ByteSerializeStack, ByteDeserializeSlice, ByteSerializedSizeOf, ByteSerializedLenOf, PartialEq, Clone, Copy)]);
    #[rustfmt::skip]
    impl LiquidityFlag {
        pub fn added() -> Self{ LiquidityFlag(b'A') }
        pub fn closing_cross() -> Self{ LiquidityFlag(b'C') }
        pub fn retail_designated_that_added_display_liq() -> Self{ LiquidityFlag(b'e') }
        pub fn halt_ipo_cross() -> Self{ LiquidityFlag(b'H') }
        pub fn after_hours_closing_cross() -> Self{ LiquidityFlag(b'i') }
        pub fn non_display_adding_liq() -> Self{ LiquidityFlag(b'J') }
        pub fn rpi_order_provides_liq() -> Self{ LiquidityFlag(b'j') }
        pub fn added_liq_via_midpoint_order() -> Self{ LiquidityFlag(b'k') }
        pub fn halt_cross() -> Self{ LiquidityFlag(b'K') }
        pub fn closing_cross_imbalance() -> Self{ LiquidityFlag(b'L') }
        pub fn opening_cross_imbalance() -> Self{ LiquidityFlag(b'M') }
        pub fn removed_liq_at_midpoint() -> Self{ LiquidityFlag(b'm') }
        pub fn passing_midpoint_execution() -> Self{ LiquidityFlag(b'N') }
        pub fn midpoint_extended_life_order() -> Self{ LiquidityFlag(b'n') }
        pub fn opening_cross() -> Self{ LiquidityFlag(b'O') }
        pub fn removed_price_improving_non_display_liq() -> Self{ LiquidityFlag(b'p') }
        pub fn rmo_retail_order_removes_non_rpi_midpoint_liq() -> Self{ LiquidityFlag(b'q') }
        pub fn removed() -> Self{ LiquidityFlag(b'R') }
        pub fn retail_order_removes_rpi_liq() -> Self{ LiquidityFlag(b'r') }
        pub fn retain_order_removes_price_improving_non_display_liq_not_rpi_liq() -> Self{ LiquidityFlag(b't') }
        pub fn supplemental_order_execution() -> Self{ LiquidityFlag(b'0') }
        pub fn displayed_liq_adding_order_improves_nnbo() -> Self{ LiquidityFlag(b'7') }
        pub fn displayed_liq_adding_order_sets_qbbo() -> Self{ LiquidityFlag(b'8') }
        pub fn rpi_order_provides_liq_no_rpii() -> Self{ LiquidityFlag(b'1') }
    }
}

pub mod aiq_strategy {
    use super::*;

    char_ascii!(AiqStrategy, #[derive(ByteSerializeStack, ByteDeserializeSlice, ByteSerializedSizeOf, ByteSerializedLenOf, PartialEq, Clone, Copy)]);
    impl Default for AiqStrategy {
        fn default() -> Self {
            AiqStrategy(b'?') // specification does not list valid values
        }
    }
}

pub mod match_number {
    use super::*;

    u64_tuple!(MatchNumber, "be", #[derive(ByteSerializeStack, ByteDeserializeSlice, ByteSerializedSizeOf, ByteSerializedLenOf, PartialEq, Clone, Copy, Debug, Default)]);
    #[derive(Default)]
    pub struct MatchNumberIterator {
        last: u64,
    }
    impl Iterator for MatchNumberIterator {
        type Item = MatchNumber;
        fn next(&mut self) -> Option<Self::Item> {
            self.last += 1;
            Some(MatchNumber::from(self.last))
        }
    }
}

pub mod broken_trade_reason {
    use super::*;

    char_ascii!(BrokenTradeReason, #[derive(ByteSerializeStack, ByteDeserializeSlice, ByteSerializedSizeOf, ByteSerializedLenOf, PartialEq, Clone, Copy)]);
    #[rustfmt::skip]
    impl BrokenTradeReason {
        pub fn erroneous() -> Self{ BrokenTradeReason(b'E') }
        pub fn consent() -> Self{ BrokenTradeReason(b'C') }
        pub fn supervisory() -> Self{ BrokenTradeReason(b'S') }
        pub fn external() -> Self{ BrokenTradeReason(b'X') }
        pub fn is_erroneous(reason: &BrokenTradeReason) -> bool{ Self::erroneous() == *reason }
        pub fn is_consent(reason: &BrokenTradeReason) -> bool{ Self::consent() == *reason }
        pub fn is_supervisory(reason: &BrokenTradeReason) -> bool{ Self::supervisory() == *reason }
        pub fn is_external(reason: &BrokenTradeReason) -> bool{ Self::external() == *reason }
    }
}

pub mod order_reject_reason {
    use super::*;

    u16_tuple!(RejectReason, "be", #[derive(ByteSerializeStack, ByteDeserializeSlice, ByteSerializedSizeOf, ByteSerializedLenOf, PartialEq, Clone, Copy, Debug, Default)]);
    #[rustfmt::skip]
    impl RejectReason{
        pub fn quote_unavailable() -> Self{ RejectReason(0x01) }
        pub fn destination_closed() -> Self{ RejectReason(0x02) }
        pub fn invalid_display() -> Self{ RejectReason(0x03) }
        pub fn invalid_max_floor() -> Self{ RejectReason(0x04) }
        pub fn invalid_peg_type() -> Self{ RejectReason(0x05) }
        pub fn fat_finger() -> Self{ RejectReason(0x06) }
        pub fn halted() -> Self { RejectReason(0x07) }
        pub fn iso_not_allowed() -> Self { RejectReason(0x08) } 
        pub fn invalid_side() -> Self { RejectReason(0x09) } 
        pub fn processing_error() -> Self { RejectReason(0x0A) } 
        pub fn cancel_pending() -> Self { RejectReason(0x0B) } 
        pub fn firm_not_authorized() -> Self { RejectReason(0x0C) } 
        pub fn invalid_min_quantity() -> Self { RejectReason(0x0D) } 
        pub fn no_closing_reference_price() -> Self { RejectReason(0x0E) } 
        pub fn other() -> Self { RejectReason(0x0F) } 
        pub fn cancel_not_allowed() -> Self { RejectReason(0x10) } 
        pub fn pegging_not_allowed() -> Self { RejectReason(0x11) } 
        pub fn crossed_market() -> Self { RejectReason(0x12) } 
        pub fn invalid_quantity() -> Self { RejectReason(0x13) } 
        pub fn invalid_cross_order() -> Self { RejectReason(0x14) } 
        pub fn replace_not_allowed() -> Self { RejectReason(0x15) } 
        pub fn routing_not_allowed() -> Self { RejectReason(0x16) } 
        pub fn invalid_symbol() -> Self { RejectReason(0x17) } 
        pub fn test() -> Self { RejectReason(0x18) } 
        pub fn late_loc_too_aggressive() -> Self { RejectReason(0x19) } 
        pub fn retail_not_allowed() -> Self { RejectReason(0x1A) } 
        pub fn invalid_midpoint_post_only_price() -> Self { RejectReason(0x1B) } 
        pub fn invalid_destination() -> Self { RejectReason(0x1C) } 
        pub fn invalid_price() -> Self { RejectReason(0x1D) } 
        pub fn shares_exceed_threshold() -> Self { RejectReason(0x1E) } 
        pub fn exceeds_maximum_allowed_notional_value() -> Self { RejectReason(0x1F) } 
        pub fn risk_aggregate_exposure_exceeded() -> Self { RejectReason(0x20) } 
        pub fn risk_market_impact() -> Self { RejectReason(0x21) } 
        pub fn risk_restricted_stock() -> Self { RejectReason(0x22) } 
        pub fn risk_short_sell_restricted() -> Self { RejectReason(0x23) }
        pub fn risk_order_type_restricted() -> Self { RejectReason(0x24) }
        pub fn risk_exceeds_adv_limit() -> Self { RejectReason(0x25) }
        pub fn risk_fat_finger() -> Self { RejectReason(0x26) }
        pub fn risk_locate_required() -> Self { RejectReason(0x27) }
        pub fn risk_symbol_message_rate_restriction() -> Self { RejectReason(0x28) }
        pub fn risk_port_message_rate_restriction() -> Self { RejectReason(0x29) }
        pub fn risk_duplicate_message_rate_restriction() -> Self { RejectReason(0x2A) }
    }
}

pub mod order_restated_reason {
    use super::*;

    char_ascii!(RestatedReason, #[derive(ByteSerializeStack, ByteDeserializeSlice, ByteSerializedSizeOf, ByteSerializedLenOf, PartialEq, Clone, Copy, Default)]);

    #[rustfmt::skip]
    impl RestatedReason{
        pub fn refresh_of_display() -> Self { RestatedReason(b'R') }
        pub fn update_of_displayed_price() -> Self { RestatedReason(b'P') }
        pub fn is_refresh_of_display(reason: &RestatedReason) -> bool { Self::refresh_of_display() == *reason }
        pub fn is_update_of_displayed_price(reason: &RestatedReason) -> bool { Self::update_of_displayed_price() == *reason }
    }
}
