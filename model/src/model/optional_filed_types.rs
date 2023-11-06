use byteserde::prelude::*;
use byteserde_derive::{ByteDeserializeSlice, ByteSerializeStack, ByteSerializedLenOf, ByteSerializedSizeOf};
use byteserde_types::{char_ascii, f32_tuple, string_ascii_fixed, u16_tuple, u32_tuple, u64_tuple};
use serde::{Deserialize, Serialize};
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
        impl Serialize for $TYPE {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where S: serde::Serializer {
                serializer.serialize_f32(f32::from(self))
            }
        }
        impl<'de> Deserialize<'de> for $TYPE {
            fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                let f = f32::deserialize(deserializer)?;
                Ok(f.into())
            }
        }
        impl Debug for $TYPE {
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
                setup::log::configure_compact();
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
        string_ascii_fixed!(Firm, 4, b' ', true, #[derive(ByteSerializeStack, ByteDeserializeSlice, ByteSerializedSizeOf, ByteSerializedLenOf, PartialEq, Clone, Copy)]);
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
                setup::log::configure_compact();
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
                setup::log::configure_compact();
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
        char_ascii!(CustomerType, true, #[derive(ByteSerializeStack, ByteDeserializeSlice, ByteSerializedSizeOf, ByteSerializedLenOf, PartialEq, Clone, Copy)]);
        option_tag!(CustomerType, 4);
        char_ascii_into_tag_value!(CustomerType);

        #[rustfmt::skip]
        impl CustomerType{
            pub fn retail() -> Self{ CustomerType(b'R') }
            pub fn non_retail() -> Self{ CustomerType(b'N') }
            pub fn port_default() -> Self{ CustomerType(b' ') }
            pub fn is_retail(other: CustomerType) -> bool{ CustomerType(b'R') == other}
            pub fn is_non_retail(other: CustomerType) -> bool{ CustomerType(b'N') == other }
            pub fn is_port_default(other: CustomerType) -> bool{ CustomerType(b' ') == other }
        }
        impl Default for CustomerType {
            /// Space, Port Default
            #[inline(always)]
            fn default() -> Self {
                CustomerType::port_default()
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
                setup::log::configure_compact();
                type T = TagValueElement<CustomerType>;
                let msg_inp: T = b'N'.into();
                info!("msg_inp: {:?}", msg_inp);

                let json_out = to_string(&msg_inp).unwrap();
                info!("json_out: {}", json_out);
                assert_eq!(json_out, r#""N""#);

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
                setup::log::configure_compact();
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
        char_ascii!(PriceType, true, #[derive(ByteSerializeStack, ByteDeserializeSlice, ByteSerializedSizeOf, ByteSerializedLenOf, PartialEq, Clone, Copy)]);
        option_tag!(PriceType, 6);
        char_ascii_into_tag_value!(PriceType);

        #[rustfmt::skip]
        impl PriceType{
            pub fn limit() -> Self{ PriceType(b'L') }
            pub fn market_peg() -> Self{ PriceType(b'P') }
            pub fn mid_point_peg() -> Self{ PriceType(b'M') }
            pub fn primary_peg() -> Self{ PriceType(b'R') }
            pub fn market_maker_peg() -> Self{ PriceType(b'Q') }
            pub fn is_limit(other: PriceType) -> bool{ PriceType(b'L') == other }
            pub fn is_market_peg(other: PriceType) -> bool{ PriceType(b'P') == other }
            pub fn is_mid_point_peg(other: PriceType) -> bool{ PriceType(b'M') == other }
            pub fn is_primary_peg(other: PriceType) -> bool{ PriceType(b'R') == other }
            pub fn is_market_maker_peg(other: PriceType) -> bool{ PriceType(b'Q') == other }
        }
        impl Default for PriceType {
            /// 'L', Limit
            #[inline(always)]
            fn default() -> Self {
                PriceType::limit()
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
                setup::log::configure_compact();
                type T = TagValueElement<PriceType>;
                let msg_inp: T = b'L'.into();
                info!("msg_inp: {:?}", msg_inp);

                let json_out = to_string(&msg_inp).unwrap();
                info!("json_out: {}", json_out);
                assert_eq!(json_out, r#""L""#);

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
                setup::log::configure_compact();
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
        use std::fmt::Debug;

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
                setup::log::configure_compact();
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
        char_ascii!(DiscretionPriceType, true, #[derive(ByteSerializeStack, ByteDeserializeSlice, ByteSerializedSizeOf, ByteSerializedLenOf, PartialEq, Clone, Copy)]);
        option_tag!(DiscretionPriceType, 10);
        char_ascii_into_tag_value!(DiscretionPriceType);

        #[rustfmt::skip]
        impl DiscretionPriceType{
            pub fn limit() -> Self{ DiscretionPriceType(b'L') }
            pub fn market_peg() -> Self{ DiscretionPriceType(b'P') }
            pub fn mid_point_peg() -> Self{ DiscretionPriceType(b'M') }
            pub fn primary_peg() -> Self{ DiscretionPriceType(b'R') }
            pub fn is_limit(other: DiscretionPriceType) -> bool{ DiscretionPriceType(b'L') == other }
            pub fn is_market_peg(other: DiscretionPriceType) -> bool{ DiscretionPriceType(b'P') == other }
            pub fn is_mid_point_peg(other: DiscretionPriceType) -> bool{ DiscretionPriceType(b'M') == other }
            pub fn is_primary_peg(other: DiscretionPriceType) -> bool{ DiscretionPriceType(b'R') == other }
        }
        impl Default for DiscretionPriceType {
            /// 'L', Limit
            #[inline(always)]
            fn default() -> Self {
                DiscretionPriceType::limit()
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
                setup::log::configure_compact();
                type T = TagValueElement<DiscretionPriceType>;
                let msg_inp: T = b'L'.into();
                info!("msg_inp: {:?}", msg_inp);

                let json_out = to_string(&msg_inp).unwrap();
                info!("json_out: {}", json_out);
                assert_eq!(json_out, r#""L""#);

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
                setup::log::configure_compact();
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
        char_ascii!(PostOnly, true, #[derive(ByteSerializeStack, ByteDeserializeSlice, ByteSerializedSizeOf, ByteSerializedLenOf, PartialEq, Clone, Copy)]);
        option_tag!(PostOnly, 12);
        char_ascii_into_tag_value!(PostOnly);

        #[rustfmt::skip]
        impl PostOnly{
            pub fn yes() -> Self{ PostOnly(b'P') }
            pub fn no() -> Self{ PostOnly(b'N') }
            pub fn is_yes(other: PostOnly) -> bool{ PostOnly(b'P') == other }
            pub fn is_no(other: PostOnly) -> bool{ PostOnly(b'N') == other }
        }
        impl Default for PostOnly {
            /// 'N', No Post
            #[inline(always)]
            fn default() -> Self {
                PostOnly::no()
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
                setup::log::configure_compact();
                type T = TagValueElement<PostOnly>;
                let msg_inp: T = b'P'.into();
                info!("msg_inp: {:?}", msg_inp);

                let json_out = to_string(&msg_inp).unwrap();
                info!("json_out: {}", json_out);
                assert_eq!(json_out, r#""P""#);

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
                setup::log::configure_compact();
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
        string_ascii_fixed!(Route, 4, b' ', true, #[derive(ByteSerializeStack, ByteDeserializeSlice, ByteSerializedSizeOf, ByteSerializedLenOf, PartialEq, Clone, Copy)]);
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
                setup::log::configure_compact();
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
        u32_tuple!(ExpireTimeSec, "be", #[derive(ByteSerializeStack, ByteDeserializeSlice, ByteSerializedSizeOf, ByteSerializedLenOf, Serialize, Deserialize, PartialEq, Debug, Clone, Copy)]);
        option_tag!(ExpireTimeSec, 15);
        numeric_into_tag_value!(ExpireTimeSec, u32);

        impl Default for ExpireTimeSec {
            /// Zero, no expire time
            #[inline(always)]
            fn default() -> Self {
                ExpireTimeSec(0)
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
                setup::log::configure_compact();
                type T = TagValueElement<ExpireTimeSec>;
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
        char_ascii!(TradeNow, true, #[derive(ByteSerializeStack, ByteDeserializeSlice, ByteSerializedSizeOf, ByteSerializedLenOf, PartialEq, Clone, Copy)]);
        option_tag!(TradeNow, 16);
        char_ascii_into_tag_value!(TradeNow);

        #[rustfmt::skip]
        impl TradeNow{
            pub fn yes() -> Self{ TradeNow(b'Y') }
            pub fn no() -> Self{ TradeNow(b'N') }
            pub fn port_default() -> Self{ TradeNow(b' ') }
            pub fn is_yes(other: TradeNow) -> bool{ TradeNow(b'Y') == other }
            pub fn is_no(other: TradeNow) -> bool{ TradeNow(b'N') == other }
            pub fn is_port_default(other: TradeNow) -> bool{ TradeNow(b' ') == other }
        }
        impl Default for TradeNow {
            /// Space, port default
            #[inline(always)]
            fn default() -> Self {
                TradeNow::port_default()
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
                setup::log::configure_compact();
                type T = TagValueElement<TradeNow>;
                let msg_inp: T = b'Y'.into();
                info!("msg_inp: {:?}", msg_inp);

                let json_out = to_string(&msg_inp).unwrap();
                info!("json_out: {}", json_out);
                assert_eq!(json_out, r#""Y""#);

                let msg_out: T = from_str(&json_out).unwrap();
                info!("msg_out: {:?}", msg_out);
                assert_eq!(msg_out, msg_inp);
            }
        }
    }
    pub mod handle_inst {
        use super::*;
        char_ascii!(HandleInst, true, #[derive(ByteSerializeStack, ByteDeserializeSlice, ByteSerializedSizeOf, ByteSerializedLenOf, PartialEq, Clone, Copy)]);
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
            pub fn is_imbalance_only(other: HandleInst) -> bool{ HandleInst(b'I') == other }
            pub fn is_retail_order_type_1(other: HandleInst) -> bool{ HandleInst(b'O') == other }
            pub fn is_retail_order_type_2(other: HandleInst) -> bool{ HandleInst(b'T') == other }
            pub fn is_retail_price_improvement(other: HandleInst) -> bool{ HandleInst(b'Q') == other }
            pub fn is_extended_life_continuous(other: HandleInst) -> bool{ HandleInst(b'B') == other }
            pub fn is_direct_listing_capital_raise(other: HandleInst) -> bool{ HandleInst(b'D') == other }
            pub fn is_hidden_price_improvement(other: HandleInst) -> bool{ HandleInst(b'R') == other }
        }
        impl Default for HandleInst {
            /// Space, No Instructions
            #[inline(always)]
            fn default() -> Self {
                HandleInst::no_instructions()
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
                setup::log::configure_compact();
                type T = TagValueElement<HandleInst>;
                let msg_inp: T = b'I'.into();
                info!("msg_inp: {:?}", msg_inp);

                let json_out = to_string(&msg_inp).unwrap();
                info!("json_out: {}", json_out);
                assert_eq!(json_out, r#""I""#);

                let msg_out: T = from_str(&json_out).unwrap();
                info!("msg_out: {:?}", msg_out);
                assert_eq!(msg_out, msg_inp);
            }
        }
    }
    pub mod bbo_weight_indicator {
        use super::*;
        char_ascii!(BBOWeightIndicator, true, #[derive(ByteSerializeStack, ByteDeserializeSlice, ByteSerializedSizeOf, ByteSerializedLenOf, PartialEq, Clone, Copy)]);
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
            pub fn is_zero_point_2(other: BBOWeightIndicator) -> bool{ BBOWeightIndicator(b'0') == other }
            pub fn is_point_2_one(other: BBOWeightIndicator) -> bool{ BBOWeightIndicator(b'1') == other }
            pub fn is_one_two(other: BBOWeightIndicator) -> bool{ BBOWeightIndicator(b'2') == other }
            pub fn is_two_above(other: BBOWeightIndicator) -> bool{ BBOWeightIndicator(b'3') == other }
            pub fn is_unspecified(other: BBOWeightIndicator) -> bool{ BBOWeightIndicator(b' ') == other }
            pub fn is_sets_qbbo_while_joining_nbbo(other: BBOWeightIndicator) -> bool{ BBOWeightIndicator(b'S') == other }
            pub fn is_improves_nbbo_upon_entry(other: BBOWeightIndicator) -> bool{ BBOWeightIndicator(b'N') == other }
        }

        #[cfg(test)]
        mod test {
            use crate::prelude::*;
            use links_core::unittest::setup;
            use log::info;
            use serde_json::{from_str, to_string};

            #[test]
            fn test_msg_serde() {
                setup::log::configure_compact();
                type T = TagValueElement<BBOWeightIndicator>;
                let msg_inp: T = b'0'.into();
                info!("msg_inp: {:?}", msg_inp);

                let json_out = to_string(&msg_inp).unwrap();
                info!("json_out: {}", json_out);
                assert_eq!(json_out, r#""0""#);

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
                setup::log::configure_compact();
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
    pub mod display_price {
        use super::*;
        u64_tuple!(DisplayPrice, "be", #[derive(ByteSerializeStack, ByteDeserializeSlice, ByteSerializedSizeOf, ByteSerializedLenOf, Serialize, Deserialize, PartialEq, Debug, Clone, Copy)]);
        option_tag!(DisplayPrice, 23);
        numeric_into_tag_value!(DisplayPrice, u64);

        #[cfg(test)]
        mod test {
            use crate::prelude::*;
            use links_core::unittest::setup;
            use log::info;
            use serde_json::{from_str, to_string};

            #[test]
            fn test_msg_serde() {
                setup::log::configure_compact();
                type T = TagValueElement<DisplayPrice>;
                let msg_inp: T = 1.into();
                info!("msg_inp: {:?}", msg_inp);

                let json_out = to_string(&msg_inp).unwrap();
                info!("json_out: {}", json_out);
                assert_eq!(json_out, "1");

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
                setup::log::configure_compact();
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
        char_ascii!(SharesLocated, true, #[derive(ByteSerializeStack, ByteDeserializeSlice, ByteSerializedSizeOf, ByteSerializedLenOf, PartialEq, Clone, Copy)]);
        option_tag!(SharesLocated, 25);
        char_ascii_into_tag_value!(SharesLocated);

        #[rustfmt::skip]
        impl SharesLocated{
            pub fn yes() -> Self{ SharesLocated(b'Y') }
            pub fn no() -> Self{ SharesLocated(b'N') }
            pub fn is_yes(other: SharesLocated) -> bool{ SharesLocated(b'Y') == other }
            pub fn is_no(other: SharesLocated) -> bool{ SharesLocated(b'N') == other }
        }
        impl Default for SharesLocated {
            /// 'N', No
            #[inline(always)]
            fn default() -> Self {
                SharesLocated::no()
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
                setup::log::configure_compact();
                type T = TagValueElement<SharesLocated>;
                let msg_inp: T = b'Y'.into();
                info!("msg_inp: {:?}", msg_inp);

                let json_out = to_string(&msg_inp).unwrap();
                info!("json_out: {}", json_out);
                assert_eq!(json_out, r#""Y""#);

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
pub struct TagValueElement<T: ByteSerializeStack+ByteDeserializeSlice<T>+ByteSerializedLenOf+OptionTag+Clone>(
    // TODO it should not be a requirement to add T bounds if byteserde is fixed to auto include own bounds during derive
    #[serde(skip)]
    #[byteserde(replace( 1 + size_of::<T>() as u8 ))]
    u8,
    #[serde(skip)]
    #[byteserde(replace(T::tag()))]
    u8,
    T,
);
impl<T: ByteSerializeStack+ByteDeserializeSlice<T>+ByteSerializedLenOf+OptionTag+Clone> TagValueElement<T> {
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
impl<T: OptionTag+ByteSerializeStack+ByteDeserializeSlice<T>+ByteSerializedLenOf+Clone> From<TagValueElementJsonShadow<T>> for TagValueElement<T> {
    #[inline(always)]
    fn from(v: TagValueElementJsonShadow<T>) -> Self {
        TagValueElement::new(v.0)
    }
}
impl<T: OptionTag+ByteSerializeStack+ByteDeserializeSlice<T>+ByteSerializedLenOf+Clone> From<TagValueElement<T>> for TagValueElementJsonShadow<T> {
    #[inline(always)]
    fn from(v: TagValueElement<T>) -> Self {
        TagValueElementJsonShadow(v.2)
    }
}
