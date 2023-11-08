use crate::prelude::*;
use byteserde_derive::{ByteDeserializeSlice, ByteSerializeStack, ByteSerializedLenOf};
use serde::{Deserialize, Serialize};

#[derive(ByteSerializeStack, ByteDeserializeSlice, ByteSerializedLenOf, Serialize, Deserialize, PartialEq, Clone, Debug)]
#[byteserde(endian = "be")]
pub struct OrderRejected {
    #[serde(default, skip_serializing)]
    packet_type: PacketTypeOrderRejected,
    pub timestamp: Timestamp, // Venue assigned
    pub user_ref_number: UserRefNumber,
    pub reject_reason: OrderRejectReason,
    pub clt_order_id: CltOrderId,
}

impl<T: CancelableOrder> From<(&T, OrderRejectReason)> for OrderRejected {
    /// `T`: [CancelableOrder]
    fn from(value: (&T, OrderRejectReason)) -> Self {
        let (ord, reject_reason) = value;
        Self {
            packet_type: PacketTypeOrderRejected::default(),
            timestamp: Timestamp::default(), // Venue assigned
            user_ref_number: ord.user_ref_number(),
            reject_reason,
            clt_order_id: ord.cl_ord_id(),
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
        let msg_inp = OrderRejected::from((&enter_order, OrderRejectReason::quote_unavailable()));

        let ser: ByteSerializerStack<128> = to_serializer_stack(&msg_inp).unwrap();
        info!("ser: {:#x}", ser);

        let msg_out: OrderRejected = from_serializer_stack(&ser).unwrap();

        info!("msg_inp: {:?}", msg_inp);
        info!("msg_out: {:?}", msg_out);
        assert_eq!(msg_out, msg_inp);
    }

    #[test]
    fn test_msg_serde() {
        setup::log::configure_compact();

        let enter_order = EnterOrder::default();
        let mut msg_inp = OrderRejected::from((&enter_order, OrderRejectReason::quote_unavailable()));
        msg_inp.timestamp = 1.into();
        // info!("msg_inp: {:?}", msg_inp);

        let json_out = to_string(&msg_inp).unwrap();
        let json_exp = r#"{"timestamp":1,"user_ref_number":1,"reason":1,"clt_order_id":"1"}"#;
        info!("json_out: {}", json_out);

        if matches!(diff(&json_out, json_exp, ","), (dist, _) if dist != 0) {
            print_diff(&json_out, json_exp, ",");
            assert_eq!(json_out, json_exp);
        }

        let msg_out: OrderRejected = from_str(&json_out).unwrap();
        // info!("msg_out: {:?}", msg_out);
        assert_eq!(msg_out, msg_inp)
    }
}
