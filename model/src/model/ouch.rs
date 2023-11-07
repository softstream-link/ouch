use byteserde_derive::{ByteDeserializeSlice, ByteSerializeStack, ByteSerializedLenOf};
use derive_more::TryInto;
use soupbintcp_model::prelude::{CltSoupBinTcpMsg, SoupBinTcpMsg, SoupBinTcpPayload, SvcSoupBinTcpMsg, SOUPBINTCP_MAX_FRAME_SIZE_EXCLUDING_PAYLOAD_DEBUG};

use crate::prelude::*;

use super::svc::_04_order_aiq_canceled::OrderAiqCanceled;

#[rustfmt::skip]
#[derive(ByteSerializeStack, ByteDeserializeSlice, ByteSerializedLenOf, PartialEq, Clone, Debug, TryInto)]
#[try_into(owned, ref, ref_mut)]
#[byteserde(peek(0, 1))]
pub enum CltOuchPayload {
    #[byteserde(eq(PacketTypeEnterOrder::as_slice()))]
    Enter(EnterOrder),
    #[byteserde(eq(PacketTypeReplaceOrder::as_slice()))]
    Replace(ReplaceOrder),
    #[byteserde(eq(PacketTypeCancelOrder::as_slice()))]
    Cancel(CancelOrder),
    #[byteserde(eq(PacketTypeModifyOrder::as_slice()))]
    Modify(ModifyOrder),
    #[byteserde(eq(PacketTypeAccountQueryRequest::as_slice()))]
    AccQry(AccountQueryRequest),
}
impl SoupBinTcpPayload<CltOuchPayload> for CltOuchPayload {}

pub const SVC_OUCH_MAX_PLD_SIZE: usize = 72; // TODO revise Options fields and remeasure
pub const SVC_OUCH_MAX_FRAME_SIZE: usize = SVC_OUCH_MAX_PLD_SIZE + SOUPBINTCP_MAX_FRAME_SIZE_EXCLUDING_PAYLOAD_DEBUG;

pub const CLT_OUCH_MAX_PLD_SIZE: usize = 51; // TODO revise Options fields and remeasure
pub const CLT_OUCH_MAX_FRAME_SIZE: usize = CLT_OUCH_MAX_PLD_SIZE + SOUPBINTCP_MAX_FRAME_SIZE_EXCLUDING_PAYLOAD_DEBUG;
/// Both [ReplaceOrder] & [OrderReplaced] are serialized as b'U' hence it is impossible to distinguish deserialization type unless they are in two different enums.
#[rustfmt::skip]
#[derive(ByteSerializeStack, ByteDeserializeSlice, ByteSerializedLenOf, PartialEq, Clone, Debug, TryInto)]
#[try_into(owned, ref, ref_mut)]
#[byteserde(peek(0, 1))]
pub enum SvcOuchPayload {
    #[byteserde(eq(PacketTypeOrderAccepted::as_slice()))]
    Accepted(OrderAccepted),
    #[byteserde(eq(PacketTypeOrderExecuted::as_slice()))]
    Executed(OrderExecuted),
    #[byteserde(eq(PacketTypeOrderReplaced::as_slice()))]
    Replaced(OrderReplaced),
    #[byteserde(eq(PacketTypeOrderCanceled::as_slice()))]
    Canceled(OrderCanceled),
    #[byteserde(eq(PacketTypeOrderRejected::as_slice()))]
    Rejected(OrderRejected),
    #[byteserde(eq(PacketTypeOrderModified::as_slice()))]
    Modified(OrderModified),
    #[byteserde(eq(PacketTypeOrderRestated::as_slice()))]
    Restated(OrderRestated),

    #[byteserde(eq(PacketTypeCancelPending::as_slice()))]
    CanPending(CancelPending),
    #[byteserde(eq(PacketTypeCancelReject::as_slice()))]
    CanReject(CancelReject),
    #[byteserde(eq(PacketTypeOrderAiqCanceled::as_slice()))]
    AiqCanceled(OrderAiqCanceled),

    #[byteserde(eq(PacketTypeBrokenTrade::as_slice()))]
    BrokenTrade(BrokenTrade),    
    #[byteserde(eq(PacketTypePriorityUpdate::as_slice()))]
    PriUpdate(PriorityUpdate),
    #[byteserde(eq(PacketTypeAccountQueryResponse::as_slice()))]
    AccQryRes(AccountQueryResponse),
    #[byteserde(eq(PacketTypeSystemEvent::as_slice()))]
    SysEvt(SystemEvent),
}

impl SoupBinTcpPayload<SvcOuchPayload> for SvcOuchPayload {}

pub type CltOuchMsg = CltSoupBinTcpMsg<CltOuchPayload>;
pub type SvcOuchMsg = SvcSoupBinTcpMsg<SvcOuchPayload>;

pub type UniOuchMsg = SoupBinTcpMsg<CltOuchPayload, SvcOuchPayload>;

mod from_clt_pld {
    use super::*;
    impl From<EnterOrder> for CltOuchMsg {
        #[inline(always)]
        fn from(payload: EnterOrder) -> Self {
            CltOuchMsg::udata(CltOuchPayload::Enter(payload))
        }
    }
    impl From<EnterOrder> for UniOuchMsg {
        #[inline(always)]
        fn from(payload: EnterOrder) -> Self {
            UniOuchMsg::Clt(payload.into())
        }
    }
    impl From<ReplaceOrder> for CltOuchMsg {
        #[inline(always)]
        fn from(payload: ReplaceOrder) -> Self {
            CltOuchMsg::udata(CltOuchPayload::Replace(payload))
        }
    }
    impl From<ReplaceOrder> for UniOuchMsg {
        #[inline(always)]
        fn from(payload: ReplaceOrder) -> Self {
            UniOuchMsg::Clt(payload.into())
        }
    }
    impl From<CancelOrder> for CltOuchMsg {
        #[inline(always)]
        fn from(payload: CancelOrder) -> Self {
            CltOuchMsg::udata(CltOuchPayload::Cancel(payload))
        }
    }
    impl From<CancelOrder> for UniOuchMsg {
        #[inline(always)]
        fn from(payload: CancelOrder) -> Self {
            UniOuchMsg::Clt(payload.into())
        }
    }
    impl From<ModifyOrder> for CltOuchMsg {
        #[inline(always)]
        fn from(payload: ModifyOrder) -> Self {
            CltOuchMsg::udata(CltOuchPayload::Modify(payload))
        }
    }
    impl From<ModifyOrder> for UniOuchMsg {
        #[inline(always)]
        fn from(payload: ModifyOrder) -> Self {
            UniOuchMsg::Clt(payload.into())
        }
    }
    impl From<AccountQueryRequest> for CltOuchMsg {
        #[inline(always)]
        fn from(payload: AccountQueryRequest) -> Self {
            CltOuchMsg::udata(CltOuchPayload::AccQry(payload))
        }
    }
    impl From<AccountQueryRequest> for UniOuchMsg {
        #[inline(always)]
        fn from(payload: AccountQueryRequest) -> Self {
            UniOuchMsg::Clt(payload.into())
        }
    }
}

mod from_svc_pld {
    use super::*;
    impl From<OrderAccepted> for SvcOuchMsg {
        #[inline(always)]
        fn from(payload: OrderAccepted) -> Self {
            SvcOuchMsg::udata(SvcOuchPayload::Accepted(payload))
        }
    }
    impl From<OrderAccepted> for UniOuchMsg {
        #[inline(always)]
        fn from(payload: OrderAccepted) -> Self {
            UniOuchMsg::Svc(payload.into())
        }
    }
    impl From<OrderExecuted> for SvcOuchMsg {
        #[inline(always)]
        fn from(payload: OrderExecuted) -> Self {
            SvcOuchMsg::udata(SvcOuchPayload::Executed(payload))
        }
    }
    impl From<OrderExecuted> for UniOuchMsg {
        #[inline(always)]
        fn from(payload: OrderExecuted) -> Self {
            UniOuchMsg::Svc(payload.into())
        }
    }
    impl From<OrderReplaced> for SvcOuchMsg {
        #[inline(always)]
        fn from(payload: OrderReplaced) -> Self {
            SvcOuchMsg::udata(SvcOuchPayload::Replaced(payload))
        }
    }
    impl From<OrderReplaced> for UniOuchMsg {
        #[inline(always)]
        fn from(payload: OrderReplaced) -> Self {
            UniOuchMsg::Svc(payload.into())
        }
    }
    impl From<OrderCanceled> for SvcOuchMsg {
        #[inline(always)]
        fn from(payload: OrderCanceled) -> Self {
            SvcOuchMsg::udata(SvcOuchPayload::Canceled(payload))
        }
    }
    impl From<OrderCanceled> for UniOuchMsg {
        #[inline(always)]
        fn from(payload: OrderCanceled) -> Self {
            UniOuchMsg::Svc(payload.into())
        }
    }
    impl From<OrderRejected> for SvcOuchMsg {
        #[inline(always)]
        fn from(payload: OrderRejected) -> Self {
            SvcOuchMsg::udata(SvcOuchPayload::Rejected(payload))
        }
    }
    impl From<OrderRejected> for UniOuchMsg {
        #[inline(always)]
        fn from(payload: OrderRejected) -> Self {
            UniOuchMsg::Svc(payload.into())
        }
    }
    impl From<OrderModified> for SvcOuchMsg {
        #[inline(always)]
        fn from(payload: OrderModified) -> Self {
            SvcOuchMsg::udata(SvcOuchPayload::Modified(payload))
        }
    }
    impl From<OrderModified> for UniOuchMsg {
        #[inline(always)]
        fn from(payload: OrderModified) -> Self {
            UniOuchMsg::Svc(payload.into())
        }
    }
    impl From<OrderRestated> for SvcOuchMsg {
        #[inline(always)]
        fn from(payload: OrderRestated) -> Self {
            SvcOuchMsg::udata(SvcOuchPayload::Restated(payload))
        }
    }
    impl From<OrderRestated> for UniOuchMsg {
        #[inline(always)]
        fn from(payload: OrderRestated) -> Self {
            UniOuchMsg::Svc(payload.into())
        }
    }
    impl From<CancelPending> for SvcOuchMsg {
        #[inline(always)]
        fn from(payload: CancelPending) -> Self {
            SvcOuchMsg::udata(SvcOuchPayload::CanPending(payload))
        }
    }
    impl From<CancelPending> for UniOuchMsg {
        #[inline(always)]
        fn from(payload: CancelPending) -> Self {
            UniOuchMsg::Svc(payload.into())
        }
    }
    impl From<CancelReject> for SvcOuchMsg {
        #[inline(always)]
        fn from(payload: CancelReject) -> Self {
            SvcOuchMsg::udata(SvcOuchPayload::CanReject(payload))
        }
    }
    impl From<CancelReject> for UniOuchMsg {
        #[inline(always)]
        fn from(payload: CancelReject) -> Self {
            UniOuchMsg::Svc(payload.into())
        }
    }
    impl From<OrderAiqCanceled> for SvcOuchMsg {
        #[inline(always)]
        fn from(payload: OrderAiqCanceled) -> Self {
            SvcOuchMsg::udata(SvcOuchPayload::AiqCanceled(payload))
        }
    }
    impl From<OrderAiqCanceled> for UniOuchMsg {
        #[inline(always)]
        fn from(payload: OrderAiqCanceled) -> Self {
            UniOuchMsg::Svc(payload.into())
        }
    }
    impl From<BrokenTrade> for SvcOuchMsg {
        #[inline(always)]
        fn from(payload: BrokenTrade) -> Self {
            SvcOuchMsg::udata(SvcOuchPayload::BrokenTrade(payload))
        }
    }
    impl From<BrokenTrade> for UniOuchMsg {
        #[inline(always)]
        fn from(payload: BrokenTrade) -> Self {
            UniOuchMsg::Svc(payload.into())
        }
    }
    impl From<PriorityUpdate> for SvcOuchMsg {
        #[inline(always)]
        fn from(payload: PriorityUpdate) -> Self {
            SvcOuchMsg::udata(SvcOuchPayload::PriUpdate(payload))
        }
    }
    impl From<PriorityUpdate> for UniOuchMsg {
        #[inline(always)]
        fn from(payload: PriorityUpdate) -> Self {
            UniOuchMsg::Svc(payload.into())
        }
    }
    impl From<AccountQueryResponse> for SvcOuchMsg {
        #[inline(always)]
        fn from(payload: AccountQueryResponse) -> Self {
            SvcOuchMsg::udata(SvcOuchPayload::AccQryRes(payload))
        }
    }
    impl From<AccountQueryResponse> for UniOuchMsg {
        #[inline(always)]
        fn from(payload: AccountQueryResponse) -> Self {
            UniOuchMsg::Svc(payload.into())
        }
    }
    impl From<SystemEvent> for SvcOuchMsg {
        #[inline(always)]
        fn from(payload: SystemEvent) -> Self {
            SvcOuchMsg::udata(SvcOuchPayload::SysEvt(payload))
        }
    }
    impl From<SystemEvent> for UniOuchMsg {
        #[inline(always)]
        fn from(payload: SystemEvent) -> Self {
            UniOuchMsg::Svc(payload.into())
        }
    }
}

#[cfg(test)]
mod test {

    use crate::{
        model::ouch::{CLT_OUCH_MAX_PLD_SIZE, SVC_OUCH_MAX_PLD_SIZE},
        prelude::*,
    };
    use byteserde::prelude::*;
    use links_core::unittest::setup;
    use log::info;

    // TODO max message length needed to optimize stack serialization assume 512 bytes for now
    #[test]
    fn test_ouch_with_envelope_ser_des() {
        setup::log::configure_compact();

        let enter_ord = EnterOrder::default();
        let replace_ord = ReplaceOrder::from(&enter_ord);
        let cancel_ord = CancelOrder::from(&enter_ord);
        let modify_order = ModifyOrder::from((&enter_ord, Side::buy(), 10.into()));

        let ord_accepted = OrderAccepted::from((&enter_ord, OrderReferenceNumber::default(), OrderState::live()));
        let ord_replaced = OrderReplaced::from((&enter_ord, &replace_ord));
        let ord_canceled = OrderCanceled::from((&enter_ord, &cancel_ord));
        let ord_aqi_canceled = OrderAiqCanceled::from((&enter_ord, 0.into(), CancelAiqReason::default(), 0.into(), 0.0.into(), LiquidityFlag::added(), AiqStrategy::default()));
        let ord_executed = OrderExecuted::from(&enter_ord);
        let brkn_trade = BrokenTrade::from((&enter_ord, 1.into(), BrokenTradeReason::erroneous()));
        let ord_rejected = OrderRejected::from((&enter_ord, RejectReason::halted()));
        let can_pending = CancelPending::from(&enter_ord);
        let can_reject = CancelReject::from(&enter_ord);
        let pri_update = PriorityUpdate::from((&enter_ord, OrderReferenceNumber::default()));
        let ord_modified = OrderModified::from((&enter_ord, 1.into()));
        let ord_rstd = OrderRestated::from((&enter_ord, RestatedReason::refresh_of_display(), 1.into(), 0.0.into(), 1.into()));

        let msg_inp = vec![
            enter_ord.into(),
            replace_ord.into(),
            cancel_ord.into(),
            modify_order.into(),
            AccountQueryRequest::default().into(),
            ord_accepted.into(),
            ord_executed.into(),
            ord_replaced.into(),
            ord_canceled.into(),
            ord_rejected.into(),
            ord_modified.into(),
            ord_rstd.into(),
            can_pending.into(),
            can_reject.into(),
            ord_aqi_canceled.into(),
            brkn_trade.into(),
            pri_update.into(),
            AccountQueryResponse::from(1).into(),
            SystemEvent::start_of_day().into(),
        ];
        let mut ser = ByteSerializerStack::<1024>::default();
        for msg in msg_inp.iter() {
            match msg {
                UniOuchMsg::Clt(msg_inp_inb) => {
                    info!("msg_inp_inb: {:?}", msg_inp_inb);
                    let _ = ser.serialize(msg_inp_inb).unwrap();
                }
                UniOuchMsg::Svc(msg_inp_oub) => {
                    info!("msg_inp_oub: {:?}", msg_inp_oub);
                    let _ = ser.serialize(msg_inp_oub).unwrap();
                }
            }
        }
        let mut des = ByteDeserializerSlice::new(ser.as_slice());

        for ouch in msg_inp.iter() {
            match ouch {
                UniOuchMsg::Clt(msg_inp_inb) => {
                    let msg_out_inb = des.deserialize::<CltOuchMsg>().unwrap();
                    info!("msg_out_inb: {:?}", msg_out_inb);
                    assert_eq!(msg_inp_inb, &msg_out_inb);
                }
                UniOuchMsg::Svc(msg_inp_oub) => {
                    let msg_out_oub = des.deserialize::<SvcOuchMsg>().unwrap();
                    info!("msg_out_oub: {:?}", msg_out_oub);
                    assert_eq!(msg_inp_oub, &msg_out_oub);
                }
            }
        }
        assert!(des.is_empty());
    }

    #[test]
    fn test_ouch5_max_size() {
        setup::log::configure_compact();

        let enter_ord = EnterOrder::default();
        let replace_ord = ReplaceOrder::from(&enter_ord);
        let cancel_ord = CancelOrder::from(&enter_ord);
        let modify_ord = ModifyOrder::from((&enter_ord, Side::buy(), 10.into()));

        let ord_accepted = OrderAccepted::from((&enter_ord, OrderReferenceNumber::default(), OrderState::live()));
        let ord_replaced = OrderReplaced::from((&enter_ord, &replace_ord));
        let ord_canceled = OrderCanceled::from((&enter_ord, &cancel_ord));
        let ord_aqi_canceled = OrderAiqCanceled::from((&enter_ord, 0.into(), CancelAiqReason::default(), 0.into(), 0.0.into(), LiquidityFlag::added(), AiqStrategy::default()));
        let ord_executed = OrderExecuted::from(&enter_ord);
        let brkn_trade = BrokenTrade::from((&enter_ord, 1.into(), BrokenTradeReason::erroneous()));
        let ord_rejected = OrderRejected::from((&enter_ord, RejectReason::halted()));
        let can_pending = CancelPending::from(&enter_ord);
        let can_reject = CancelReject::from(&enter_ord);
        let pri_update = PriorityUpdate::from((&enter_ord, OrderReferenceNumber::default()));
        let ord_modified = OrderModified::from((&enter_ord, 1.into()));
        let ord_rstd = OrderRestated::from((&enter_ord, RestatedReason::refresh_of_display(), 1.into(), 0.0.into(), 1.into()));
        let inb = vec![
            CltOuchPayload::Enter(enter_ord),
            CltOuchPayload::Replace(replace_ord),
            CltOuchPayload::Cancel(cancel_ord),
            CltOuchPayload::Modify(modify_ord),
            CltOuchPayload::AccQry(AccountQueryRequest::default()),
        ];
        let oub = vec![
            SvcOuchPayload::SysEvt(SystemEvent::start_of_day()),
            SvcOuchPayload::Accepted(ord_accepted),
            SvcOuchPayload::Replaced(ord_replaced),
            SvcOuchPayload::Canceled(ord_canceled),
            SvcOuchPayload::AiqCanceled(ord_aqi_canceled),
            SvcOuchPayload::Executed(ord_executed),
            SvcOuchPayload::BrokenTrade(brkn_trade),
            SvcOuchPayload::Rejected(ord_rejected),
            SvcOuchPayload::CanPending(can_pending),
            SvcOuchPayload::CanReject(can_reject),
            SvcOuchPayload::PriUpdate(pri_update),
            SvcOuchPayload::Modified(ord_modified),
            SvcOuchPayload::Restated(ord_rstd),
            SvcOuchPayload::AccQryRes(AccountQueryResponse::from(1)),
        ];

        let inb = inb.into_iter().map(|msg| (msg.byte_len(), msg)).collect::<Vec<_>>();
        for (byte_len, msg) in inb.iter() {
            info!("byte_len: {:>3}, msg: {:?}", byte_len, msg);
        }
        let max_frame_size_clt = inb.iter().map(|(len, _)| *len).max().unwrap();
        info!("max_frame_size_clt: {}", max_frame_size_clt);
        assert_eq!(max_frame_size_clt, CLT_OUCH_MAX_PLD_SIZE);

        let oub = oub.into_iter().map(|msg| (msg.byte_len(), msg)).collect::<Vec<_>>();
        for (byte_len, msg) in oub.iter() {
            info!("byte_len: {:>3}, msg: {:?}", byte_len, msg);
        }
        let max_frame_size_svc = oub.iter().map(|(len, _)| *len).max().unwrap();
        info!("max_frame_size_svc: {}", max_frame_size_svc);
        assert_eq!(max_frame_size_svc, SVC_OUCH_MAX_PLD_SIZE);
    }
}
