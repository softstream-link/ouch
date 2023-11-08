pub use super::optional_filed_types::*;

pub use aiq_strategy::AiqStrategy;
pub use broken_trade_reason::BrokenTradeReason;
pub use cancel_reason::CancelReason;
pub use cancel_reason_aiq::CancelAiqReason;
pub use capacity::Capacity;
pub use clt_order_id::*;
pub use cross_type::CrossType;
pub use display::Display;
pub use event_code::EventCode;
pub use int_mkt_sweep_eligibility::IntMktSweepEligibility;
pub use liquidity_flag::LiquidityFlag;
pub use match_number::MatchNumber;
pub use order_reference_number::OrderReferenceNumber;
pub use order_reject_reason::OrderRejectReason;
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
use links_core::core::macros::short_type_name;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

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
    char_ascii!(TimeInForce, #[derive(ByteSerializeStack, ByteDeserializeSlice, ByteSerializedSizeOf, ByteSerializedLenOf, PartialEq, Clone, Copy)]);
    #[rustfmt::skip]
    impl TimeInForce{
        pub fn market_hours() -> Self { TimeInForce(b'0') }
        pub fn immediate_or_cancel() -> Self { TimeInForce(b'3') }
        pub fn good_till_extended_hours() -> Self { TimeInForce(b'5') }
        pub fn good_till_triggered() -> Self { TimeInForce(b'6') }
        pub fn after_hours() -> Self { TimeInForce(b'E') }
        pub fn is_market_hours(&self) -> bool { self.0 == b'0' }
        pub fn is_immediate_or_cancel(&self) -> bool { self.0 == b'3' }
        pub fn is_good_till_extended_hours(&self) -> bool { self.0 == b'5' }
        pub fn is_good_till_triggered(&self) -> bool { self.0 == b'6' }
        pub fn is_after_hours(&self) -> bool { self.0 == b'E' }
    }
    impl Serialize for TimeInForce {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: Serializer {
            if self.is_market_hours() {
                serializer.serialize_str("MARKET_HOURS")
            } else if self.is_immediate_or_cancel() {
                serializer.serialize_str("IMMEDIATE_OR_CANCEL")
            } else if self.is_good_till_extended_hours() {
                serializer.serialize_str("GOOD_TILL_EXTENDED_HOURS")
            } else if self.is_good_till_triggered() {
                serializer.serialize_str("GOOD_TILL_TRIGGERED")
            } else if self.is_after_hours() {
                serializer.serialize_str("AFTER_HOURS")
            } else {
                serializer.serialize_str("UNKNOWN")
            }
        }
    }
    impl<'de> Deserialize<'de> for TimeInForce {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where D: Deserializer<'de> {
            let value = String::deserialize(deserializer)?.to_uppercase();
            match value.as_str() {
                "MARKET_HOURS" | "0" => Ok(Self::market_hours()),
                "IMMEDIATE_OR_CANCEL" | "3" => Ok(Self::immediate_or_cancel()),
                "GOOD_TILL_EXTENDED_HOURS" | "5" => Ok(Self::good_till_extended_hours()),
                "GOOD_TILL_TRIGGERED" | "6" => Ok(Self::good_till_triggered()),
                "AFTER_HOURS" | "E" => Ok(Self::after_hours()),
                _ => panic!("Unknown value for {}: {}", short_type_name::<Self>(), value),
            }
        }
    }
}

pub mod display {

    use super::*;
    char_ascii!(Display, #[derive(ByteSerializeStack, ByteDeserializeSlice, ByteSerializedSizeOf, ByteSerializedLenOf, PartialEq, Clone, Copy)]);
    #[rustfmt::skip]
    impl Display {
        pub fn visible() -> Self { Display(b'Y') }
        pub fn hidden() -> Self { Display(b'N') }
        pub fn attributable() -> Self { Display(b'A') }
        pub fn conformant() -> Self { Display(b'Z') }
        pub fn is_visible(&self) -> bool { self.0 == b'Y' }
        pub fn is_hidden(&self) -> bool { self.0 == b'N' }
        pub fn is_attributable(&self) -> bool { self.0 == b'A' }
        pub fn is_conformant(&self) -> bool { self.0 == b'Z' }
    }
    impl Serialize for Display {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: Serializer {
            if self.is_visible() {
                serializer.serialize_str("VISIBLE")
            } else if self.is_hidden() {
                serializer.serialize_str("HIDDEN")
            } else if self.is_attributable() {
                serializer.serialize_str("ATTRIBUTABLE")
            } else if self.is_conformant() {
                serializer.serialize_str("CONFORMANT")
            } else {
                serializer.serialize_str("UNKNOWN")
            }
        }
    }
    impl<'de> Deserialize<'de> for Display {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where D: Deserializer<'de> {
            let value = String::deserialize(deserializer)?.to_uppercase();
            match value.as_str() {
                "VISIBLE" | "Y" => Ok(Self::visible()),
                "HIDDEN" | "N" => Ok(Self::hidden()),
                "ATTRIBUTABLE" | "A" => Ok(Self::attributable()),
                "CONFORMANT" | "Z" => Ok(Self::conformant()),
                _ => panic!("Unknown value for {}: {}", short_type_name::<Self>(), value),
            }
        }
    }
}

pub mod capacity {
    use super::*;
    char_ascii!(Capacity, #[derive(ByteSerializeStack, ByteDeserializeSlice, ByteSerializedSizeOf, ByteSerializedLenOf, PartialEq, Clone, Copy)]);
    #[rustfmt::skip]
    impl Capacity{
        pub fn agency() -> Self { Capacity(b'A') }
        pub fn principal() -> Self { Capacity(b'P') }
        pub fn riskless_principal() -> Self { Capacity(b'R') }
        pub fn other() -> Self { Capacity(b'O') }
        pub fn is_agency(&self) -> bool { self.0 == b'A' }
        pub fn is_principal(&self) -> bool { self.0 == b'P' }
        pub fn is_riskless_principal(&self) -> bool { self.0 == b'R' }
        pub fn is_other(&self) -> bool { self.0 == b'O' }
    }
    impl Serialize for Capacity {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: Serializer {
            if self.is_agency() {
                serializer.serialize_str("AGENCY")
            } else if self.is_principal() {
                serializer.serialize_str("PRINCIPAL")
            } else if self.is_riskless_principal() {
                serializer.serialize_str("RISKLESS_PRINCIPAL")
            } else if self.is_other() {
                serializer.serialize_str("OTHER")
            } else {
                serializer.serialize_str("UNKNOWN")
            }
        }
    }
    impl<'de> Deserialize<'de> for Capacity {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where D: Deserializer<'de> {
            let value = String::deserialize(deserializer)?.to_uppercase();
            match value.as_str() {
                "AGENCY" | "A" => Ok(Self::agency()),
                "PRINCIPAL" | "P" => Ok(Self::principal()),
                "RISKLESS_PRINCIPAL" | "R" => Ok(Self::riskless_principal()),
                "OTHER" | "O" => Ok(Self::other()),
                _ => panic!("Unknown value for {}: {}", short_type_name::<Self>(), value),
            }
        }
    }
}

pub mod int_mkt_sweep_eligibility {
    use super::*;
    char_ascii!(IntMktSweepEligibility, #[derive(ByteSerializeStack, ByteDeserializeSlice, ByteSerializedSizeOf, ByteSerializedLenOf, PartialEq, Clone, Copy)]);
    #[rustfmt::skip]
    impl IntMktSweepEligibility{
        pub fn eligible() -> Self { IntMktSweepEligibility(b'Y') }
        pub fn not_eligible() -> Self { IntMktSweepEligibility(b'N') }
        pub fn is_eligible(&self) -> bool { self.0 == b'Y' }
        pub fn is_not_eligible(&self) -> bool { self.0 == b'N' }
    }
    impl Serialize for IntMktSweepEligibility {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: Serializer {
            if self.is_eligible() {
                serializer.serialize_str("ELIGIBLE")
            } else if self.is_not_eligible() {
                serializer.serialize_str("NOT_ELIGIBLE")
            } else {
                serializer.serialize_str("UNKNOWN")
            }
        }
    }
    impl<'de> Deserialize<'de> for IntMktSweepEligibility {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where D: Deserializer<'de> {
            let value = String::deserialize(deserializer)?.to_uppercase();
            match value.as_str() {
                "ELIGIBLE" | "Y" => Ok(Self::eligible()),
                "NOT_ELIGIBLE" | "N" => Ok(Self::not_eligible()),
                _ => panic!("Unknown value for {}: {}", short_type_name::<Self>(), value),
            }
        }
    }
}

pub mod cross_type {
    use super::*;
    char_ascii!(CrossType, #[derive(ByteSerializeStack, ByteDeserializeSlice, ByteSerializedSizeOf, ByteSerializedLenOf, PartialEq, Clone, Copy)]);
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
        pub fn is_continuous_market(&self) -> bool { self.0 == b'N' }
        pub fn is_opening_cross(&self) -> bool { self.0 == b'O' }
        pub fn is_closing_cross(&self) -> bool { self.0 == b'C' }
        pub fn is_halt_ipo(&self) -> bool { self.0 == b'H' }
        pub fn is_supplemental(&self) -> bool { self.0 == b'S' }
        pub fn is_retail(&self) -> bool { self.0 == b'R' }
        pub fn is_extended_life(&self) -> bool { self.0 == b'E' }
        pub fn is_after_hours_close(&self) -> bool { self.0 == b'A' }
    }
    impl Serialize for CrossType {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: Serializer {
            if self.is_continuous_market() {
                serializer.serialize_str("CONTINUOUS_MARKET")
            } else if self.is_opening_cross() {
                serializer.serialize_str("OPENING_CROSS")
            } else if self.is_closing_cross() {
                serializer.serialize_str("CLOSING_CROSS")
            } else if self.is_halt_ipo() {
                serializer.serialize_str("HALT_IPO")
            } else if self.is_supplemental() {
                serializer.serialize_str("SUPPLEMENTAL")
            } else if self.is_retail() {
                serializer.serialize_str("RETAIL")
            } else if self.is_extended_life() {
                serializer.serialize_str("EXTENDED_LIFE")
            } else if self.is_after_hours_close() {
                serializer.serialize_str("AFTER_HOURS_CLOSE")
            } else {
                serializer.serialize_str("UNKNOWN")
            }
        }
    }
    impl<'de> Deserialize<'de> for CrossType {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where D: Deserializer<'de> {
            let value = String::deserialize(deserializer)?.to_uppercase();
            match value.as_str() {
                "CONTINUOUS_MARKET" | "N" => Ok(Self::continuous_market()),
                "OPENING_CROSS" | "O" => Ok(Self::opening_cross()),
                "CLOSING_CROSS" | "C" => Ok(Self::closing_cross()),
                "HALT_IPO" | "H" => Ok(Self::halt_ipo()),
                "SUPPLEMENTAL" | "S" => Ok(Self::supplemental()),
                "RETAIL" | "R" => Ok(Self::retail()),
                "EXTENDED_LIFE" | "E" => Ok(Self::extended_life()),
                "AFTER_HOURS_CLOSE" | "A" => Ok(Self::after_hours_close()),
                _ => panic!("Unknown value for {}: {}", short_type_name::<Self>(), value),
            }
        }
    }
}

pub mod event_code {
    use super::*;
    char_ascii!(EventCode, #[derive(ByteSerializeStack, ByteDeserializeSlice, ByteSerializedSizeOf, ByteSerializedLenOf, PartialEq, Clone, Copy)]);
    #[rustfmt::skip]
    impl EventCode{
        pub fn start_of_day() -> Self { EventCode(b'S') }
        pub fn end_of_day() -> Self { EventCode(b'E') }
        pub fn is_start_of_day(&self) -> bool { self.0 == b'S' }
        pub fn is_end_of_day(&self) -> bool { self.0 == b'E' }
    }
    impl Serialize for EventCode {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: Serializer {
            if self.is_start_of_day() {
                serializer.serialize_str("START_OF_DAY")
            } else if self.is_end_of_day() {
                serializer.serialize_str("END_OF_DAY")
            } else {
                serializer.serialize_str("UNKNOWN")
            }
        }
    }
    impl<'de> Deserialize<'de> for EventCode {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where D: Deserializer<'de> {
            let value = String::deserialize(deserializer)?.to_uppercase();
            match value.as_str() {
                "START_OF_DAY" | "S" => Ok(Self::start_of_day()),
                "END_OF_DAY" | "E" => Ok(Self::end_of_day()),
                _ => panic!("Unknown value for {}: {}", short_type_name::<Self>(), value),
            }
        }
    }
}

pub mod order_state {
    use super::*;
    char_ascii!(OrderState, #[derive(ByteSerializeStack, ByteDeserializeSlice, ByteSerializedSizeOf, ByteSerializedLenOf, PartialEq, Clone, Copy)]);
    #[rustfmt::skip]
    impl OrderState{
        pub fn live() -> Self { OrderState(b'L') }
        pub fn dead() -> Self { OrderState(b'D') }
        pub fn is_live(&self) -> bool { self.0 == b'L' }
        pub fn is_dead(&self) -> bool { self.0 == b'D' }
    }
    impl Serialize for OrderState {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: Serializer {
            if self.is_live() {
                serializer.serialize_str("LIVE")
            } else if self.is_dead() {
                serializer.serialize_str("DEAD")
            } else {
                serializer.serialize_str("UNKNOWN")
            }
        }
    }
    impl<'de> Deserialize<'de> for OrderState {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where D: Deserializer<'de> {
            let value = String::deserialize(deserializer)?.to_uppercase();
            match value.as_str() {
                "LIVE" | "L" => Ok(Self::live()),
                "DEAD" | "D" => Ok(Self::dead()),
                _ => panic!("Unknown value for {}: {}", short_type_name::<Self>(), value),
            }
        }
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

    // TODO add json friendly serialization 1h:30m:15s:123ms:456us:789ns example: 01:30:15.123456789
    u64_tuple!(Timestamp, "be", #[derive(ByteSerializeStack, ByteDeserializeSlice, ByteSerializedSizeOf, ByteSerializedLenOf, Serialize, Deserialize, PartialEq, Debug, Clone, Copy)]);
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
        #[inline(always)]
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
    u64_tuple!(OrderReferenceNumber, "be", #[derive(ByteSerializeStack, ByteDeserializeSlice, ByteSerializedSizeOf, ByteSerializedLenOf, Serialize, Deserialize, PartialEq, Clone, Copy, Debug, Default)]);

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
        pub fn is_reg_restriction(&self) -> bool{ self.0 == b'D' }
        pub fn is_closed(&self) -> bool{ self.0 == b'E' }
        pub fn is_post_only_cancel_nms(&self) -> bool{ self.0 == b'F' }
        pub fn is_post_only_cancel_displayed(&self) -> bool{ self.0 == b'G' }
        pub fn is_halted(&self) -> bool{ self.0 == b'H' }
        pub fn is_immediate_or_cancel(&self) -> bool{ self.0 == b'I' }
        pub fn is_market_collars(&self) -> bool{ self.0 == b'K' }
        pub fn is_self_match_prevention(&self) -> bool{ self.0 == b'Q' }
        pub fn is_supervisory(&self) -> bool{ self.0 == b'S' }
        pub fn is_timeout(&self) -> bool{ self.0 == b'T' }
        pub fn is_user_requested(&self) -> bool{ self.0 == b'U' }
        pub fn is_open_protection(&self) -> bool{ self.0 == b'X' }
        pub fn is_system_cancel(&self) -> bool{ self.0 == b'Z' }
        pub fn is_exceeds_allowable_shares(&self) -> bool{ self.0 == b'e' }        
    }
    impl Serialize for CancelReason {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: Serializer {
            if self.is_reg_restriction() {
                serializer.serialize_str("REG_RESTRICTION")
            } else if self.is_closed() {
                serializer.serialize_str("CLOSED")
            } else if self.is_post_only_cancel_nms() {
                serializer.serialize_str("POST_ONLY_CANCEL_NMS")
            } else if self.is_post_only_cancel_displayed() {
                serializer.serialize_str("POST_ONLY_CANCEL_DISPLAYED")
            } else if self.is_halted() {
                serializer.serialize_str("HALTED")
            } else if self.is_immediate_or_cancel() {
                serializer.serialize_str("IMMEDIATE_OR_CANCEL")
            } else if self.is_market_collars() {
                serializer.serialize_str("MARKET_COLLARS")
            } else if self.is_self_match_prevention() {
                serializer.serialize_str("SELF_MATCH_PREVENTION")
            } else if self.is_supervisory() {
                serializer.serialize_str("SUPERVISORY")
            } else if self.is_timeout() {
                serializer.serialize_str("TIMEOUT")
            } else if self.is_user_requested() {
                serializer.serialize_str("USER_REQUESTED")
            } else if self.is_open_protection() {
                serializer.serialize_str("OPEN_PROTECTION")
            } else if self.is_system_cancel() {
                serializer.serialize_str("SYSTEM_CANCEL")
            } else if self.is_exceeds_allowable_shares() {
                serializer.serialize_str("EXCEEDS_ALLOWABLE_SHARES")
            } else {
                serializer.serialize_str("UNKNOWN")
            }
        }
    }
    impl<'de> Deserialize<'de> for CancelReason {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where D: Deserializer<'de> {
            let value = String::deserialize(deserializer)?.to_uppercase();
            match value.as_str() {
                "REG_RESTRICTION" | "D" => Ok(Self::reg_restriction()),
                "CLOSED" | "E" => Ok(Self::closed()),
                "POST_ONLY_CANCEL_NMS" | "F" => Ok(Self::post_only_cancel_nms()),
                "POST_ONLY_CANCEL_DISPLAYED" | "G" => Ok(Self::post_only_cancel_displayed()),
                "HALTED" | "H" => Ok(Self::halted()),
                "IMMEDIATE_OR_CANCEL" | "I" => Ok(Self::immediate_or_cancel()),
                "MARKET_COLLARS" | "K" => Ok(Self::market_collars()),
                "SELF_MATCH_PREVENTION" | "Q" => Ok(Self::self_match_prevention()),
                "SUPERVISORY" | "S" => Ok(Self::supervisory()),
                "TIMEOUT" | "T" => Ok(Self::timeout()),
                "USER_REQUESTED" | "U" => Ok(Self::user_requested()),
                "OPEN_PROTECTION" | "X" => Ok(Self::open_protection()),
                "SYSTEM_CANCEL" | "Z" => Ok(Self::system_cancel()),
                "EXCEEDS_ALLOWABLE_SHARES" | "e" => Ok(Self::exceeds_allowable_shares()),
                _ => panic!("Unknown value for {}: {}", short_type_name::<Self>(), value),
            }
        }
    }
}

pub mod cancel_reason_aiq {
    use super::*;

    #[rustfmt::skip]
    const_char_ascii!(CancelAiqReason, b'Q', #[derive(ByteSerializeStack, ByteSerializedSizeOf, ByteSerializedLenOf, PartialEq, Clone, Copy)]);
}

pub mod liquidity_flag {
    use super::*;

    #[rustfmt::skip]
    char_ascii!(LiquidityFlag,  #[derive(ByteSerializeStack, ByteDeserializeSlice, ByteSerializedSizeOf, ByteSerializedLenOf, PartialEq, Clone, Copy)]);
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
        pub fn is_added(&self) -> bool{ self.0 == b'A' }
        pub fn is_closing_cross(&self) -> bool{ self.0 == b'C' }
        pub fn is_retail_designated_that_added_display_liq(&self) -> bool{ self.0 == b'e' }
        pub fn is_halt_ipo_cross(&self) -> bool{ self.0 == b'H' }
        pub fn is_after_hours_closing_cross(&self) -> bool{ self.0 == b'i' }
        pub fn is_non_display_adding_liq(&self) -> bool{ self.0 == b'J' }
        pub fn is_rpi_order_provides_liq(&self) -> bool{ self.0 == b'j' }
        pub fn is_added_liq_via_midpoint_order(&self) -> bool{ self.0 == b'k' }
        pub fn is_halt_cross(&self) -> bool{ self.0 == b'K' }
        pub fn is_closing_cross_imbalance(&self) -> bool{ self.0 == b'L' }
        pub fn is_opening_cross_imbalance(&self) -> bool{ self.0 == b'M' }
        pub fn is_removed_liq_at_midpoint(&self) -> bool{ self.0 == b'm' }
        pub fn is_passing_midpoint_execution(&self) -> bool{ self.0 == b'N' }
        pub fn is_midpoint_extended_life_order(&self) -> bool{ self.0 == b'n' }
        pub fn is_opening_cross(&self) -> bool{ self.0 == b'O' }
        pub fn is_removed_price_improving_non_display_liq(&self) -> bool{ self.0 == b'p' }
        pub fn is_rmo_retail_order_removes_non_rpi_midpoint_liq(&self) -> bool{ self.0 == b'q' }
        pub fn is_removed(&self) -> bool{ self.0 == b'R' }
        pub fn is_retail_order_removes_rpi_liq(&self) -> bool{ self.0 == b'r' }
        pub fn is_retain_order_removes_price_improving_non_display_liq_not_rpi_liq(&self) -> bool{ self.0 == b't' }
        pub fn is_supplemental_order_execution(&self) -> bool{ self.0 == b'0' }
        pub fn is_displayed_liq_adding_order_improves_nnbo(&self) -> bool{ self.0 == b'7' }
        pub fn is_displayed_liq_adding_order_sets_qbbo(&self) -> bool{ self.0 == b'8' }
        pub fn is_rpi_order_provides_liq_no_rpii(&self) -> bool{ self.0 == b'1' }
    }
    impl Serialize for LiquidityFlag {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: Serializer {
            if self.is_added() {
                serializer.serialize_str("ADDED")
            } else if self.is_closing_cross() {
                serializer.serialize_str("CLOSING_CROSS")
            } else if self.is_retail_designated_that_added_display_liq() {
                serializer.serialize_str("RETAIL_DESIGNATED_THAT_ADDED_DISPLAY_LIQ")
            } else if self.is_halt_ipo_cross() {
                serializer.serialize_str("HALT_IPO_CROSS")
            } else if self.is_after_hours_closing_cross() {
                serializer.serialize_str("AFTER_HOURS_CLOSING_CROSS")
            } else if self.is_non_display_adding_liq() {
                serializer.serialize_str("NON_DISPLAY_ADDING_LIQ")
            } else if self.is_rpi_order_provides_liq() {
                serializer.serialize_str("RPI_ORDER_PROVIDES_LIQ")
            } else if self.is_added_liq_via_midpoint_order() {
                serializer.serialize_str("ADDED_LIQ_VIA_MIDPOINT_ORDER")
            } else if self.is_halt_cross() {
                serializer.serialize_str("HALT_CROSS")
            } else if self.is_closing_cross_imbalance() {
                serializer.serialize_str("CLOSING_CROSS_IMBALANCE")
            } else if self.is_opening_cross_imbalance() {
                serializer.serialize_str("OPENING_CROSS_IMBALANCE")
            } else if self.is_removed_liq_at_midpoint() {
                serializer.serialize_str("REMOVED_LIQ_AT_MIDPOINT")
            } else if self.is_passing_midpoint_execution() {
                serializer.serialize_str("PASSING_MIDPOINT_EXECUTION")
            } else if self.is_midpoint_extended_life_order() {
                serializer.serialize_str("MIDPOINT_EXTENDED_LIFE_ORDER")
            } else if self.is_opening_cross() {
                serializer.serialize_str("OPENING_CROSS")
            } else if self.is_removed_price_improving_non_display_liq() {
                serializer.serialize_str("REMOVED_PRICE_IMPROVING_NON_DISPLAY_LIQ")
            } else if self.is_rmo_retail_order_removes_non_rpi_midpoint_liq() {
                serializer.serialize_str("RMO_RETAIL_ORDER_REMOVES_NON_RPI_MIDPOINT_LIQ")
            } else if self.is_removed() {
                serializer.serialize_str("REMOVED")
            } else if self.is_retail_order_removes_rpi_liq() {
                serializer.serialize_str("RETAIL_ORDER_REMOVES_RPI_LIQ")
            } else if self.is_retain_order_removes_price_improving_non_display_liq_not_rpi_liq() {
                serializer.serialize_str("RETAIN_ORDER_REMOVES_PRICE_IMPROVING_NON_DISPLAY_LIQ_NOT_RPI_LIQ")
            } else if self.is_supplemental_order_execution() {
                serializer.serialize_str("SUPPLEMENTAL_ORDER_EXECUTION")
            } else if self.is_displayed_liq_adding_order_improves_nnbo() {
                serializer.serialize_str("DISPLAYED_LIQ_ADDING_ORDER_IMPROVES_NNBO")
            } else if self.is_displayed_liq_adding_order_sets_qbbo() {
                serializer.serialize_str("DISPLAYED_LIQ_ADDING_ORDER_SETS_QBBO")
            } else if self.is_rpi_order_provides_liq_no_rpii() {
                serializer.serialize_str("RPI_ORDER_PROVIDES_LIQ_NO_RPII")
            } else {
                serializer.serialize_str("UNKNOWN")
            }
        }
    }
    impl<'de> Deserialize<'de> for LiquidityFlag {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where D: Deserializer<'de> {
            let value = String::deserialize(deserializer)?.to_uppercase();
            match value.as_str() {
                "ADDED" | "A" => Ok(Self::added()),
                "CLOSING_CROSS" | "C" => Ok(Self::closing_cross()),
                "RETAIL_DESIGNATED_THAT_ADDED_DISPLAY_LIQ" | "e" => Ok(Self::retail_designated_that_added_display_liq()),
                "HALT_IPO_CROSS" | "H" => Ok(Self::halt_ipo_cross()),
                "AFTER_HOURS_CLOSING_CROSS" | "i" => Ok(Self::after_hours_closing_cross()),
                "NON_DISPLAY_ADDING_LIQ" | "J" => Ok(Self::non_display_adding_liq()),
                "RPI_ORDER_PROVIDES_LIQ" | "j" => Ok(Self::rpi_order_provides_liq()),
                "ADDED_LIQ_VIA_MIDPOINT_ORDER" | "k" => Ok(Self::added_liq_via_midpoint_order()),
                "HALT_CROSS" | "K" => Ok(Self::halt_cross()),
                "CLOSING_CROSS_IMBALANCE" | "L" => Ok(Self::closing_cross_imbalance()),
                "OPENING_CROSS_IMBALANCE" | "M" => Ok(Self::opening_cross_imbalance()),
                "REMOVED_LIQ_AT_MIDPOINT" | "m" => Ok(Self::removed_liq_at_midpoint()),
                "PASSING_MIDPOINT_EXECUTION" | "N" => Ok(Self::passing_midpoint_execution()),
                "MIDPOINT_EXTENDED_LIFE_ORDER" | "n" => Ok(Self::midpoint_extended_life_order()),
                "OPENING_CROSS" | "O" => Ok(Self::opening_cross()),
                "REMOVED_PRICE_IMPROVING_NON_DISPLAY_LIQ" | "p" => Ok(Self::removed_price_improving_non_display_liq()),
                "RMO_RETAIL_ORDER_REMOVES_NON_RPI_MIDPOINT_LIQ" | "q" => Ok(Self::rmo_retail_order_removes_non_rpi_midpoint_liq()),
                "REMOVED" | "R" => Ok(Self::removed()),
                "RETAIL_ORDER_REMOVES_RPI_LIQ" | "r" => Ok(Self::retail_order_removes_rpi_liq()),
                "RETAIN_ORDER_REMOVES_PRICE_IMPROVING_NON_DISPLAY_LIQ_NOT_RPI_LIQ" | "t" => Ok(Self::retain_order_removes_price_improving_non_display_liq_not_rpi_liq()),
                "SUPPLEMENTAL_ORDER_EXECUTION" | "0" => Ok(Self::supplemental_order_execution()),
                "DISPLAYED_LIQ_ADDING_ORDER_IMPROVES_NNBO" | "7" => Ok(Self::displayed_liq_adding_order_improves_nnbo()),
                "DISPLAYED_LIQ_ADDING_ORDER_SETS_QBBO" | "8" => Ok(Self::displayed_liq_adding_order_sets_qbbo()),
                "RPI_ORDER_PROVIDES_LIQ_NO_RPII" | "1" => Ok(Self::rpi_order_provides_liq_no_rpii()),
                _ => panic!("Unknown value for {}: {}", short_type_name::<Self>(), value),
            }
        }
    }
}

pub mod aiq_strategy {
    use super::*;

    char_ascii!(AiqStrategy, true, #[derive(ByteSerializeStack, ByteDeserializeSlice, ByteSerializedSizeOf, ByteSerializedLenOf, PartialEq, Clone, Copy)]);
    impl Default for AiqStrategy {
        fn default() -> Self {
            AiqStrategy(b'?') // specification does not list valid values
        }
    }
}

pub mod match_number {
    use super::*;

    u64_tuple!(MatchNumber, "be", #[derive(ByteSerializeStack, ByteDeserializeSlice, ByteSerializedSizeOf, ByteSerializedLenOf, Serialize, Deserialize, PartialEq, Clone, Copy, Debug, Default)]);
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
        pub fn is_erroneous(&self) -> bool{ self.0 == b'E' }
        pub fn is_consent(&self) -> bool{ self.0 == b'C' }
        pub fn is_supervisory(&self) -> bool{ self.0 == b'S' }
        pub fn is_external(&self) -> bool{ self.0 == b'X' }
    }
    impl Serialize for BrokenTradeReason {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: Serializer {
            if self.is_erroneous() {
                serializer.serialize_str("ERRONEOUS")
            } else if self.is_consent() {
                serializer.serialize_str("CONSENT")
            } else if self.is_supervisory() {
                serializer.serialize_str("SUPERVISORY")
            } else if self.is_external() {
                serializer.serialize_str("EXTERNAL")
            } else {
                serializer.serialize_str("UNKNOWN")
            }
        }
    }
    impl<'de> Deserialize<'de> for BrokenTradeReason {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where D: Deserializer<'de> {
            let value = String::deserialize(deserializer)?.to_uppercase();
            match value.as_str() {
                "ERRONEOUS" | "E" => Ok(Self::erroneous()),
                "CONSENT" | "C" => Ok(Self::consent()),
                "SUPERVISORY" | "S" => Ok(Self::supervisory()),
                "EXTERNAL" | "X" => Ok(Self::external()),
                _ => panic!("Unknown value for {}: {}", short_type_name::<Self>(), value),
            }
        }
    }
}

pub mod order_reject_reason {
    use super::*;

    u16_tuple!(OrderRejectReason, "be", #[derive(ByteSerializeStack, ByteDeserializeSlice, ByteSerializedSizeOf, ByteSerializedLenOf, Serialize, Deserialize, PartialEq, Clone, Copy, Debug, Default)]);
    #[rustfmt::skip]
    impl OrderRejectReason{
        pub fn quote_unavailable() -> Self{ OrderRejectReason(0x01) }
        pub fn destination_closed() -> Self{ OrderRejectReason(0x02) }
        pub fn invalid_display() -> Self{ OrderRejectReason(0x03) }
        pub fn invalid_max_floor() -> Self{ OrderRejectReason(0x04) }
        pub fn invalid_peg_type() -> Self{ OrderRejectReason(0x05) }
        pub fn fat_finger() -> Self{ OrderRejectReason(0x06) }
        pub fn halted() -> Self { OrderRejectReason(0x07) }
        pub fn iso_not_allowed() -> Self { OrderRejectReason(0x08) } 
        pub fn invalid_side() -> Self { OrderRejectReason(0x09) } 
        pub fn processing_error() -> Self { OrderRejectReason(0x0A) } 
        pub fn cancel_pending() -> Self { OrderRejectReason(0x0B) } 
        pub fn firm_not_authorized() -> Self { OrderRejectReason(0x0C) } 
        pub fn invalid_min_quantity() -> Self { OrderRejectReason(0x0D) } 
        pub fn no_closing_reference_price() -> Self { OrderRejectReason(0x0E) } 
        pub fn other() -> Self { OrderRejectReason(0x0F) } 
        pub fn cancel_not_allowed() -> Self { OrderRejectReason(0x10) } 
        pub fn pegging_not_allowed() -> Self { OrderRejectReason(0x11) } 
        pub fn crossed_market() -> Self { OrderRejectReason(0x12) } 
        pub fn invalid_quantity() -> Self { OrderRejectReason(0x13) } 
        pub fn invalid_cross_order() -> Self { OrderRejectReason(0x14) } 
        pub fn replace_not_allowed() -> Self { OrderRejectReason(0x15) } 
        pub fn routing_not_allowed() -> Self { OrderRejectReason(0x16) } 
        pub fn invalid_symbol() -> Self { OrderRejectReason(0x17) } 
        pub fn test() -> Self { OrderRejectReason(0x18) } 
        pub fn late_loc_too_aggressive() -> Self { OrderRejectReason(0x19) } 
        pub fn retail_not_allowed() -> Self { OrderRejectReason(0x1A) } 
        pub fn invalid_midpoint_post_only_price() -> Self { OrderRejectReason(0x1B) } 
        pub fn invalid_destination() -> Self { OrderRejectReason(0x1C) } 
        pub fn invalid_price() -> Self { OrderRejectReason(0x1D) } 
        pub fn shares_exceed_threshold() -> Self { OrderRejectReason(0x1E) } 
        pub fn exceeds_maximum_allowed_notional_value() -> Self { OrderRejectReason(0x1F) } 
        pub fn risk_aggregate_exposure_exceeded() -> Self { OrderRejectReason(0x20) } 
        pub fn risk_market_impact() -> Self { OrderRejectReason(0x21) } 
        pub fn risk_restricted_stock() -> Self { OrderRejectReason(0x22) } 
        pub fn risk_short_sell_restricted() -> Self { OrderRejectReason(0x23) }
        pub fn risk_order_type_restricted() -> Self { OrderRejectReason(0x24) }
        pub fn risk_exceeds_adv_limit() -> Self { OrderRejectReason(0x25) }
        pub fn risk_fat_finger() -> Self { OrderRejectReason(0x26) }
        pub fn risk_locate_required() -> Self { OrderRejectReason(0x27) }
        pub fn risk_symbol_message_rate_restriction() -> Self { OrderRejectReason(0x28) }
        pub fn risk_port_message_rate_restriction() -> Self { OrderRejectReason(0x29) }
        pub fn risk_duplicate_message_rate_restriction() -> Self { OrderRejectReason(0x2A) }
        pub fn is_quote_unavailable(&self) -> bool { self.0 == 0x01 }
        pub fn is_destination_closed(&self) -> bool { self.0 == 0x02 }
        pub fn is_invalid_display(&self) -> bool { self.0 == 0x03 }
        pub fn is_invalid_max_floor(&self) -> bool { self.0 == 0x04 }
        pub fn is_invalid_peg_type(&self) -> bool { self.0 == 0x05 }
        pub fn is_fat_finger(&self) -> bool { self.0 == 0x06 }
        pub fn is_halted(&self) -> bool { self.0 == 0x07 }
        pub fn is_iso_not_allowed(&self) -> bool { self.0 == 0x08 }
        pub fn is_invalid_side(&self) -> bool { self.0 == 0x09 }
        pub fn is_processing_error(&self) -> bool { self.0 == 0x0A }
        pub fn is_cancel_pending(&self) -> bool { self.0 == 0x0B }
        pub fn is_firm_not_authorized(&self) -> bool { self.0 == 0x0C }
        pub fn is_invalid_min_quantity(&self) -> bool { self.0 == 0x0D }
        pub fn is_no_closing_reference_price(&self) -> bool { self.0 == 0x0E }
        pub fn is_other(&self) -> bool { self.0 == 0x0F }
        pub fn is_risk_aggregate_exposure_exceeded(&self) -> bool { self.0 == 0x20 }
        pub fn is_risk_market_impact(&self) -> bool { self.0 == 0x21 }
        pub fn is_risk_restricted_stock(&self) -> bool { self.0 == 0x22 }
        pub fn is_risk_short_sell_restricted(&self) -> bool { self.0 == 0x23 }
        pub fn is_risk_order_type_restricted(&self) -> bool { self.0 == 0x24 }
        pub fn is_risk_exceeds_adv_limit(&self) -> bool { self.0 == 0x25 }
        pub fn is_risk_fat_finger(&self) -> bool { self.0 == 0x26 }
        pub fn is_risk_locate_required(&self) -> bool { self.0 == 0x27 }
        pub fn is_risk_symbol_message_rate_restriction(&self) -> bool { self.0 == 0x28 }
        pub fn is_risk_port_message_rate_restriction(&self) -> bool { self.0 == 0x29 }
        pub fn is_risk_duplicate_message_rate_restriction(&self) -> bool { self.0 == 0x2A }
    }
}

pub mod order_restated_reason {
    use super::*;

    char_ascii!(RestatedReason, #[derive(ByteSerializeStack, ByteDeserializeSlice, ByteSerializedSizeOf, ByteSerializedLenOf, PartialEq, Clone, Copy, Default)]);

    #[rustfmt::skip]
    impl RestatedReason{
        pub fn refresh_of_display() -> Self { RestatedReason(b'R') }
        pub fn update_of_displayed_price() -> Self { RestatedReason(b'P') }
        pub fn is_refresh_of_display(&self) -> bool { self.0 == b'R' }
        pub fn is_update_of_displayed_price(&self) -> bool { self.0 == b'P' }
    }
    impl Serialize for RestatedReason {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: Serializer {
            if self.is_refresh_of_display() {
                serializer.serialize_str("REFRESH_OF_DISPLAY")
            } else if self.is_update_of_displayed_price() {
                serializer.serialize_str("UPDATE_OF_DISPLAYED_PRICE")
            } else {
                serializer.serialize_str("UNKNOWN")
            }
        }
    }
    impl<'de> Deserialize<'de> for RestatedReason {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where D: Deserializer<'de> {
            let value = String::deserialize(deserializer)?.to_uppercase();
            match value.as_str() {
                "REFRESH_OF_DISPLAY" | "R" => Ok(Self::refresh_of_display()),
                "UPDATE_OF_DISPLAYED_PRICE" | "P" => Ok(Self::update_of_displayed_price()),
                _ => panic!("Unknown value for {}: {}", short_type_name::<Self>(), value),
            }
        }
    }
}
