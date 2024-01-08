use crate::prelude::*;
use byteserde::prelude::*;
use byteserde_derive::{ByteDeserializeSlice, ByteSerializeStack, ByteSerializedLenOf};
use serde::{Deserialize, Serialize};

#[derive(ByteSerializeStack, ByteDeserializeSlice, ByteSerializedLenOf, Serialize, Deserialize, PartialEq, Clone, Debug)]
#[byteserde(endian = "be")]
#[serde(from = "OrderExecutedJsonDes")]
pub struct OrderExecuted {
    #[serde(default, skip_serializing)]
    packet_type: PacketTypeOrderExecuted,
    pub timestamp: Timestamp, // Venue assigned
    pub user_ref_number: UserRefNumber,
    pub quantity: Quantity,
    pub price: Price,
    pub liquidity_flag: LiquidityFlag,
    pub match_number: MatchNumber,
    #[serde(skip)]
    #[byteserde(replace( appendages.byte_len() ))]
    appendage_length: u16,
    #[byteserde(deplete(appendage_length))]
    pub appendages: EnterOrderAppendage,
}

impl From<&EnterOrder> for OrderExecuted {
    fn from(enter_order: &EnterOrder) -> Self {
        Self {
            packet_type: PacketTypeOrderExecuted::default(),
            timestamp: Timestamp::default(), // Venue assigned
            user_ref_number: enter_order.user_ref_number,
            quantity: enter_order.quantity,
            price: enter_order.price,
            liquidity_flag: LiquidityFlag::added(),
            match_number: MatchNumber::default(),
            appendage_length: enter_order.appendages.byte_len() as u16,
            appendages: enter_order.appendages,
        }
    }
}

#[derive(Deserialize)]
struct OrderExecutedJsonDes {
    timestamp: Timestamp, // Venue assigned
    user_ref_number: UserRefNumber,
    quantity: Quantity,
    price: Price,
    liquidity_flag: LiquidityFlag,
    match_number: MatchNumber,
    appendages: EnterOrderAppendage,
}
impl From<OrderExecutedJsonDes> for OrderExecuted {
    fn from(value: OrderExecutedJsonDes) -> Self {
        Self {
            packet_type: PacketTypeOrderExecuted::default(),
            timestamp: value.timestamp, // Venue assigned
            user_ref_number: value.user_ref_number,
            quantity: value.quantity,
            price: value.price,
            liquidity_flag: value.liquidity_flag,
            match_number: value.match_number,
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
    use serde_json::to_string;
    use text_diff::{diff, print_diff};

    #[test]
    fn test_msg_byteserde() {
        setup::log::configure();
        let enter_order = EnterOrder::default();
        let msg_inp = OrderExecuted::from(&enter_order);

        let ser: ByteSerializerStack<128> = to_serializer_stack(&msg_inp).unwrap();
        info!("ser: {:#x}", ser);

        let msg_out: OrderExecuted = from_serializer_stack(&ser).unwrap();

        info!("msg_inp: {:?}", msg_inp);
        info!("msg_out: {:?}", msg_out);
        assert_eq!(msg_out, msg_inp);
    }

    #[test]
    fn test_msg_serde() {
        setup::log::configure();
        let enter_order = EnterOrder::default();
        let mut msg_inp = OrderExecuted::from(&enter_order);
        msg_inp.timestamp = 1.into();
        // info!("msg_inp: {:?}", msg_inp);

        let json_out = to_string(&msg_inp).unwrap();
        let json_exp = r#"{"timestamp":1,"user_ref_number":1,"quantity":100,"price":1.2345,"liquidity_flag":"ADDED","match_number":0,"appendages":{"firm":"????","min_qty":0,"customer_type":"PORT_DEFAULT","max_floor":0,"price_type":"LIMIT","peg_offset":-1.1234,"discretion_price":0.0,"discretion_price_type":"LIMIT","discretion_peg_offset":-1.1234,"post_only":"NO","random_reserves":0,"route":"????","expire_time":0,"trade_now":"PORT_DEFAULT","handle_inst":"NO_INSTRUCTIONS","group_id":0,"shares_located":"NO"}}"#;
        info!("json_out: {}", json_out);

        if matches!(diff(&json_out, json_exp, ","), (dist, _) if dist != 0) {
            print_diff(&json_out, json_exp, ",");
            assert_eq!(json_out, json_exp);
        }

        let msg_out: OrderExecuted = serde_json::from_str(&json_out).unwrap();
        // info!("msg_out: {:?}", msg_out);
        assert_eq!(msg_out, msg_inp);
    }
}
