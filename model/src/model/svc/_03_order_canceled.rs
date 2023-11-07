use crate::prelude::*;
use byteserde_derive::{ByteDeserializeSlice, ByteSerializeStack, ByteSerializedLenOf};
use serde::{Deserialize, Serialize};

#[derive(ByteSerializeStack, ByteDeserializeSlice, ByteSerializedLenOf, Serialize, Deserialize, PartialEq, Clone, Debug)]
#[byteserde(endian = "be")]
pub struct OrderCanceled {
    #[serde(default, skip_serializing)]
    packet_type: PacketTypeOrderCanceled,
    pub timestamp: Timestamp, // Venue assigned
    pub orig_user_ref_number: UserRefNumber,
    pub user_ref_number: UserRefNumber,
    pub quantity: Quantity,
    pub cancel_reason: CancelReason,
}
impl From<(&EnterOrder, &CancelOrder)> for OrderCanceled {
    fn from(value: (&EnterOrder, &CancelOrder)) -> Self {
        let (enter_order, cancel_order) = value;
        Self {
            packet_type: PacketTypeOrderCanceled::default(),
            timestamp: Timestamp::default(),
            orig_user_ref_number: enter_order.user_ref_number,
            user_ref_number: cancel_order.user_ref_number,
            quantity: cancel_order.quantity,
            cancel_reason: CancelReason::user_requested(),
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
        let mut cancel_order = CancelOrder::from(&enter_order);
        cancel_order.user_ref_number = UserRefNumber::new(enter_order.user_ref_number.value() + 1);

        let msg_inp = OrderCanceled::from((&enter_order, &cancel_order));

        let ser: ByteSerializerStack<128> = to_serializer_stack(&msg_inp).unwrap();
        info!("ser: {:#x}", ser);

        let msg_out: OrderCanceled = from_serializer_stack(&ser).unwrap();

        info!("msg_inp: {:?}", msg_inp);
        info!("msg_out: {:?}", msg_out);
        assert_eq!(msg_out, msg_inp);
    }

    #[test]
    fn test_msg_serde() {
        setup::log::configure_compact();
        let enter_order = EnterOrder::default();
        let mut cancel_order = CancelOrder::from(&enter_order);
        cancel_order.user_ref_number = UserRefNumber::new(enter_order.user_ref_number.value() + 1);

        let mut msg_inp = OrderCanceled::from((&enter_order, &cancel_order));
        msg_inp.timestamp = Timestamp::from(1);

        // info!("msg_inp: {:?}", msg_inp);
        let json_out = to_string(&msg_inp).unwrap();
        let json_exp = r#"{"timestamp":1,"orig_user_ref_number":1,"user_ref_number":2,"quantity":0,"cancel_reason":"USER_REQUESTED"}"#;
        info!("json_out: {}", json_out);

        if matches!(diff(&json_out, json_exp, ","), (dist, _) if dist != 0) {
            print_diff(&json_out, json_exp, ",");
            assert_eq!(json_out, json_exp);
        }

        let msg_out: OrderCanceled = from_str(&json_out).unwrap();

        assert_eq!(msg_out, msg_inp);
    }
}
