use crate::prelude::*;
use byteserde_derive::{ByteDeserializeSlice, ByteSerializeStack, ByteSerializedLenOf};
use serde::{Deserialize, Serialize};

#[derive(ByteSerializeStack, ByteDeserializeSlice, ByteSerializedLenOf, Serialize, Deserialize, PartialEq, Clone, Debug)]
#[byteserde(endian = "be")]
pub struct CancelOrder {
    #[serde(default, skip_serializing)]
    packet_type: PacketTypeCancelOrder,
    pub user_ref_number: UserRefNumber,
    pub quantity: Quantity,
}
pub trait CancelableOrder {
    fn user_ref_number(&self) -> UserRefNumber;
    fn quantity(&self) -> Quantity;
    fn cl_ord_id(&self) -> CltOrderId;
}
impl<T: CancelableOrder> From<(&T, Quantity)> for CancelOrder {
    fn from(value: (&T, Quantity)) -> Self {
        let (ord, quantity) = (value.0, value.1);

        Self {
            packet_type: PacketTypeCancelOrder::default(),
            user_ref_number: ord.user_ref_number(),
            quantity,
        }
    }
}
impl<T: CancelableOrder> From<&T> for CancelOrder {
    fn from(ord: &T) -> Self {
        Self {
            packet_type: PacketTypeCancelOrder::default(),
            user_ref_number: ord.user_ref_number(),
            quantity: 0.into(),
        }
    }
}
impl CancelOrder {
    pub fn new(user_ref_number: UserRefNumber, quantity: Quantity) -> Self {
        Self {
            packet_type: PacketTypeCancelOrder::default(),
            user_ref_number,
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

        let msg_inp = CancelOrder::from(&EnterOrder::default());

        let ser: ByteSerializerStack<128> = to_serializer_stack(&msg_inp).unwrap();
        info!("ser: {:#x}", ser);

        let msg_out: CancelOrder = from_serializer_stack(&ser).unwrap();

        info!("msg_inp: {:?}", msg_inp);
        info!("msg_out: {:?}", msg_out);
        assert_eq!(msg_out, msg_inp);
    }

    #[test]
    fn test_msg_serde() {
        setup::log::configure_compact();

        let msg_inp = CancelOrder::from(&EnterOrder::default());
        // info!("msg_inp: {:?}", msg_inp);

        let json_out = to_string(&msg_inp).unwrap();
        info!("json_out: {}", json_out);
        let json_exp = r#"{"user_ref_number":1,"quantity":0}"#;

        let (dist, _) = diff(&json_out, json_exp, "\n"); // pretty print the diff
        if dist != 0 {
            print_diff(&json_out, json_exp, "\n")
        }

        let msg_out: CancelOrder = from_str(&json_out).unwrap();

        // info!("msg_out: {:?}", msg_out);
        assert_eq!(msg_out, msg_inp);
    }
}
