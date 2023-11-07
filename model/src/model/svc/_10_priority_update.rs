use crate::prelude::*;
use byteserde_derive::{ByteDeserializeSlice, ByteSerializeStack, ByteSerializedLenOf};
use serde::{Deserialize, Serialize};

#[derive(ByteSerializeStack, ByteDeserializeSlice, ByteSerializedLenOf, Serialize, Deserialize, PartialEq, Clone, Debug)]
#[byteserde(endian = "be")]
pub struct PriorityUpdate {
    #[serde(default, skip_serializing)]
    packet_type: PacketTypePriorityUpdate,
    pub timestamp: Timestamp, // Venue assigned
    pub user_ref_number: UserRefNumber,
    pub price: Price,
    pub display: Display,
    pub order_reference_number: OrderReferenceNumber, // Venue assigned
}

impl From<(&EnterOrder, OrderReferenceNumber)> for PriorityUpdate {
    fn from(value: (&EnterOrder, OrderReferenceNumber)) -> Self {
        let (ord, order_reference_number) = value;
        Self {
            packet_type: PacketTypePriorityUpdate::default(),
            timestamp: Timestamp::default(), // Venue assigned
            user_ref_number: ord.user_ref_number,
            price: ord.price,
            display: ord.display,
            order_reference_number, // Venue assigned
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use byteserde::prelude::*;
    use links_core::unittest::setup;
    use log::info;
    use serde_json::{to_string, from_str};
    use text_diff::{diff, print_diff};

    #[test]
    fn test_msg_byteserde() {
        setup::log::configure_compact();

        let enter_order = EnterOrder::default();
        let msg_inp = PriorityUpdate::from((&enter_order, OrderReferenceNumber::default()));

        let ser: ByteSerializerStack<128> = to_serializer_stack(&msg_inp).unwrap();
        info!("ser: {:#x}", ser);

        let msg_out: PriorityUpdate = from_serializer_stack(&ser).unwrap();

        info!("msg_inp: {:?}", msg_inp);
        info!("msg_out: {:?}", msg_out);
        assert_eq!(msg_out, msg_inp);
    }

    #[test]
    fn test_msg_serde() {
        setup::log::configure_compact();

        let enter_order = EnterOrder::default();
        let mut msg_inp = PriorityUpdate::from((&enter_order, OrderReferenceNumber::default()));
        msg_inp.timestamp = 1.into();
        // info!("msg_inp: {:?}", msg_inp);

        let json_out = to_string(&msg_inp).unwrap();
        let json_exp = r#"{"timestamp":1,"user_ref_number":1,"price":1.2345,"display":"VISIBLE","order_reference_number":0}"#;
        info!("json_out: {}", json_out);

        if matches!(diff(&json_out, json_exp, ","), (dist, _) if dist != 0) {
            print_diff(&json_out, json_exp, ",");
            assert_eq!(json_out, json_exp);
        }

        let msg_out: PriorityUpdate = from_str(&json_out).unwrap();
        // info!("msg_out: {:?}", msg_out);
        assert_eq!(msg_out, msg_inp)
    }
}
