use crate::prelude::*;
use byteserde_derive::{ByteDeserializeSlice, ByteSerializeStack, ByteSerializedLenOf};
use serde::{Deserialize, Serialize};

#[rustfmt::skip]
#[derive(ByteSerializeStack, ByteDeserializeSlice, ByteSerializedLenOf, Serialize, Deserialize, PartialEq, Clone, Debug, Default)]
#[byteserde(endian = "be")]
pub struct AccountQueryRequest {
    #[serde(default, skip_serializing)]
    packet_type: PacketTypeAccountQueryRequest,
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
        setup::log::configure();
        let msg_inp = AccountQueryRequest::default();

        let ser: ByteSerializerStack<128> = to_serializer_stack(&msg_inp).unwrap();
        info!("ser: {:#x}", ser);

        let msg_out: AccountQueryRequest = from_serializer_stack(&ser).unwrap();

        info!("msg_inp: {:?}", msg_inp);
        info!("msg_out: {:?}", msg_out);
        assert_eq!(msg_out, msg_inp);
    }

    #[test]
    fn test_msg_serde() {
        setup::log::configure_compact();

        let msg_inp = AccountQueryRequest::default();
        // info!("msg_inp: {:?}", msg_inp);

        let json_out = to_string(&msg_inp).unwrap();
        info!("json_out: {}", json_out);
        let json_exp = r#"{}"#;

        let (dist, _) = diff(&json_out, json_exp, "\n"); // pretty print the diff
        if dist != 0 {
            print_diff(&json_out, json_exp, "\n")
        }

        let msg_out: AccountQueryRequest = from_str(&json_out).unwrap();
        // info!("msg_out: {:?}", msg_out);
        assert_eq!(msg_out, msg_inp);
    }
}
