use crate::prelude::*;
use byteserde_derive::{ByteDeserializeSlice, ByteSerializeStack, ByteSerializedLenOf};
use serde::{Deserialize, Serialize};

#[derive(ByteSerializeStack, ByteDeserializeSlice, ByteSerializedLenOf, Serialize, Deserialize, PartialEq, Clone, Debug)]
#[byteserde(endian = "be")]
pub struct OrderModified {
    #[serde(default, skip_serializing)]
    packet_type: PacketTypeOrderModified,
    pub timestamp: Timestamp, // Venue assigned
    pub user_ref_number: UserRefNumber,
    pub side: Side,
    pub quantity: Quantity,
}

impl From<(&EnterOrder, Quantity)> for OrderModified {
    fn from(value: (&EnterOrder, Quantity)) -> Self {
        let (ord, quantity) = value;
        Self {
            packet_type: PacketTypeOrderModified::default(),
            timestamp: Timestamp::default(), // Venue assigned
            user_ref_number: ord.user_ref_number(),
            side: ord.side,
            quantity,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use byteserde::prelude::*;
    use links_core::unittest::setup;
    use log::info;
    use serde_json::{from_str, to_string};
    use text_diff::{diff, print_diff};

    #[test]
    fn test_msg_byteserde() {
        setup::log::configure_compact();

        let enter_order = EnterOrder::default();
        let msg_inp = OrderModified::from((&enter_order, 1.into()));

        let ser: ByteSerializerStack<128> = to_serializer_stack(&msg_inp).unwrap();
        info!("ser: {:#x}", ser);

        let msg_out: OrderModified = from_serializer_stack(&ser).unwrap();

        info!("msg_inp: {:?}", msg_inp);
        info!("msg_out: {:?}", msg_out);
        assert_eq!(msg_out, msg_inp);
    }
    #[test]
    fn test_msg_serde() {
        setup::log::configure_compact();

        let enter_order = EnterOrder::default();
        let mut msg_inp = OrderModified::from((&enter_order, 1.into()));
        msg_inp.timestamp = 1.into();
        // info!("msg_inp: {:?}", msg_inp);

        let json_out = to_string(&msg_inp).unwrap();
        let json_exp = r#"{"timestamp":1,"user_ref_number":1,"side":"BUY","quantity":1}"#;
        info!("json_out: {}", json_out);

        if matches!(diff(&json_out, json_exp, ","), (dist, _) if dist != 0) {
            print_diff(&json_out, json_exp, ",");
            assert_eq!(json_out, json_exp);
        }

        let msg_out: OrderModified = from_str(&json_out).unwrap();
        // info!("msg_out: {:?}", msg_out);
        assert_eq!(msg_out, msg_inp);
    }
}
