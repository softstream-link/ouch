pub mod setup {
    pub mod model {

        use crate::prelude::*;
        
        pub fn svc_msgs_default() -> Vec<SvcOuchMsg> {
            vec![
                SystemEvent::start_of_day().into(),
                OrderAccepted::from((&EnterOrder::default(), OrderReferenceNumber::new(1), OrderState::live())).into(),
                OrderReplaced::from((&EnterOrder::default(), &ReplaceOrder::from(&EnterOrder::default()))).into(),
                OrderCanceled::from((&EnterOrder::default(), &CancelOrder::from((&EnterOrder::default(), 10.into())))).into(),
                OrderAiqCanceled::from((&EnterOrder::default(), 0.into(), CancelAiqReason::default(), 0.into(), 0.0.into(), LiquidityFlag::added(), AiqStrategy::default())).into(),
                OrderExecuted::from(&EnterOrder::default()).into(),
                BrokenTrade::from((&EnterOrder::default(), 0.into(), BrokenTradeReason::erroneous())).into(),
                OrderRejected::from((&EnterOrder::default(), OrderRejectReason::quote_unavailable())).into(),
                CancelPending::from(&EnterOrder::default()).into(),
                CancelReject::from(&EnterOrder::default()).into(),
                PriorityUpdate::from((&EnterOrder::default(), 0.into())).into(),
                OrderModified::from((&EnterOrder::default(), 10.into())).into(),
                OrderRestated::from((&EnterOrder::default(), RestatedReason::refresh_of_display(), 1.into(), 0.0.into(), 1.into())).into(),
                AccountQueryResponse::from(UserRefNumber::from(1)).into(),
                SvcHeartbeat::default().into(),
                Debug::default().into(),
                LoginAccepted::default().into(),
                LoginRejected::not_authorized().into(),
                EndOfSession::default().into(),
            ]
        }

        pub fn clt_msgs_default() -> Vec<CltOuchMsg> {
            vec![
                EnterOrder::default().into(),
                ReplaceOrder::from(&EnterOrder::default()).into(),
                CancelOrder::from((&EnterOrder::default(), 10.into())).into(),
                ModifyOrder::from((&EnterOrder::default(), Side::buy(), 10.into())).into(),
                AccountQueryRequest::default().into(),
                CltHeartbeat::default().into(),
                Debug::default().into(),
                LoginRequest::default().into(),
                LogoutRequest::default().into(),
            ]
        }
    }
}
