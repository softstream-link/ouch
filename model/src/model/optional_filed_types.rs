use byteserde::prelude::*;
use byteserde_derive::{ByteDeserializeSlice, ByteSerializeStack, ByteSerializedLenOf, ByteSerializedSizeOf};
use byteserde_types::{char_ascii, f32_tuple, string_ascii_fixed, u16_tuple, u32_tuple, u64_tuple};
use links_core::core::macros::short_type_name;
use serde::{Deserialize, Serialize};
use serde::{Deserializer, Serializer};
use std::mem::size_of;

pub use optional_values::{
    bbo_weight_indicator::*, customer_type::*, discretion_peg_offset::*, discretion_price::*, discretion_price_type::*, display_price::*, display_qty::*, expire_time::*, firm::*, group_id::*, handle_inst::*, max_floor::*, min_qty::*, peg_offset::*, post_only::*, price_type::*, random_reserves::*,
    route::*, secondary_ord_ref_num::*, shares_located::*, trade_now::*,
};

pub trait OptionTag {
    fn tag() -> u8;
    fn tag_as_slice() -> &'static [u8];
}
/// Implements [OptionTag] trait used by byteserde to serialize/deserialize optional fields
macro_rules! option_tag {
    ($T:ty, $TAG:literal) => {
        impl OptionTag for $T {
            #[inline(always)]
            fn tag() -> u8 {
                $TAG
            }
            #[inline(always)]
            fn tag_as_slice() -> &'static [u8] {
                &[$TAG]
            }
        }
    };
}
/// Implements [From] between all optional fields and [TagValueElement] container for optional fields of type numerics
macro_rules! numeric_into_tag_value {
    ($FOR_TYPE:ty, $FROM_TYPE:ty) => {
        impl From<$FROM_TYPE> for TagValueElement<$FOR_TYPE> {
            #[inline(always)]
            fn from(v: $FROM_TYPE) -> Self {
                let v: $FOR_TYPE = v.into();
                TagValueElement::new(v)
            }
        }
        impl From<$FOR_TYPE> for TagValueElement<$FOR_TYPE> {
            #[inline(always)]
            fn from(v: $FOR_TYPE) -> Self {
                TagValueElement::new(v)
            }
        }
    };
}
/// Implements [From] between all optional fields and [TagValueElement] container for optional fields of type string_ascii_fixed
macro_rules! string_ascii_fixed_into_tag_value {
    ($FOR_TYPE:ty) => {
        impl From<&[u8; size_of::<$FOR_TYPE>()]> for TagValueElement<$FOR_TYPE> {
            #[inline(always)]
            fn from(v: &[u8; size_of::<$FOR_TYPE>()]) -> Self {
                let v: $FOR_TYPE = v.into();
                TagValueElement::new(v)
            }
        }
        impl From<&[u8]> for TagValueElement<$FOR_TYPE> {
            #[inline(always)]
            fn from(v: &[u8]) -> Self {
                let v: $FOR_TYPE = v.into();
                TagValueElement::new(v)
            }
        }
        impl From<$FOR_TYPE> for TagValueElement<$FOR_TYPE> {
            #[inline(always)]
            fn from(v: $FOR_TYPE) -> Self {
                TagValueElement::new(v)
            }
        }
    };
}
/// Implements [From] between all optional fields and [TagValueElement] container for optional fields of type char_ascii
macro_rules! char_ascii_into_tag_value {
    ($FOR_TYPE:ty) => {
        impl From<u8> for TagValueElement<$FOR_TYPE> {
            #[inline(always)]
            fn from(v: u8) -> Self {
                let v: $FOR_TYPE = v.into();
                TagValueElement::new(v)
            }
        }
        impl From<$FOR_TYPE> for TagValueElement<$FOR_TYPE> {
            #[inline(always)]
            fn from(v: $FOR_TYPE) -> Self {
                TagValueElement::new(v)
            }
        }
    };
}
/// Implements [From] and [serde] serialization for Price fields types to enable f32 user friendly interface
macro_rules! f32_unsigned_price_u64 {
    ($TYPE:ty) => {
        impl From<f32> for $TYPE {
            #[inline(always)]
            fn from(f: f32) -> Self {
                debug_assert!(f >= 0.0, "from: {} must be positive to create struct of {} type", f, short_type_name::<$TYPE>());
                Self((f * crate::prelude::price::PRICE_SCALE) as u64)
            }
        }
        impl From<&$TYPE> for f32 {
            #[inline(always)]
            fn from(p: &$TYPE) -> f32 {
                p.0 as f32 / crate::prelude::price::PRICE_SCALE
            }
        }
        impl From<f32> for TagValueElement<$TYPE> {
            #[inline(always)]
            fn from(v: f32) -> Self {
                let v: $TYPE = v.into();
                TagValueElement::new(v)
            }
        }
        impl serde::Serialize for $TYPE {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                serializer.serialize_f32(f32::from(self))
            }
        }
        impl<'de> serde::Deserialize<'de> for $TYPE {
            fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                let f = f32::deserialize(deserializer)?;
                Ok(f.into())
            }
        }
        impl std::fmt::Debug for $TYPE {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.debug_tuple(short_type_name::<$TYPE>()).field(&f32::from(self)).finish()
            }
        }
    };
}
mod optional_values {
    use super::*;
    pub mod secondary_ord_ref_num {
        use super::*;
        u64_tuple!(SecondaryOrdRefNum, "be", #[derive(ByteSerializeStack, ByteDeserializeSlice, ByteSerializedSizeOf, ByteSerializedLenOf, Serialize, Deserialize, PartialEq, Debug, Clone, Copy)]);
        option_tag!(SecondaryOrdRefNum, 1);
        numeric_into_tag_value!(SecondaryOrdRefNum, u64);

        #[cfg(test)]
        mod test {
            use crate::prelude::*;
            use links_core::unittest::setup;
            use log::info;
            use serde_json::{from_str, to_string};
            #[test]
            fn test_msg_serde() {
                setup::log::configure_compact(log::LevelFilter::Info);
                type T = TagValueElement<SecondaryOrdRefNum>;
                let msg_inp: T = 1.into();
                info!("msg_inp: {:?}", msg_inp);

                let json_out = to_string(&msg_inp).unwrap();
                info!("json_out: {}", json_out);
                assert_eq!(json_out, r#"1"#);

                let msg_out: T = from_str(&json_out).unwrap();
                info!("msg_out: {:?}", msg_out);
                assert_eq!(msg_out, msg_inp);
            }
        }
    }
    pub mod firm {
        use super::*;
        string_ascii_fixed!(Firm, 4, b' ', true, true, #[derive(ByteSerializeStack, ByteDeserializeSlice, ByteSerializedSizeOf, ByteSerializedLenOf, PartialEq, Clone, Copy)]);
        option_tag!(Firm, 2);
        string_ascii_fixed_into_tag_value!(Firm);

        impl Default for Firm {
            /// Space filled, port default
            #[inline(always)]
            fn default() -> Self {
                Firm(*b"    ")
            }
        }

        #[cfg(test)]
        mod test {
            use crate::prelude::*;
            use links_core::unittest::setup;
            use log::info;
            use serde_json::{from_str, to_string};
            #[test]
            fn test_msg_serde() {
                setup::log::configure_compact(log::LevelFilter::Info);
                type T = TagValueElement<Firm>;
                let msg_inp: T = b"ABCD".into();
                info!("msg_inp: {:?}", msg_inp);
                let msg_inp: T = b"AB".as_slice().into();
                info!("msg_inp: {:?}", msg_inp);

                let json_out = to_string(&msg_inp).unwrap();
                info!("json_out: {}", json_out);
                assert_eq!(json_out, r#""AB""#);

                let msg_out: T = from_str(&json_out).unwrap();
                info!("msg_out: {:?}", msg_out);
                assert_eq!(msg_out, msg_inp);
            }
        }
    }
    pub mod min_qty {
        use super::*;
        u32_tuple!(MinQty, "be", #[derive(ByteSerializeStack, ByteDeserializeSlice, ByteSerializedSizeOf, ByteSerializedLenOf, Serialize, Deserialize, PartialEq, Debug, Clone, Copy)]);
        option_tag!(MinQty, 3);
        numeric_into_tag_value!(MinQty, u32);

        impl Default for MinQty {
            /// Zero, no min qty
            #[inline(always)]
            fn default() -> Self {
                MinQty(0)
            }
        }
        #[cfg(test)]
        mod test {
            use crate::prelude::*;
            use links_core::unittest::setup;
            use log::info;
            use serde_json::{from_str, to_string};
            #[test]
            fn test_msg_serde() {
                setup::log::configure_compact(log::LevelFilter::Info);
                type T = TagValueElement<MinQty>;
                let msg_inp: T = 1.into();
                info!("msg_inp: {:?}", msg_inp);

                let json_out = to_string(&msg_inp).unwrap();
                info!("json_out: {}", json_out);
                assert_eq!(json_out, r#"1"#);

                let msg_out: T = from_str(&json_out).unwrap();
                info!("msg_out: {:?}", msg_out);
                assert_eq!(msg_out, msg_inp);
            }
        }
    }
    pub mod customer_type {
        use super::*;
        char_ascii!(CustomerType,  #[derive(ByteSerializeStack, ByteDeserializeSlice, ByteSerializedSizeOf, ByteSerializedLenOf, PartialEq, Clone, Copy)]);
        option_tag!(CustomerType, 4);
        char_ascii_into_tag_value!(CustomerType);

        #[rustfmt::skip]
        impl CustomerType{
            pub fn retail() -> Self{ CustomerType(b'R') }
            pub fn non_retail() -> Self{ CustomerType(b'N') }
            pub fn port_default() -> Self{ CustomerType(b' ') }
            pub fn is_retail(&self) -> bool{ self.0 == b'R' }
            pub fn is_non_retail(&self) -> bool{ self.0 == b'N' }
            pub fn is_port_default(&self) -> bool{ self.0 == b' ' }

        }
        impl Default for CustomerType {
            /// Space, Port Default
            #[inline(always)]
            fn default() -> Self {
                CustomerType::port_default()
            }
        }
        impl Serialize for CustomerType {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: Serializer,
            {
                if self.is_retail() {
                    serializer.serialize_str("RETAIL")
                } else if self.is_non_retail() {
                    serializer.serialize_str("NON_RETAIL")
                } else if self.is_port_default() {
                    serializer.serialize_str("PORT_DEFAULT")
                } else {
                    serializer.serialize_str("UNKNOWN")
                }
            }
        }
        impl<'de> Deserialize<'de> for CustomerType {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: Deserializer<'de>,
            {
                let value = String::deserialize(deserializer)?.to_uppercase();
                match value.as_str() {
                    "RETAIL" | "R" => Ok(CustomerType::retail()),
                    "NON_RETAIL" | "N" => Ok(CustomerType::non_retail()),
                    "PORT_DEFAULT" | " " => Ok(CustomerType::port_default()),
                    _ => panic!("Unknown value for {}: {}", short_type_name::<Self>(), value),
                }
            }
        }
        #[cfg(test)]
        mod test {
            use crate::prelude::*;
            use links_core::unittest::setup;
            use log::info;
            use serde_json::{from_str, to_string};
            #[test]
            fn test_msg_serde() {
                setup::log::configure_compact(log::LevelFilter::Info);
                type T = TagValueElement<CustomerType>;
                let msg_inp: T = b'N'.into();
                info!("msg_inp: {:?}", msg_inp);

                let json_out = to_string(&msg_inp).unwrap();
                info!("json_out: {}", json_out);
                assert_eq!(json_out, r#""NON_RETAIL""#);

                let msg_out: T = from_str(&json_out).unwrap();
                info!("msg_out: {:?}", msg_out);
                assert_eq!(msg_out, msg_inp);
            }
        }
    }
    pub mod max_floor {
        use super::*;
        u32_tuple!(MaxFloor, "be", #[derive(ByteSerializeStack, ByteDeserializeSlice, ByteSerializedSizeOf, ByteSerializedLenOf, Serialize, Deserialize, PartialEq, Debug, Clone, Copy)]);
        option_tag!(MaxFloor, 5);
        numeric_into_tag_value!(MaxFloor, u32);

        impl Default for MaxFloor {
            /// Zero, no max floor
            #[inline(always)]
            fn default() -> Self {
                MaxFloor(0)
            }
        }
        #[cfg(test)]
        mod test {
            use crate::prelude::*;
            use links_core::unittest::setup;
            use log::info;
            use serde_json::{from_str, to_string};
            #[test]
            fn test_msg_serde() {
                setup::log::configure_compact(log::LevelFilter::Info);
                type T = TagValueElement<MaxFloor>;
                let msg_inp: T = 1.into();
                info!("msg_inp: {:?}", msg_inp);

                let json_out = to_string(&msg_inp).unwrap();
                info!("json_out: {}", json_out);
                assert_eq!(json_out, r#"1"#);

                let msg_out: T = from_str(&json_out).unwrap();
                info!("msg_out: {:?}", msg_out);
                assert_eq!(msg_out, msg_inp);
            }
        }
    }
    pub mod price_type {
        use super::*;
        char_ascii!(PriceType, #[derive(ByteSerializeStack, ByteDeserializeSlice, ByteSerializedSizeOf, ByteSerializedLenOf, PartialEq, Clone, Copy)]);
        option_tag!(PriceType, 6);
        char_ascii_into_tag_value!(PriceType);

        #[rustfmt::skip]
        impl PriceType{
            pub fn limit() -> Self{ PriceType(b'L') }
            pub fn market_peg() -> Self{ PriceType(b'P') }
            pub fn mid_point_peg() -> Self{ PriceType(b'M') }
            pub fn primary_peg() -> Self{ PriceType(b'R') }
            pub fn market_maker_peg() -> Self{ PriceType(b'Q') }
            pub fn is_limit(&self) -> bool{ self.0 == b'L' }
            pub fn is_market_peg(&self) -> bool{ self.0 == b'P' }
            pub fn is_mid_point_peg(&self) -> bool{ self.0 == b'M' }
            pub fn is_primary_peg(&self) -> bool{ self.0 == b'R' }
            pub fn is_market_maker_peg(&self) -> bool{ self.0 == b'Q' }

        }
        impl Default for PriceType {
            /// 'L', Limit
            #[inline(always)]
            fn default() -> Self {
                PriceType::limit()
            }
        }
        impl Serialize for PriceType {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: Serializer,
            {
                if self.is_limit() {
                    serializer.serialize_str("LIMIT")
                } else if self.is_market_peg() {
                    serializer.serialize_str("MARKET_PEG")
                } else if self.is_mid_point_peg() {
                    serializer.serialize_str("MID_POINT_PEG")
                } else if self.is_primary_peg() {
                    serializer.serialize_str("PRIMARY_PEG")
                } else if self.is_market_maker_peg() {
                    serializer.serialize_str("MARKET_MAKER_PEG")
                } else {
                    serializer.serialize_str("UNKNOWN")
                }
            }
        }
        impl<'de> Deserialize<'de> for PriceType {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: Deserializer<'de>,
            {
                let value = String::deserialize(deserializer)?.to_uppercase();
                match value.as_str() {
                    "LIMIT" | "L" => Ok(PriceType::limit()),
                    "MARKET_PEG" | "P" => Ok(PriceType::market_peg()),
                    "MID_POINT_PEG" | "M" => Ok(PriceType::mid_point_peg()),
                    "PRIMARY_PEG" | "R" => Ok(PriceType::primary_peg()),
                    "MARKET_MAKER_PEG" | "Q" => Ok(PriceType::market_maker_peg()),
                    _ => panic!("Unknown value for {}: {}", short_type_name::<Self>(), value),
                }
            }
        }
        #[cfg(test)]
        mod test {
            use crate::prelude::*;
            use links_core::unittest::setup;
            use log::info;
            use serde_json::{from_str, to_string};
            #[test]
            fn test_msg_serde() {
                setup::log::configure_compact(log::LevelFilter::Info);
                type T = TagValueElement<PriceType>;
                let msg_inp: T = b'L'.into();
                info!("msg_inp: {:?}", msg_inp);

                let json_out = to_string(&msg_inp).unwrap();
                info!("json_out: {}", json_out);
                assert_eq!(json_out, r#""LIMIT""#);

                let msg_out: T = from_str(&json_out).unwrap();
                info!("msg_out: {:?}", msg_out);
                assert_eq!(msg_out, msg_inp);
            }
        }
    }
    // THIS IS A SIGNED PRICE
    pub mod peg_offset {
        use super::*;

        f32_tuple!(PegOffset, "be", #[derive(ByteSerializeStack, ByteDeserializeSlice, ByteSerializedSizeOf, ByteSerializedLenOf, Serialize, Deserialize, PartialEq, Debug, Clone, Copy)]);
        option_tag!(PegOffset, 7);
        numeric_into_tag_value!(PegOffset, f32);

        impl Default for PegOffset {
            /// Zero, no peg offset
            #[inline(always)]
            fn default() -> Self {
                PegOffset(0.0)
            }
        }

        #[cfg(test)]
        mod test {
            use crate::prelude::*;
            use links_core::unittest::setup;
            use log::info;
            use serde_json::{from_str, to_string};
            #[test]
            fn test_msg_serde() {
                setup::log::configure_compact(log::LevelFilter::Info);
                type T = TagValueElement<PegOffset>;
                let msg_inp: T = (-1.1234).into();
                info!("msg_inp: {:?}", msg_inp);

                let json_out = to_string(&msg_inp).unwrap();
                info!("json_out: {}", json_out);
                assert_eq!(json_out, r#"-1.1234"#);

                let msg_out: T = from_str(&json_out).unwrap();
                info!("msg_out: {:?}", msg_out);
                assert_eq!(msg_out, msg_inp);
            }
        }
    }
    // THIS IS A UN-SIGNED PRICE
    pub mod discretion_price {
        use super::*;
        use links_core::core::macros::short_type_name;

        u64_tuple!(DiscretionPrice, "be", #[derive(ByteSerializeStack, ByteDeserializeSlice, ByteSerializedSizeOf, ByteSerializedLenOf, PartialEq, Clone, Copy)]);
        option_tag!(DiscretionPrice, 9);
        numeric_into_tag_value!(DiscretionPrice, u64);
        f32_unsigned_price_u64!(DiscretionPrice);
        impl Default for DiscretionPrice {
            /// Zero, no discretion price
            #[inline(always)]
            fn default() -> Self {
                DiscretionPrice(0)
            }
        }

        #[cfg(test)]
        mod test {
            use crate::prelude::*;
            use links_core::unittest::setup;
            use log::info;
            use serde_json::{from_str, to_string};
            #[test]
            fn test_msg_serde() {
                setup::log::configure_compact(log::LevelFilter::Info);
                type T = TagValueElement<DiscretionPrice>;
                let msg_inp: T = 1.1234.into();
                info!("msg_inp: {:?}", msg_inp);

                let json_out = to_string(&msg_inp).unwrap();
                info!("json_out: {}", json_out);
                assert_eq!(json_out, r#"1.1234"#);

                let msg_out: T = from_str(&json_out).unwrap();
                info!("msg_out: {:?}", msg_out);
                assert_eq!(msg_out, msg_inp);
            }
            #[test]
            #[should_panic]
            fn test_msg_fail() {
                let _ = DiscretionPrice::from(-1.1234);
            }
        }
    }
    pub mod discretion_price_type {
        use super::*;
        char_ascii!(DiscretionPriceType, #[derive(ByteSerializeStack, ByteDeserializeSlice, ByteSerializedSizeOf, ByteSerializedLenOf, PartialEq, Clone, Copy)]);
        option_tag!(DiscretionPriceType, 10);
        char_ascii_into_tag_value!(DiscretionPriceType);

        #[rustfmt::skip]
        impl DiscretionPriceType{
            pub fn limit() -> Self{ DiscretionPriceType(b'L') }
            pub fn market_peg() -> Self{ DiscretionPriceType(b'P') }
            pub fn mid_point_peg() -> Self{ DiscretionPriceType(b'M') }
            pub fn primary_peg() -> Self{ DiscretionPriceType(b'R') }
            pub fn is_limit(&self) -> bool{ self.0 == b'L' }
            pub fn is_market_peg(&self) -> bool{ self.0 == b'P' }
            pub fn is_mid_point_peg(&self) -> bool{ self.0 == b'M' }
            pub fn is_primary_peg(&self) -> bool{ self.0 == b'R' }
        }
        impl Default for DiscretionPriceType {
            /// 'L', Limit
            #[inline(always)]
            fn default() -> Self {
                DiscretionPriceType::limit()
            }
        }
        impl Serialize for DiscretionPriceType {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: Serializer,
            {
                if self.is_limit() {
                    serializer.serialize_str("LIMIT")
                } else if self.is_market_peg() {
                    serializer.serialize_str("MARKET_PEG")
                } else if self.is_mid_point_peg() {
                    serializer.serialize_str("MID_POINT_PEG")
                } else if self.is_primary_peg() {
                    serializer.serialize_str("PRIMARY_PEG")
                } else {
                    serializer.serialize_str("UNKNOWN")
                }
            }
        }
        impl<'de> Deserialize<'de> for DiscretionPriceType {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: Deserializer<'de>,
            {
                let value = String::deserialize(deserializer)?.to_uppercase();
                match value.as_str() {
                    "LIMIT" | "L" => Ok(DiscretionPriceType::limit()),
                    "MARKET_PEG" | "P" => Ok(DiscretionPriceType::market_peg()),
                    "MID_POINT_PEG" | "M" => Ok(DiscretionPriceType::mid_point_peg()),
                    "PRIMARY_PEG" | "R" => Ok(DiscretionPriceType::primary_peg()),
                    _ => panic!("Unknown value for {}: {}", short_type_name::<Self>(), value),
                }
            }
        }

        #[cfg(test)]
        mod test {
            use crate::prelude::*;
            use links_core::unittest::setup;
            use log::info;
            use serde_json::{from_str, to_string};
            #[test]
            fn test_msg_serde() {
                setup::log::configure_compact(log::LevelFilter::Info);
                type T = TagValueElement<DiscretionPriceType>;
                let msg_inp: T = b'L'.into();
                info!("msg_inp: {:?}", msg_inp);

                let json_out = to_string(&msg_inp).unwrap();
                info!("json_out: {}", json_out);
                assert_eq!(json_out, r#""LIMIT""#);

                let msg_out: T = from_str(&json_out).unwrap();
                info!("msg_out: {:?}", msg_out);
                assert_eq!(msg_out, msg_inp);
            }
        }
    }
    // THIS IS A SIGNED PRICE
    pub mod discretion_peg_offset {
        use super::*;

        f32_tuple!(DiscretionPegOffset, "be", #[derive(ByteSerializeStack, ByteDeserializeSlice, ByteSerializedSizeOf, ByteSerializedLenOf, Serialize, Deserialize, PartialEq, Debug, Clone, Copy)]);
        option_tag!(DiscretionPegOffset, 11);
        numeric_into_tag_value!(DiscretionPegOffset, f32);

        impl Default for DiscretionPegOffset {
            /// Zero, no offset
            #[inline(always)]
            fn default() -> Self {
                DiscretionPegOffset(0.0)
            }
        }
        #[cfg(test)]
        mod test {
            use crate::prelude::*;
            use links_core::unittest::setup;
            use log::info;
            use serde_json::{from_str, to_string};
            #[test]
            fn test_msg_serde() {
                setup::log::configure_compact(log::LevelFilter::Info);
                type T = TagValueElement<DiscretionPegOffset>;
                let msg_inp: T = (-1.1234).into();
                info!("msg_inp: {:?}", msg_inp);

                let json_out = to_string(&msg_inp).unwrap();
                info!("json_out: {}", json_out);
                assert_eq!(json_out, r#"-1.1234"#);

                let msg_out: T = from_str(&json_out).unwrap();
                info!("msg_out: {:?}", msg_out);
                assert_eq!(msg_out, msg_inp);
            }
        }
    }
    pub mod post_only {
        use std::char;

        use super::*;
        char_ascii!(PostOnly, #[derive(ByteSerializeStack, ByteDeserializeSlice, ByteSerializedSizeOf, ByteSerializedLenOf, PartialEq, Clone, Copy)]);
        option_tag!(PostOnly, 12);
        char_ascii_into_tag_value!(PostOnly);

        #[rustfmt::skip]
        impl PostOnly{
            pub fn yes() -> Self{ PostOnly(b'P') }
            pub fn no() -> Self{ PostOnly(b'N') }
            pub fn is_yes(&self) -> bool { self.0 == b'P' }
            pub fn is_no(&self) -> bool { self.0 == b'N' }
        }
        impl Default for PostOnly {
            /// 'N', No Post
            #[inline(always)]
            fn default() -> Self {
                PostOnly::no()
            }
        }
        impl Serialize for PostOnly {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: Serializer,
            {
                if self.is_yes() {
                    serializer.serialize_str("YES")
                } else if self.is_no() {
                    serializer.serialize_str("NO")
                } else {
                    serializer.serialize_str("UNKNOWN")
                }
            }
        }
        impl<'de> Deserialize<'de> for PostOnly {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: Deserializer<'de>,
            {
                let value = String::deserialize(deserializer)?.to_uppercase();
                match value.as_str() {
                    "YES" | "Y" => Ok(PostOnly::yes()),
                    "NO" | "N" => Ok(PostOnly::no()),
                    _ => panic!("Unknown value for {}: {}", short_type_name::<Self>(), value),
                }
            }
        }

        #[cfg(test)]
        mod test {
            use crate::prelude::*;
            use links_core::unittest::setup;
            use log::info;
            use serde_json::{from_str, to_string};
            #[test]
            fn test_msg_serde() {
                setup::log::configure_compact(log::LevelFilter::Info);
                type T = TagValueElement<PostOnly>;
                let msg_inp: T = b'P'.into();
                info!("msg_inp: {:?}", msg_inp);

                let json_out = to_string(&msg_inp).unwrap();
                info!("json_out: {}", json_out);
                assert_eq!(json_out, r#""YES""#);

                let msg_out: T = from_str(&json_out).unwrap();
                info!("msg_out: {:?}", msg_out);
                assert_eq!(msg_out, msg_inp);
            }
        }
    }
    pub mod random_reserves {
        use super::*;
        u32_tuple!(RandomReserves, "be", #[derive(ByteSerializeStack, ByteDeserializeSlice, ByteSerializedSizeOf, ByteSerializedLenOf, Serialize, Deserialize, PartialEq, Debug, Clone, Copy)]);
        option_tag!(RandomReserves, 13);
        numeric_into_tag_value!(RandomReserves, u32);

        impl Default for RandomReserves {
            /// Zero, no random reserves
            #[inline(always)]
            fn default() -> Self {
                RandomReserves(0)
            }
        }

        #[cfg(test)]
        mod test {
            use crate::prelude::*;
            use links_core::unittest::setup;
            use log::info;
            use serde_json::{from_str, to_string};
            #[test]
            fn test_msg_serde() {
                setup::log::configure_compact(log::LevelFilter::Info);
                type T = TagValueElement<RandomReserves>;
                let msg_inp: T = 1.into();
                info!("msg_inp: {:?}", msg_inp);

                let json_out = to_string(&msg_inp).unwrap();
                info!("json_out: {}", json_out);
                assert_eq!(json_out, r#"1"#);

                let msg_out: T = from_str(&json_out).unwrap();
                info!("msg_out: {:?}", msg_out);
                assert_eq!(msg_out, msg_inp);
            }
        }
    }
    pub mod route {
        use super::*;
        string_ascii_fixed!(Route, 4, b' ', true, true, #[derive(ByteSerializeStack, ByteDeserializeSlice, ByteSerializedSizeOf, ByteSerializedLenOf, PartialEq, Clone, Copy)]);
        option_tag!(Route, 14);
        string_ascii_fixed_into_tag_value!(Route);

        #[cfg(test)]
        mod test {
            use crate::prelude::*;
            use links_core::unittest::setup;
            use log::info;
            use serde_json::{from_str, to_string};
            #[test]
            fn test_msg_serde() {
                setup::log::configure_compact(log::LevelFilter::Info);
                type T = TagValueElement<Route>;
                let msg_inp: T = b"ABCD".into();
                info!("msg_inp: {:?}", msg_inp);
                let msg_inp: T = b"AB".as_slice().into();
                info!("msg_inp: {:?}", msg_inp);

                let json_out = to_string(&msg_inp).unwrap();
                info!("json_out: {}", json_out);
                assert_eq!(json_out, r#""AB""#);

                let msg_out: T = from_str(&json_out).unwrap();
                info!("msg_out: {:?}", msg_out);
                assert_eq!(msg_out, msg_inp);
            }
        }
    }
    pub mod expire_time {
        use super::*;
        u32_tuple!(ExpireTime, "be", #[derive(ByteSerializeStack, ByteDeserializeSlice, ByteSerializedSizeOf, ByteSerializedLenOf, Serialize, Deserialize, PartialEq, Debug, Clone, Copy)]);
        option_tag!(ExpireTime, 15);
        numeric_into_tag_value!(ExpireTime, u32);

        impl Default for ExpireTime {
            /// Zero, no expire time
            #[inline(always)]
            fn default() -> Self {
                ExpireTime(0)
            }
        }
        #[cfg(test)]
        mod test {
            use crate::prelude::*;
            use links_core::unittest::setup;
            use log::info;
            use serde_json::{from_str, to_string};
            #[test]
            fn test_msg_serde() {
                setup::log::configure_compact(log::LevelFilter::Info);
                type T = TagValueElement<ExpireTime>;
                let msg_inp: T = 1.into();
                info!("msg_inp: {:?}", msg_inp);

                let json_out = to_string(&msg_inp).unwrap();
                info!("json_out: {}", json_out);
                assert_eq!(json_out, r#"1"#);

                let msg_out: T = from_str(&json_out).unwrap();
                info!("msg_out: {:?}", msg_out);
                assert_eq!(msg_out, msg_inp);
            }
        }
    }
    pub mod trade_now {
        use super::*;
        char_ascii!(TradeNow, #[derive(ByteSerializeStack, ByteDeserializeSlice, ByteSerializedSizeOf, ByteSerializedLenOf, PartialEq, Clone, Copy)]);
        option_tag!(TradeNow, 16);
        char_ascii_into_tag_value!(TradeNow);

        #[rustfmt::skip]
        impl TradeNow{
            pub fn yes() -> Self{ TradeNow(b'Y') }
            pub fn no() -> Self{ TradeNow(b'N') }
            pub fn port_default() -> Self{ TradeNow(b' ') }
            pub fn is_yes(&self) -> bool{ self.0 == b'Y' }
            pub fn is_no(&self) -> bool{ self.0 == b'N' }
            pub fn is_port_default(&self) -> bool{ self.0 == b' ' }
        }
        impl Default for TradeNow {
            /// Space, port default
            #[inline(always)]
            fn default() -> Self {
                TradeNow::port_default()
            }
        }
        impl Serialize for TradeNow {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: Serializer,
            {
                if self.is_yes() {
                    serializer.serialize_str("YES")
                } else if self.is_no() {
                    serializer.serialize_str("NO")
                } else if self.is_port_default() {
                    serializer.serialize_str("PORT_DEFAULT")
                } else {
                    serializer.serialize_str("UNKNOWN")
                }
            }
        }
        impl<'de> Deserialize<'de> for TradeNow {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: Deserializer<'de>,
            {
                let value = String::deserialize(deserializer)?.to_uppercase();
                match value.as_str() {
                    "YES" | "Y" => Ok(TradeNow::yes()),
                    "NO" | "N" => Ok(TradeNow::no()),
                    "PORT_DEFAULT" | " " => Ok(TradeNow::port_default()),
                    _ => panic!("Unknown value for {}: {}", short_type_name::<Self>(), value),
                }
            }
        }

        #[cfg(test)]
        mod test {
            use crate::prelude::*;
            use links_core::unittest::setup;
            use log::info;
            use serde_json::{from_str, to_string};
            #[test]
            fn test_msg_serde() {
                setup::log::configure_compact(log::LevelFilter::Info);
                type T = TagValueElement<TradeNow>;
                let msg_inp: T = b'Y'.into();
                info!("msg_inp: {:?}", msg_inp);

                let json_out = to_string(&msg_inp).unwrap();
                info!("json_out: {}", json_out);
                assert_eq!(json_out, r#""YES""#);

                let msg_out: T = from_str(&json_out).unwrap();
                info!("msg_out: {:?}", msg_out);
                assert_eq!(msg_out, msg_inp);
            }
        }
    }
    pub mod handle_inst {
        use super::*;
        char_ascii!(HandleInst, #[derive(ByteSerializeStack, ByteDeserializeSlice, ByteSerializedSizeOf, ByteSerializedLenOf, PartialEq, Clone, Copy)]);
        option_tag!(HandleInst, 17);
        char_ascii_into_tag_value!(HandleInst);

        #[rustfmt::skip]
        impl HandleInst{
            pub fn no_instructions() -> Self{ HandleInst(b' ') }
            pub fn imbalance_only() -> Self{ HandleInst(b'I') }
            pub fn retail_order_type_1() -> Self{ HandleInst(b'O') }
            pub fn retail_order_type_2() -> Self{ HandleInst(b'T') }
            pub fn retail_price_improvement() -> Self{ HandleInst(b'Q') }
            pub fn extended_life_continuous() -> Self{ HandleInst(b'B') }
            pub fn direct_listing_capital_raise() -> Self{ HandleInst(b'D') }
            pub fn hidden_price_improvement() -> Self{ HandleInst(b'R') }
            pub fn is_no_instructions(&self) -> bool{ self.0 == b' ' }
            pub fn is_imbalance_only(&self) -> bool{ self.0 == b'I' }
            pub fn is_retail_order_type_1(&self) -> bool{ self.0 == b'O' }
            pub fn is_retail_order_type_2(&self) -> bool{ self.0 == b'T' }
            pub fn is_retail_price_improvement(&self) -> bool{ self.0 == b'Q' }
            pub fn is_extended_life_continuous(&self) -> bool{ self.0 == b'B' }
            pub fn is_direct_listing_capital_raise(&self) -> bool{ self.0 == b'D' }
            pub fn is_hidden_price_improvement(&self) -> bool{ self.0 == b'R' }
        }
        impl Default for HandleInst {
            /// Space, No Instructions
            #[inline(always)]
            fn default() -> Self {
                HandleInst::no_instructions()
            }
        }
        impl Serialize for HandleInst {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: Serializer,
            {
                if self.is_no_instructions() {
                    serializer.serialize_str("NO_INSTRUCTIONS")
                } else if self.is_imbalance_only() {
                    serializer.serialize_str("IMBALANCE_ONLY")
                } else if self.is_retail_order_type_1() {
                    serializer.serialize_str("RETAIL_ORDER_TYPE_1")
                } else if self.is_retail_order_type_2() {
                    serializer.serialize_str("RETAIL_ORDER_TYPE_2")
                } else if self.is_retail_price_improvement() {
                    serializer.serialize_str("RETAIL_PRICE_IMPROVEMENT")
                } else if self.is_extended_life_continuous() {
                    serializer.serialize_str("EXTENDED_LIFE_CONTINUOUS")
                } else if self.is_direct_listing_capital_raise() {
                    serializer.serialize_str("DIRECT_LISTING_CAPITAL_RAISE")
                } else if self.is_hidden_price_improvement() {
                    serializer.serialize_str("HIDDEN_PRICE_IMPROVEMENT")
                } else {
                    serializer.serialize_str("UNKNOWN")
                }
            }
        }
        impl<'de> Deserialize<'de> for HandleInst {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: Deserializer<'de>,
            {
                let value = String::deserialize(deserializer)?.to_uppercase();
                match value.as_str() {
                    "NO_INSTRUCTIONS" | " " => Ok(HandleInst::no_instructions()),
                    "IMBALANCE_ONLY" | "I" => Ok(HandleInst::imbalance_only()),
                    "RETAIL_ORDER_TYPE_1" | "O" => Ok(HandleInst::retail_order_type_1()),
                    "RETAIL_ORDER_TYPE_2" | "T" => Ok(HandleInst::retail_order_type_2()),
                    "RETAIL_PRICE_IMPROVEMENT" | "Q" => Ok(HandleInst::retail_price_improvement()),
                    "EXTENDED_LIFE_CONTINUOUS" | "B" => Ok(HandleInst::extended_life_continuous()),
                    "DIRECT_LISTING_CAPITAL_RAISE" | "D" => Ok(HandleInst::direct_listing_capital_raise()),
                    "HIDDEN_PRICE_IMPROVEMENT" | "R" => Ok(HandleInst::hidden_price_improvement()),
                    _ => panic!("Unknown value for {}: {}", short_type_name::<Self>(), value),
                }
            }
        }

        #[cfg(test)]
        mod test {
            use crate::prelude::*;
            use links_core::unittest::setup;
            use log::info;
            use serde_json::{from_str, to_string};

            #[test]
            fn test_msg_serde() {
                setup::log::configure_compact(log::LevelFilter::Info);
                type T = TagValueElement<HandleInst>;
                let msg_inp: T = b'I'.into();
                info!("msg_inp: {:?}", msg_inp);

                let json_out = to_string(&msg_inp).unwrap();
                info!("json_out: {}", json_out);
                assert_eq!(json_out, r#""IMBALANCE_ONLY""#);

                let msg_out: T = from_str(&json_out).unwrap();
                info!("msg_out: {:?}", msg_out);
                assert_eq!(msg_out, msg_inp);
            }
        }
    }
    pub mod bbo_weight_indicator {
        use super::*;
        char_ascii!(BBOWeightIndicator, #[derive(ByteSerializeStack, ByteDeserializeSlice, ByteSerializedSizeOf, ByteSerializedLenOf, PartialEq, Clone, Copy)]);
        option_tag!(BBOWeightIndicator, 18);
        char_ascii_into_tag_value!(BBOWeightIndicator);

        #[rustfmt::skip]
        impl BBOWeightIndicator{
            pub fn zero_point_2() -> Self{ BBOWeightIndicator(b'0') }
            pub fn point_2_one() -> Self{ BBOWeightIndicator(b'1') }
            pub fn one_two() -> Self{ BBOWeightIndicator(b'2') }
            pub fn two_above() -> Self{ BBOWeightIndicator(b'3') }
            pub fn unspecified() -> Self{ BBOWeightIndicator(b' ') }
            pub fn sets_qbbo_while_joining_nbbo() -> Self{ BBOWeightIndicator(b'S') }
            pub fn improves_nbbo_upon_entry() -> Self{ BBOWeightIndicator(b'N') }
            pub fn is_zero_point_2(&self) -> bool{ self.0 == b'0' }
            pub fn is_point_2_one(&self) -> bool{ self.0 == b'1' }
            pub fn is_one_two(&self) -> bool{ self.0 == b'2' }
            pub fn is_two_above(&self) -> bool{ self.0 == b'3' }
            pub fn is_unspecified(&self) -> bool{ self.0 == b' ' }
            pub fn is_sets_qbbo_while_joining_nbbo(&self) -> bool{ self.0 == b'S' }
            pub fn is_improves_nbbo_upon_entry(&self) -> bool{ self.0 == b'N' }
        }
        impl Serialize for BBOWeightIndicator {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: Serializer,
            {
                if self.is_zero_point_2() {
                    serializer.serialize_str("ZERO_POINT_2")
                } else if self.is_point_2_one() {
                    serializer.serialize_str("POINT_2_ONE")
                } else if self.is_one_two() {
                    serializer.serialize_str("ONE_TWO")
                } else if self.is_two_above() {
                    serializer.serialize_str("TWO_ABOVE")
                } else if self.is_unspecified() {
                    serializer.serialize_str("UNSPECIFIED")
                } else if self.is_sets_qbbo_while_joining_nbbo() {
                    serializer.serialize_str("SETS_QBBO_WHILE_JOINING_NBBO")
                } else if self.is_improves_nbbo_upon_entry() {
                    serializer.serialize_str("IMPROVES_NBBO_UPON_ENTRY")
                } else {
                    serializer.serialize_str("UNKNOWN")
                }
            }
        }
        impl<'de> Deserialize<'de> for BBOWeightIndicator {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: Deserializer<'de>,
            {
                let value = String::deserialize(deserializer)?.to_uppercase();
                match value.as_str() {
                    "ZERO_POINT_2" | "0" => Ok(BBOWeightIndicator::zero_point_2()),
                    "POINT_2_ONE" | "1" => Ok(BBOWeightIndicator::point_2_one()),
                    "ONE_TWO" | "2" => Ok(BBOWeightIndicator::one_two()),
                    "TWO_ABOVE" | "3" => Ok(BBOWeightIndicator::two_above()),
                    "UNSPECIFIED" | " " => Ok(BBOWeightIndicator::unspecified()),
                    "SETS_QBBO_WHILE_JOINING_NBBO" | "S" => Ok(BBOWeightIndicator::sets_qbbo_while_joining_nbbo()),
                    "IMPROVES_NBBO_UPON_ENTRY" | "N" => Ok(BBOWeightIndicator::improves_nbbo_upon_entry()),
                    _ => panic!("Unknown value for {}: {}", short_type_name::<Self>(), value),
                }
            }
        }

        #[cfg(test)]
        mod test {
            use crate::prelude::*;
            use links_core::unittest::setup;
            use log::info;
            use serde_json::{from_str, to_string};

            #[test]
            fn test_msg_serde() {
                setup::log::configure_compact(log::LevelFilter::Info);
                type T = TagValueElement<BBOWeightIndicator>;
                let msg_inp: T = b'0'.into();
                info!("msg_inp: {:?}", msg_inp);

                let json_out = to_string(&msg_inp).unwrap();
                info!("json_out: {}", json_out);
                assert_eq!(json_out, r#""ZERO_POINT_2""#);

                let msg_out: T = from_str(&json_out).unwrap();
                info!("msg_out: {:?}", msg_out);
                assert_eq!(msg_out, msg_inp);
            }
        }
    }
    pub mod display_qty {
        use super::*;
        u32_tuple!(DisplayQty, "be", #[derive(ByteSerializeStack, ByteDeserializeSlice, ByteSerializedSizeOf, ByteSerializedLenOf, Serialize, Deserialize, PartialEq, Debug, Clone, Copy)]);
        option_tag!(DisplayQty, 22);
        numeric_into_tag_value!(DisplayQty, u32);

        #[cfg(test)]
        mod test {
            use crate::prelude::*;
            use links_core::unittest::setup;
            use log::info;
            use serde_json::{from_str, to_string};

            #[test]
            fn test_msg_serde() {
                setup::log::configure_compact(log::LevelFilter::Info);
                type T = TagValueElement<DisplayQty>;
                let msg_inp: T = 1.into();
                info!("msg_inp: {:?}", msg_inp);

                let json_out = to_string(&msg_inp).unwrap();
                info!("json_out: {}", json_out);
                assert_eq!(json_out, r#"1"#);

                let msg_out: T = from_str(&json_out).unwrap();
                info!("msg_out: {:?}", msg_out);
                assert_eq!(msg_out, msg_inp);
            }
        }
    }
    // THIS IS A UN-SIGNED PRICE
    pub mod display_price {
        use super::*;
        u64_tuple!(DisplayPrice, "be", #[derive(ByteSerializeStack, ByteDeserializeSlice, ByteSerializedSizeOf, ByteSerializedLenOf, PartialEq, Clone, Copy)]);
        option_tag!(DisplayPrice, 23);
        numeric_into_tag_value!(DisplayPrice, u64);
        f32_unsigned_price_u64!(DisplayPrice);

        #[cfg(test)]
        mod test {
            use crate::prelude::*;
            use links_core::unittest::setup;
            use log::info;
            use serde_json::{from_str, to_string};

            #[test]
            fn test_msg_serde() {
                setup::log::configure_compact(log::LevelFilter::Info);
                type T = TagValueElement<DisplayPrice>;
                let msg_inp: T = 1.1234.into();
                info!("msg_inp: {:?}", msg_inp);

                let json_out = to_string(&msg_inp).unwrap();
                info!("json_out: {}", json_out);
                assert_eq!(json_out, "1.1234");

                let msg_out: T = from_str(&json_out).unwrap();
                info!("msg_out: {:?}", msg_out);
                assert_eq!(msg_out, msg_inp);
            }
        }
    }
    pub mod group_id {
        use super::*;
        u16_tuple!(GroupId, "be", #[derive(ByteSerializeStack, ByteDeserializeSlice, ByteSerializedSizeOf, ByteSerializedLenOf, Serialize, Deserialize, PartialEq, Debug, Clone, Copy)]);
        option_tag!(GroupId, 24);
        numeric_into_tag_value!(GroupId, u16);
        impl Default for GroupId {
            /// Zero, no group id
            #[inline(always)]
            fn default() -> Self {
                GroupId(0)
            }
        }

        #[cfg(test)]
        mod test {
            use crate::prelude::*;
            use links_core::unittest::setup;
            use log::info;
            use serde_json::{from_str, to_string};

            #[test]
            fn test_msg_serde() {
                setup::log::configure_compact(log::LevelFilter::Info);
                type T = TagValueElement<GroupId>;
                let msg_inp: T = 1.into();
                info!("msg_inp: {:?}", msg_inp);

                let json_out = to_string(&msg_inp).unwrap();
                info!("json_out: {}", json_out);
                assert_eq!(json_out, r#"1"#);

                let msg_out: T = from_str(&json_out).unwrap();
                info!("msg_out: {:?}", msg_out);
                assert_eq!(msg_out, msg_inp);
            }
        }
    }
    pub mod shares_located {
        use super::*;
        char_ascii!(SharesLocated, #[derive(ByteSerializeStack, ByteDeserializeSlice, ByteSerializedSizeOf, ByteSerializedLenOf, PartialEq, Clone, Copy)]);
        option_tag!(SharesLocated, 25);
        char_ascii_into_tag_value!(SharesLocated);

        #[rustfmt::skip]
        impl SharesLocated{
            pub fn yes() -> Self{ SharesLocated(b'Y') }
            pub fn no() -> Self{ SharesLocated(b'N') }
            pub fn is_yes(&self) -> bool{ self.0 == b'Y' }
            pub fn is_no(&self) -> bool{ self.0 == b'N' }
        }
        impl Default for SharesLocated {
            /// 'N', No
            #[inline(always)]
            fn default() -> Self {
                SharesLocated::no()
            }
        }
        impl Serialize for SharesLocated {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: Serializer,
            {
                if self.is_yes() {
                    serializer.serialize_str("YES")
                } else if self.is_no() {
                    serializer.serialize_str("NO")
                } else {
                    serializer.serialize_str("UNKNOWN")
                }
            }
        }
        impl<'de> Deserialize<'de> for SharesLocated {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: Deserializer<'de>,
            {
                let value = String::deserialize(deserializer)?.to_uppercase();
                match value.as_str() {
                    "YES" | "Y" => Ok(SharesLocated::yes()),
                    "NO" | "N" => Ok(SharesLocated::no()),
                    _ => panic!("Unknown value for {}: {}", short_type_name::<Self>(), value),
                }
            }
        }

        #[cfg(test)]
        mod test {
            use crate::prelude::*;
            use links_core::unittest::setup;
            use log::info;
            use serde_json::{from_str, to_string};

            #[test]
            fn test_msg_serde() {
                setup::log::configure_compact(log::LevelFilter::Info);
                type T = TagValueElement<SharesLocated>;
                let msg_inp: T = b'Y'.into();
                info!("msg_inp: {:?}", msg_inp);

                let json_out = to_string(&msg_inp).unwrap();
                info!("json_out: {}", json_out);
                assert_eq!(json_out, r#""YES""#);

                let msg_out: T = from_str(&json_out).unwrap();
                info!("msg_out: {:?}", msg_out);
                assert_eq!(msg_out, msg_inp);
            }
        }
    }
}

#[derive(ByteSerializeStack, ByteDeserializeSlice, ByteSerializedLenOf, PartialEq, Serialize, Deserialize, Debug, Clone, Copy)]
// #[serde(transparent)]
#[serde(into = "TagValueElementJsonShadow<T>", from = "TagValueElementJsonShadow<T>")]
pub struct TagValueElement<T: ByteSerializeStack + ByteDeserializeSlice<T> + ByteSerializedLenOf + OptionTag + Clone>(
    // TODO it should not be a requirement to add T bounds if byteserde is fixed to auto include own bounds during derive
    #[serde(skip)]
    #[byteserde(replace( 1 + size_of::<T>() as u8 ))]
    u8,
    #[serde(skip)]
    #[byteserde(replace(T::tag()))]
    u8,
    T,
);
impl<T: ByteSerializeStack + ByteDeserializeSlice<T> + ByteSerializedLenOf + OptionTag + Clone> TagValueElement<T> {
    #[inline(always)]
    pub fn new(value: T) -> Self {
        TagValueElement(
            // remaining value of the TagValueElement
            1 + size_of::<T>() as u8, // NOTE: this only works because all types are tuples with single elements
            T::tag(),
            value,
        )
    }
    #[inline(always)]
    pub fn value(&self) -> &T {
        &self.2
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
struct TagValueElementJsonShadow<T: OptionTag>(T);
impl<T: OptionTag + ByteSerializeStack + ByteDeserializeSlice<T> + ByteSerializedLenOf + Clone> From<TagValueElementJsonShadow<T>> for TagValueElement<T> {
    #[inline(always)]
    fn from(v: TagValueElementJsonShadow<T>) -> Self {
        TagValueElement::new(v.0)
    }
}
impl<T: OptionTag + ByteSerializeStack + ByteDeserializeSlice<T> + ByteSerializedLenOf + Clone> From<TagValueElement<T>> for TagValueElementJsonShadow<T> {
    #[inline(always)]
    fn from(v: TagValueElement<T>) -> Self {
        TagValueElementJsonShadow(v.2)
    }
}
