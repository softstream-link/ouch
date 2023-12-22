use byteserde_derive::{ByteDeserializeSlice, ByteSerializeStack, ByteSerializedLenOf};
use derive_more::TryInto;
use serde::{Deserialize, Serialize};
use soupbintcp_model::prelude::{CltSoupBinTcpMsg, SoupBinTcpPayload, SvcSoupBinTcpMsg, UniSoupBinTcpMsg, SOUPBINTCP_MAX_FRAME_SIZE_EXCLUDING_PAYLOAD_DEBUG};

use crate::prelude::*;

pub const SVC_OUCH_MAX_PLD_SIZE: usize = 151; // TODO revise Options fields and remeasure
pub const SVC_OUCH_MAX_FRAME_SIZE: usize = SVC_OUCH_MAX_PLD_SIZE + SOUPBINTCP_MAX_FRAME_SIZE_EXCLUDING_PAYLOAD_DEBUG;

pub const CLT_OUCH_MAX_PLD_SIZE: usize = 134; // TODO revise Options fields and remeasure
pub const CLT_OUCH_MAX_FRAME_SIZE: usize = CLT_OUCH_MAX_PLD_SIZE + SOUPBINTCP_MAX_FRAME_SIZE_EXCLUDING_PAYLOAD_DEBUG;

pub const OUCH_MAX_FRAME_SIZE: usize = {
    if SVC_OUCH_MAX_FRAME_SIZE > CLT_OUCH_MAX_FRAME_SIZE {
        SVC_OUCH_MAX_FRAME_SIZE
    } else {
        CLT_OUCH_MAX_FRAME_SIZE
    }
};
#[derive(ByteSerializeStack, ByteDeserializeSlice, ByteSerializedLenOf, Serialize, Deserialize, PartialEq, Clone, Debug, TryInto)]
#[try_into(owned, ref, ref_mut)]
#[byteserde(peek(0, 1))]
pub enum CltOuchPayload {
    #[byteserde(eq(PacketTypeEnterOrder::as_slice()))]
    EnterOrder(EnterOrder),
    #[byteserde(eq(PacketTypeReplaceOrder::as_slice()))]
    ReplaceOrder(ReplaceOrder),
    #[byteserde(eq(PacketTypeCancelOrder::as_slice()))]
    CancelOrder(CancelOrder),
    #[byteserde(eq(PacketTypeModifyOrder::as_slice()))]
    ModifyOrder(ModifyOrder),
    #[byteserde(eq(PacketTypeAccountQueryRequest::as_slice()))]
    AccountQueryRequest(AccountQueryRequest),
}
impl SoupBinTcpPayload<CltOuchPayload> for CltOuchPayload {}

/// Both [ReplaceOrder] & [OrderReplaced] are serialized as b'U' hence it is impossible to distinguish deserialization type unless they are in two different enums.
#[derive(ByteSerializeStack, ByteDeserializeSlice, ByteSerializedLenOf, Serialize, Deserialize, PartialEq, Clone, Debug, TryInto)]
#[try_into(owned, ref, ref_mut)]
#[byteserde(peek(0, 1))]
pub enum SvcOuchPayload {
    #[byteserde(eq(PacketTypeSystemEvent::as_slice()))]
    SystemEvent(SystemEvent),
    #[byteserde(eq(PacketTypeOrderAccepted::as_slice()))]
    OrderAccepted(OrderAccepted),
    #[byteserde(eq(PacketTypeOrderReplaced::as_slice()))]
    OrderReplaced(OrderReplaced),
    #[byteserde(eq(PacketTypeOrderCanceled::as_slice()))]
    OrderCanceled(OrderCanceled),
    #[byteserde(eq(PacketTypeOrderAiqCanceled::as_slice()))]
    OrderAiqCanceled(OrderAiqCanceled),
    #[byteserde(eq(PacketTypeOrderExecuted::as_slice()))]
    OrderExecuted(OrderExecuted),
    #[byteserde(eq(PacketTypeBrokenTrade::as_slice()))]
    BrokenTrade(BrokenTrade),
    #[byteserde(eq(PacketTypeOrderRejected::as_slice()))]
    OrderRejected(OrderRejected),
    #[byteserde(eq(PacketTypeCancelPending::as_slice()))]
    CancelPending(CancelPending),
    #[byteserde(eq(PacketTypeCancelReject::as_slice()))]
    CancelReject(CancelReject),
    #[byteserde(eq(PacketTypePriorityUpdate::as_slice()))]
    PriorityUpdate(PriorityUpdate),
    #[byteserde(eq(PacketTypeOrderModified::as_slice()))]
    OrderModified(OrderModified),
    #[byteserde(eq(PacketTypeOrderRestated::as_slice()))]
    OrderRestated(OrderRestated),
    #[byteserde(eq(PacketTypeAccountQueryResponse::as_slice()))]
    AccountQueryResponse(AccountQueryResponse),
}

impl SoupBinTcpPayload<SvcOuchPayload> for SvcOuchPayload {}

pub type CltOuchMsg = CltSoupBinTcpMsg<CltOuchPayload>;
pub type SvcOuchMsg = SvcSoupBinTcpMsg<SvcOuchPayload>;

pub type UniOuchMsg = UniSoupBinTcpMsg<CltOuchPayload, SvcOuchPayload>;

macro_rules! for_uni_ouch_msg {
    ($FROM:ty, $ENUM:path) => {
        impl From<$FROM> for UniOuchMsg {
            #[inline(always)]
            fn from(payload: $FROM) -> Self {
                $ENUM(payload.into())
            }
        }
    };
}

mod froms_clt_pld {
    use super::*;
    macro_rules! for_clt_ouch_msg {
        ($FROM:ty, $PAYLOAD_ENUM:path) => {
            impl From<$FROM> for CltOuchMsg {
                #[inline(always)]
                fn from(payload: $FROM) -> Self {
                    Self::udata($PAYLOAD_ENUM(payload))
                }
            }
        };
    }
    for_clt_ouch_msg!(EnterOrder, CltOuchPayload::EnterOrder);
    for_clt_ouch_msg!(ReplaceOrder, CltOuchPayload::ReplaceOrder);
    for_clt_ouch_msg!(CancelOrder, CltOuchPayload::CancelOrder);
    for_clt_ouch_msg!(ModifyOrder, CltOuchPayload::ModifyOrder);
    for_clt_ouch_msg!(AccountQueryRequest, CltOuchPayload::AccountQueryRequest);

    for_uni_ouch_msg!(EnterOrder, UniOuchMsg::Clt);
    for_uni_ouch_msg!(ReplaceOrder, UniOuchMsg::Clt);
    for_uni_ouch_msg!(CancelOrder, UniOuchMsg::Clt);
    for_uni_ouch_msg!(ModifyOrder, UniOuchMsg::Clt);
    for_uni_ouch_msg!(AccountQueryRequest, UniOuchMsg::Clt);
}

mod froms_svc_pld {
    use super::*;
    macro_rules! for_svc_ouch_msg {
        ($FROM:ty, $PAYLOAD_ENUM:path) => {
            impl From<$FROM> for SvcOuchMsg {
                #[inline(always)]
                fn from(payload: $FROM) -> Self {
                    Self::sdata($PAYLOAD_ENUM(payload))
                }
            }
        };
    }
    for_svc_ouch_msg!(SystemEvent, SvcOuchPayload::SystemEvent);
    for_svc_ouch_msg!(OrderAccepted, SvcOuchPayload::OrderAccepted);
    for_svc_ouch_msg!(OrderReplaced, SvcOuchPayload::OrderReplaced);
    for_svc_ouch_msg!(OrderCanceled, SvcOuchPayload::OrderCanceled);
    for_svc_ouch_msg!(OrderAiqCanceled, SvcOuchPayload::OrderAiqCanceled);
    for_svc_ouch_msg!(OrderExecuted, SvcOuchPayload::OrderExecuted);
    for_svc_ouch_msg!(BrokenTrade, SvcOuchPayload::BrokenTrade);
    for_svc_ouch_msg!(OrderRejected, SvcOuchPayload::OrderRejected);
    for_svc_ouch_msg!(CancelPending, SvcOuchPayload::CancelPending);
    for_svc_ouch_msg!(CancelReject, SvcOuchPayload::CancelReject);
    for_svc_ouch_msg!(PriorityUpdate, SvcOuchPayload::PriorityUpdate);
    for_svc_ouch_msg!(OrderModified, SvcOuchPayload::OrderModified);
    for_svc_ouch_msg!(OrderRestated, SvcOuchPayload::OrderRestated);
    for_svc_ouch_msg!(AccountQueryResponse, SvcOuchPayload::AccountQueryResponse);

    for_uni_ouch_msg!(SystemEvent, UniOuchMsg::Svc);
    for_uni_ouch_msg!(OrderAccepted, UniOuchMsg::Svc);
    for_uni_ouch_msg!(OrderReplaced, UniOuchMsg::Svc);
    for_uni_ouch_msg!(OrderCanceled, UniOuchMsg::Svc);
    for_uni_ouch_msg!(OrderAiqCanceled, UniOuchMsg::Svc);
    for_uni_ouch_msg!(OrderExecuted, UniOuchMsg::Svc);
    for_uni_ouch_msg!(BrokenTrade, UniOuchMsg::Svc);
    for_uni_ouch_msg!(OrderRejected, UniOuchMsg::Svc);
    for_uni_ouch_msg!(CancelPending, UniOuchMsg::Svc);
    for_uni_ouch_msg!(CancelReject, UniOuchMsg::Svc);
    for_uni_ouch_msg!(PriorityUpdate, UniOuchMsg::Svc);
    for_uni_ouch_msg!(OrderModified, UniOuchMsg::Svc);
    for_uni_ouch_msg!(OrderRestated, UniOuchMsg::Svc);
    for_uni_ouch_msg!(AccountQueryResponse, UniOuchMsg::Svc);
}

#[cfg(test)]
#[cfg(feature = "unittest")]
mod test {

    use crate::{
        model::ouch::{CLT_OUCH_MAX_PLD_SIZE, SVC_OUCH_MAX_PLD_SIZE},
        prelude::*,
        unittest::setup::model::{clt_msgs_default, svc_msgs_default},
    };
    use byteserde::prelude::*;
    use links_core::unittest::setup;
    use log::info;
    use serde_json::to_string;

    #[test]
    fn test_ouch_with_envelope_ser_des() {
        setup::log::configure_compact(log::LevelFilter::Info);

        let mut msg_inp = vec![];
        for clt in clt_msgs_default() {
            msg_inp.push(clt.into());
        }
        for svc in svc_msgs_default() {
            msg_inp.push(svc.into());
        }
        let mut ser = ByteSerializerStack::<{ 1024 * 2 }>::default();
        for msg in msg_inp.iter() {
            match msg {
                UniOuchMsg::Clt(msg_inp_inb) => {
                    // info!("msg_inp_inb: {:?}", msg_inp_inb);
                    info!("msg_inp_inb: {}", to_string(msg_inp_inb).unwrap());
                    let _ = ser.serialize(msg_inp_inb).unwrap();
                }
                UniOuchMsg::Svc(msg_inp_oub) => {
                    // info!("msg_inp_oub: {:?}", msg_inp_oub);
                    info!("msg_inp_oub: {}", to_string(msg_inp_oub).unwrap());
                    let _ = ser.serialize(msg_inp_oub).unwrap();
                }
            }
        }
        let mut des = ByteDeserializerSlice::new(ser.as_slice());

        for ouch in msg_inp.iter() {
            match ouch {
                UniOuchMsg::Clt(msg_inp_inb) => {
                    let msg_out_inb = des.deserialize::<CltOuchMsg>().unwrap();
                    info!("msg_out_inb: {}", to_string(&msg_out_inb).unwrap());
                    assert_eq!(msg_inp_inb, &msg_out_inb);
                }
                UniOuchMsg::Svc(msg_inp_oub) => {
                    let msg_out_oub = des.deserialize::<SvcOuchMsg>().unwrap();
                    info!("msg_out_oub: {}", to_string(&msg_out_oub).unwrap());
                    assert_eq!(msg_inp_oub, &msg_out_oub);
                }
            }
        }
        assert!(des.is_empty());
    }

    #[test]
    fn test_ouch5_max_size() {
        setup::log::configure_compact(log::LevelFilter::Info);

        let inb = clt_msgs_default().into_iter().map(|msg| (msg.byte_len(), msg)).collect::<Vec<_>>();
        for (byte_len, clt_msg) in inb.iter() {
            info!("byte_len: {:>3}, clt_msg: {}", byte_len, to_string(clt_msg).unwrap());
        }
        let max_frame_size_clt = inb.iter().map(|(len, _)| *len).max().unwrap();
        info!("max_frame_size_clt: {}", max_frame_size_clt);
        assert_eq!(max_frame_size_clt, CLT_OUCH_MAX_PLD_SIZE);

        let oub = svc_msgs_default().into_iter().map(|msg| (msg.byte_len(), msg)).collect::<Vec<_>>();
        for (byte_len, svc_msg) in oub.iter() {
            info!("byte_len: {:>3}, svc_msg: {}", byte_len, to_string(svc_msg).unwrap());
        }
        let max_frame_size_svc = oub.iter().map(|(len, _)| *len).max().unwrap();
        info!("max_frame_size_svc: {}", max_frame_size_svc);
        assert_eq!(max_frame_size_svc, SVC_OUCH_MAX_PLD_SIZE);
    }
}
