use crate::prelude::*;
use byteserde_derive::{ByteDeserializeSlice, ByteSerializeStack, ByteSerializedLenOf};
use serde::{Deserialize, Serialize};

#[derive(ByteSerializeStack, ByteDeserializeSlice, ByteSerializedLenOf, Serialize, Deserialize, PartialEq, Clone, Debug)]
#[byteserde(endian = "be")]
pub struct SystemEvent {
    #[serde(default, skip_serializing)]
    packet_type: PacketTypeSystemEvent,
    timestamp: Timestamp,
    event_code: EventCode,
}

#[rustfmt::skip]
impl SystemEvent {
    pub fn start_of_day() -> Self { Self {packet_type: Default::default(), timestamp: Default::default(), event_code: EventCode::start_of_day() } }
    pub fn end_of_day() -> Self { Self {packet_type: Default::default(), timestamp: Default::default(), event_code: EventCode::end_of_day() } }
    pub fn is_start_of_day(&self) -> bool { self.event_code.is_start_of_day() }
    pub fn is_end_of_day(&self) -> bool { self.event_code.is_end_of_day() }
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
        let msg_inp = SystemEvent::start_of_day();

        let ser: ByteSerializerStack<128> = to_serializer_stack(&msg_inp).unwrap();
        info!("ser: {:#x}", ser);

        let msg_out: SystemEvent = from_serializer_stack(&ser).unwrap();

        info!("msg_inp: {:?}", msg_inp);
        info!("msg_out: {:?}", msg_out);
        assert_eq!(msg_out, msg_inp);
    }

    #[test]
    fn test_msg_serde() {
        setup::log::configure_compact();
        let mut msg_inp = SystemEvent::start_of_day();
        msg_inp.timestamp = Timestamp::from(1);
        // info!("msg_inp: {:?}", msg_inp);

        let json_out = to_string(&msg_inp).unwrap();
        info!("json_out: {}", json_out);
        let json_exp = r#"{"timestamp":1,"event_code":"START_OF_DAY"}"#;

        if matches!(diff(&json_out, json_exp, ","), (dist, _) if dist != 0) {
            print_diff(&json_out, json_exp, ",");
            assert_eq!(json_out, json_exp);
        }

        let msg_out: SystemEvent = from_str(&json_out).unwrap();
        // info!("msg_out: {:?}", msg_out);

        assert_eq!(msg_out, msg_inp);
    }
}
