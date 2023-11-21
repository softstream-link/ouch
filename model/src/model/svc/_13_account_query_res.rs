use crate::prelude::*;
use byteserde_derive::{ByteDeserializeSlice, ByteSerializeStack, ByteSerializedLenOf};
use serde::{Deserialize, Serialize};

#[derive(ByteSerializeStack, ByteDeserializeSlice, ByteSerializedLenOf, Serialize, Deserialize, PartialEq, Clone, Debug)]
#[byteserde(endian = "be")]
pub struct AccountQueryResponse {
    #[serde(default, skip_serializing)]
    packet_type: PacketTypeAccountQueryResponse,
    pub timestamp: Timestamp,
    pub next_user_ref_number: UserRefNumber,
}
impl From<UserRefNumber> for AccountQueryResponse {
    fn from(next_user_ref_number: UserRefNumber) -> Self {
        Self {
            packet_type: PacketTypeAccountQueryResponse::default(),
            timestamp: Timestamp::default(),
            next_user_ref_number,
        }
    }
}
impl From<u32> for AccountQueryResponse {
    fn from(next_user_ref_number: u32) -> Self {
        UserRefNumber::from(next_user_ref_number).into()
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
        setup::log::configure_compact(log::LevelFilter::Info);
        let msg_inp = UserRefNumber::new(1).into();

        let ser: ByteSerializerStack<128> = to_serializer_stack(&msg_inp).unwrap();
        info!("ser: {:#x}", ser);

        let msg_out: AccountQueryResponse = from_serializer_stack(&ser).unwrap();

        info!("msg_inp: {:?}", msg_inp);
        info!("msg_out: {:?}", msg_out);
        assert_eq!(msg_out, msg_inp);
    }

    #[test]
    fn test_msg_serde() {
        setup::log::configure_compact(log::LevelFilter::Info);
        let mut msg_inp: AccountQueryResponse = 1.into();
        msg_inp.timestamp = 1.into();
        // info!("msg_inp: {:?}", msg_inp);

        let json_out = to_string(&msg_inp).unwrap();
        let json_exp = r#"{"timestamp":1,"next_user_ref_number":1}"#;
        info!("json_out: {}", json_out);

        if matches!(diff(&json_out, json_exp, ","), (dist, _) if dist != 0) {
            print_diff(&json_out, json_exp, ",");
            assert_eq!(json_out, json_exp);
        }

        let msg_out: AccountQueryResponse = from_str(&json_out).unwrap();
        // info!("msg_out: {:?}", msg_out);
        assert_eq!(msg_out, msg_inp);
    }
}
