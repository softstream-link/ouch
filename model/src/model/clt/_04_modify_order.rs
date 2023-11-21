use crate::prelude::*;
use byteserde_derive::{ByteDeserializeSlice, ByteSerializeStack, ByteSerializedLenOf};
use serde::{Deserialize, Serialize};

#[derive(ByteSerializeStack, ByteDeserializeSlice, ByteSerializedLenOf, Serialize, Deserialize, PartialEq, Clone, Debug)]
#[byteserde(endian = "be")]
pub struct ModifyOrder {
    #[serde(default, skip_serializing)]
    packet_type: PacketTypeModifyOrder,
    user_ref_number: UserRefNumber,
    side: Side,
    quantity: Quantity,
}

pub trait ModifiableOrder {
    fn user_ref_number(&self) -> UserRefNumber;
    fn side(&self) -> Side;
    fn quantity(&self) -> Quantity;
}

impl<T: ModifiableOrder> From<(&T, Side, Quantity)> for ModifyOrder {
    fn from(value: (&T, Side, Quantity)) -> Self {
        let (orig_order, new_side, new_quantity) = (value.0, value.1, value.2);
        #[cfg(debug_assertions)]
        {
            if orig_order.side().is_sell() || orig_order.side().is_sell_short() || orig_order.side().is_sell_short_exempt() {
                assert!(new_side.is_sell() || new_side.is_sell_short() || new_side.is_sell_short_exempt(), "Side transition from: {}, to: {}, not allowed", orig_order.side(), new_side);
            } else if orig_order.side().is_buy() {
                assert!(new_side.is_buy(), "Side transition from: {}, to: {}, not allowed", orig_order.side(), new_side);
            } else {
                panic!("Side transition from: {}, to: {}, not allowed", orig_order.side(), new_side);
            }
        }
        Self {
            packet_type: PacketTypeModifyOrder::default(),
            user_ref_number: orig_order.user_ref_number(),
            side: new_side,
            quantity: new_quantity,
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
        setup::log::configure_compact(log::LevelFilter::Info);
        let msg_inp = ModifyOrder::from((&EnterOrder::default(), Side::buy(), 10.into()));

        let ser: ByteSerializerStack<128> = to_serializer_stack(&msg_inp).unwrap();
        info!("ser: {:#x}", ser);

        let msg_out: ModifyOrder = from_serializer_stack(&ser).unwrap();

        info!("msg_inp: {:?}", msg_inp);
        info!("msg_out: {:?}", msg_out);
        assert_eq!(msg_out, msg_inp);
    }

    #[test]
    fn test_msg_serde() {
        setup::log::configure_compact(log::LevelFilter::Info);

        let msg_inp = ModifyOrder::from((&EnterOrder::default(), Side::buy(), 10.into()));
        // info!("msg_inp: {:?}", msg_inp);

        let json_out = to_string(&msg_inp).unwrap();
        let json_exp = r#"{"user_ref_number":1,"side":"BUY","quantity":10}"#;
        info!("json_out: {}", json_out);

        if matches!(diff(&json_out, json_exp, ","), (dist, _) if dist != 0) {
            print_diff(&json_out, json_exp, ",");
            assert_eq!(json_out, json_exp);
        }

        let msg_out: ModifyOrder = from_str(&json_out).unwrap();
        // info!("msg_out: {:?}", msg_out);
        assert_eq!(msg_out, msg_inp);
    }
}
