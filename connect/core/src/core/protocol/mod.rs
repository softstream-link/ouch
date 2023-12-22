use bytes::BytesMut;
use ouch_model::prelude::*;
use soupbintcp_connect_nonblocking::prelude::*;
use std::{io::Error, time::Duration};

// Protocol
pub type CltOuchProtocolManual = CltSoupBinTcpProtocolManual<SvcOuchPayload, CltOuchPayload>;
pub type CltOuchProtocolIsConnected = CltSoupBinTcpProtocolIsConnected<SvcOuchPayload, CltOuchPayload>;

pub type SvcOuchProtocolManual = SvcSoupBinTcpProtocolManual<CltOuchPayload, SvcOuchPayload>;
pub type SvcOuchProtocolIsConnected = SvcSoupBinTcpProtocolIsConnected<CltOuchPayload, SvcOuchPayload>;

pub type CltOuchProtocolAuto = CltSoupBinTcpProtocolAuto<SvcOuchPayload, CltOuchPayload>;

type SvcSoupBinTcpLayerProtocolAuto = SvcSoupBinTcpProtocolAuto<CltOuchPayload, SvcOuchPayload>;

/// Implements Ouch protocol for server side.
///
/// Uses all features of [SvcSoupBinTcpProtocolAuto] while adding the following:
/// * [`Self::on_send`] - updates timestamps on all [SPayload] message types right before delegating to [`SvcSoupBinTcpProtocolAuto::on_send`]
///
#[derive(Debug, Clone)]
pub struct SvcOuchProtocolAuto {
    inner: SvcSoupBinTcpLayerProtocolAuto,
}
impl SvcOuchProtocolAuto {
    #[inline(always)]
    pub fn new(username: UserName, password: Password, session_id: SessionId, io_timeout: Duration, svc_max_hbeat_interval: Duration) -> Self {
        Self {
            inner: SvcSoupBinTcpLayerProtocolAuto::new(username, password, session_id, io_timeout, svc_max_hbeat_interval),
        }
    }
}
impl Framer for SvcOuchProtocolAuto {
    #[inline(always)]
    fn get_frame_length(bytes: &mut BytesMut) -> Option<usize> {
        SoupBinTcpFramer::get_frame_length(bytes)
    }
}
impl Messenger for SvcOuchProtocolAuto {
    type RecvT = <SvcSoupBinTcpLayerProtocolAuto as Messenger>::RecvT;
    type SendT = <SvcSoupBinTcpLayerProtocolAuto as Messenger>::SendT;

    #[inline(always)]
    fn serialize<const MAX_MSG_SIZE: usize>(msg: &Self::SendT) -> Result<([u8; MAX_MSG_SIZE], usize), std::io::Error> {
        SvcSoupBinTcpLayerProtocolAuto::serialize(msg)
    }
    #[inline(always)]
    fn deserialize(frame: &[u8]) -> Result<Self::RecvT, std::io::Error> {
        SvcSoupBinTcpLayerProtocolAuto::deserialize(frame)
    }
}
impl ProtocolCore for SvcOuchProtocolAuto {
    #[inline(always)]
    fn is_connected(&self) -> bool {
        self.inner.is_connected()
    }
    #[inline(always)]
    fn on_connect<C: SendNonBlocking<<Self as Messenger>::SendT> + ReSendNonBlocking<<Self as Messenger>::SendT> + RecvNonBlocking<<Self as Messenger>::RecvT> + ConnectionId>(&self, con: &mut C) -> Result<(), Error> {
        self.inner.on_connect(con)
    }
    #[inline(always)]
    fn on_disconnect(&self) -> Option<(std::time::Duration, <Self as Messenger>::SendT)> {
        self.inner.on_disconnect()
    }
    #[inline(always)]
    fn on_error<I: ConnectionId>(&self, who: &I, msg: &<Self as Messenger>::SendT, e: &std::io::Error) {
        self.inner.on_error(who, msg, e)
    }
    #[inline(always)]
    fn on_recv<I: ConnectionId>(&self, who: &I, msg: &<Self as Messenger>::RecvT) {
        self.inner.on_recv(who, msg)
    }

    #[inline(always)]
    fn on_send<I: ConnectionId>(&self, who: &I, msg: &mut <Self as Messenger>::SendT) {
        // update timestamps, right before sending
        match msg {
            SvcSoupBinTcpMsg::SPayload(SPayload { payload, .. }) => match payload {
                SvcOuchPayload::SystemEvent(SystemEvent { timestamp, .. })                      // _00
                | SvcOuchPayload::OrderAccepted(OrderAccepted { timestamp, .. })                // _01
                | SvcOuchPayload::OrderReplaced(OrderReplaced { timestamp, .. })                // _02
                | SvcOuchPayload::OrderCanceled(OrderCanceled { timestamp, .. })                // _03
                | SvcOuchPayload::OrderAiqCanceled(OrderAiqCanceled { timestamp, .. })          // _04
                | SvcOuchPayload::OrderExecuted(OrderExecuted { timestamp, .. })                // _05          
                | SvcOuchPayload::BrokenTrade(BrokenTrade { timestamp, .. })                    // _06
                | SvcOuchPayload::OrderRejected(OrderRejected { timestamp, .. })                // _07
                | SvcOuchPayload::CancelPending(CancelPending { timestamp, .. })                // _08
                | SvcOuchPayload::CancelReject(CancelReject { timestamp, .. })                  // _09
                | SvcOuchPayload::PriorityUpdate(PriorityUpdate { timestamp, .. })              // _10
                | SvcOuchPayload::OrderModified(OrderModified { timestamp, .. })                // _11
                | SvcOuchPayload::OrderRestated(OrderRestated { timestamp, .. })                // _12
                | SvcOuchPayload::AccountQueryResponse(AccountQueryResponse { timestamp, .. })  // _13
                => {
                    *timestamp = Default::default();
                }
            },
            _ => {}
        }
        self.inner.on_send(who, msg)
    }
    #[inline(always)]
    fn on_sent<I: ConnectionId>(&self, who: &I, msg: &<Self as Messenger>::SendT) {
        self.inner.on_sent(who, msg)
    }
    #[inline(always)]
    fn on_wouldblock<I: ConnectionId>(&self, who: &I, msg: &<Self as Messenger>::SendT) {
        self.inner.on_wouldblock(who, msg)
    }
}
impl Protocol for SvcOuchProtocolAuto {
    #[inline(always)]
    fn conf_heart_beat_interval(&self) -> Option<std::time::Duration> {
        self.inner.conf_heart_beat_interval()
    }
    #[inline(always)]
    fn send_heart_beat<S: SendNonBlocking<Self::SendT> + ConnectionId>(&self, sender: &mut S) -> Result<SendStatus, Error> {
        self.inner.send_heart_beat(sender)
    }
    #[inline(always)]
    fn send_reply<S: SendNonBlocking<<Self as Messenger>::SendT> + ConnectionId>(&self, msg: &<Self as Messenger>::RecvT, sender: &mut S) -> Result<(), Error> {
        self.inner.send_reply(msg, sender)
    }
}
