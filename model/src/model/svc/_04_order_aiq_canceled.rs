use crate::prelude::*;
use byteserde_derive::{ByteDeserializeSlice, ByteSerializeStack, ByteSerializedLenOf};
use serde::{Deserialize, Serialize};

#[derive(ByteSerializeStack, ByteDeserializeSlice, ByteSerializedLenOf, Serialize, Deserialize, PartialEq, Clone, Debug)]
#[byteserde(endian = "be")]
pub struct OrderAiqCanceled {
    #[serde(default, skip_serializing)]
    packet_type: PacketTypeOrderAiqCanceled,
    pub timestamp: Timestamp, // Venue assigned
    pub user_ref_number: UserRefNumber,
    pub decrement_shares: Quantity,
    #[serde(default, skip_serializing)]
    pub cancel_aiq_reason: CancelAiqReason,
    pub prevented_from_trading: Quantity,
    pub execution_price: Price,
    pub liquidity_flag: LiquidityFlag,
    pub aiq_strategy: AiqStrategy,
}
impl<T> From<(&T, Quantity, CancelAiqReason, Quantity, Price, LiquidityFlag, AiqStrategy)> for OrderAiqCanceled
where T: CancelableOrder
{
    fn from(value: (&T, Quantity, CancelAiqReason, Quantity, Price, LiquidityFlag, AiqStrategy)) -> Self {
        let (ord, decrement_shares, cancel_aiq_reason, prevented_from_trading, execution_price, liquidity_flag, aiq_strategy) = value;
        Self {
            packet_type: PacketTypeOrderAiqCanceled::default(),
            timestamp: Timestamp::default(),
            user_ref_number: ord.user_ref_number(),
            decrement_shares,
            cancel_aiq_reason,
            prevented_from_trading,
            execution_price,
            liquidity_flag,
            aiq_strategy,
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

        let msg_inp = OrderAiqCanceled::from((&enter_order, 0.into(), CancelAiqReason::default(), 0.into(), 0.0.into(), LiquidityFlag::added(), AiqStrategy::default()));

        let ser: ByteSerializerStack<128> = to_serializer_stack(&msg_inp).unwrap();
        info!("ser: {:#x}", ser);

        let msg_out: OrderAiqCanceled = from_serializer_stack(&ser).unwrap();

        info!("msg_inp: {:?}", msg_inp);
        info!("msg_out: {:?}", msg_out);
        assert_eq!(msg_out, msg_inp);
    }

    #[test]
    fn test_msg_serde() {
        setup::log::configure_compact();
        let enter_order = EnterOrder::default();

        let mut msg_inp = OrderAiqCanceled::from((&enter_order, 0.into(), CancelAiqReason::default(), 0.into(), 0.0.into(), LiquidityFlag::added(), AiqStrategy::default()));
        msg_inp.timestamp = 1.into();
        // info!("msg_inp: {:?}", msg_inp);

        let json_out = to_string(&msg_inp).unwrap();
        let json_exp = r#"{"timestamp":1,"user_ref_number":1,"decrement_shares":0,"prevented_from_trading":0,"execution_price":0.0,"liquidity_flag":"ADDED","aiq_strategy":"?"}"#;
        info!("json_out: {}", json_out);

        if matches!(diff(&json_out, json_exp, ","), (dist, _) if dist != 0) {
            print_diff(&json_out, json_exp, ",");
            assert_eq!(json_out, json_exp);
        }

        let msg_out: OrderAiqCanceled = from_str(&json_out).unwrap();
        // info!("msg_out: {:?}", msg_out);
        assert_eq!(msg_out, msg_inp)
    }
}
