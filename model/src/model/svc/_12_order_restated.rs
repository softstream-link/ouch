use crate::prelude::*;
use byteserde::prelude::*;
use byteserde_derive::{ByteDeserializeSlice, ByteSerializeStack, ByteSerializedLenOf};
use serde::{Deserialize, Serialize};

#[derive(ByteSerializeStack, ByteDeserializeSlice, ByteSerializedLenOf, Serialize, Deserialize, PartialEq, Clone, Debug)]
#[byteserde(endian = "be")]
#[byteserde(peek(1, 1))] // peek(start, len) -> peek one byte after skipping one
pub struct OrderRestatedAppendage {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[byteserde(eq(DisplayQty::tag_as_slice()))]
    pub display_qty: Option<TagValueElement<DisplayQty>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[byteserde(eq(DisplayPrice::tag_as_slice()))]
    pub display_price: Option<TagValueElement<DisplayPrice>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[byteserde(eq(SecondaryOrdRefNum::tag_as_slice()))]
    pub secondary_order_ref_num: Option<TagValueElement<SecondaryOrdRefNum>>,
}

#[derive(ByteSerializeStack, ByteDeserializeSlice, ByteSerializedLenOf, Serialize, Deserialize, PartialEq, Clone, Debug)]
#[byteserde(endian = "be")]
#[serde(from = "OrderRestatedJsonDes")]
pub struct OrderRestated {
    #[serde(default, skip_serializing)]
    packet_type: PacketTypeOrderRestated,
    pub timestamp: Timestamp, // Venue assigned
    pub user_ref_number: UserRefNumber,
    pub restate_reason: RestatedReason,
    #[serde(skip)]
    #[byteserde(replace( appendages.byte_len() ))]
    appendage_length: u16,
    #[byteserde(deplete(appendage_length))]
    pub appendages: OrderRestatedAppendage,
}
impl From<(&EnterOrder, RestatedReason, DisplayQty, DisplayPrice, SecondaryOrdRefNum)> for OrderRestated {
    fn from(value: (&EnterOrder, RestatedReason, DisplayQty, DisplayPrice, SecondaryOrdRefNum)) -> Self {
        let (ord, reason, display_qty, display_price, secondary_ord_ref_num) = value;
        let appendages = OrderRestatedAppendage {
            display_qty: Some(display_qty.into()),
            display_price: Some(display_price.into()),
            secondary_order_ref_num: Some(secondary_ord_ref_num.into()),
        };
        Self {
            packet_type: PacketTypeOrderRestated::default(),
            timestamp: Timestamp::default(), // Venue assigned
            user_ref_number: ord.user_ref_number,
            restate_reason: reason,
            appendage_length: appendages.byte_len() as u16,
            appendages,
        }
    }
}

#[derive(Deserialize)]
struct OrderRestatedJsonDes {
    timestamp: Timestamp, // Venue assigned
    user_ref_number: UserRefNumber,
    restate_reason: RestatedReason,
    appendages: OrderRestatedAppendage,
}
impl From<OrderRestatedJsonDes> for OrderRestated {
    fn from(value: OrderRestatedJsonDes) -> Self {
        Self {
            packet_type: PacketTypeOrderRestated::default(),
            timestamp: value.timestamp, // Venue assigned
            user_ref_number: value.user_ref_number,
            restate_reason: value.restate_reason,
            appendage_length: value.appendages.byte_len() as u16,
            appendages: value.appendages,
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

        let enter_order = EnterOrder::default();
        let msg_inp = OrderRestated::from((&enter_order, RestatedReason::refresh_of_display(), 1.into(), 0.0.into(), 1.into()));

        let ser: ByteSerializerStack<128> = to_serializer_stack(&msg_inp).unwrap();
        info!("ser: {:#x}", ser);

        let msg_out: OrderRestated = from_serializer_stack(&ser).unwrap();

        info!("msg_inp: {:?}", msg_inp);
        info!("msg_out: {:?}", msg_out);
        assert_eq!(msg_out, msg_inp);
    }

    #[test]
    fn test_msg_serde() {
        setup::log::configure_compact(log::LevelFilter::Info);

        let enter_order = EnterOrder::default();
        let mut msg_inp = OrderRestated::from((&enter_order, RestatedReason::refresh_of_display(), 1.into(), 0.0.into(), 1.into()));
        msg_inp.timestamp = 1.into();
        // info!("msg_inp: {:?}", msg_inp);

        let json_out = to_string(&msg_inp).unwrap();
        let json_exp = r#"{"timestamp":1,"user_ref_number":1,"restate_reason":"REFRESH_OF_DISPLAY","appendages":{"display_qty":1,"display_price":0.0,"secondary_order_ref_num":1}}"#;
        info!("json_out: {}", json_out);

        if matches!(diff(&json_out, json_exp, ","), (dist, _) if dist != 0) {
            print_diff(&json_out, json_exp, ",");
            assert_eq!(json_out, json_exp);
        }

        let msg_out: OrderRestated = from_str(&json_out).unwrap();
        // info!("msg_out: {:?}", msg_out);
        assert_eq!(msg_out, msg_inp)
    }
}
