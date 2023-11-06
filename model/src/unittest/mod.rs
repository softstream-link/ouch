pub mod setup {
    pub mod model {
        use crate::prelude::*;

        // #[rustfmt::skip]
        // pub fn svc_msgs_default<P>() -> Vec<SvcSoupBinTcpMsg<P>>
        // where
        //     P: SoupBinTcpPayload<P> + Default,
        // {
        //     vec![
        //         SvcSoupBinTcpMsg::Hbeat(SvcHeartbeat::default()),
        //         SvcSoupBinTcpMsg::Dbg(Debug::default()),
        //         SvcSoupBinTcpMsg::LoginAccepted(LoginAccepted::default()),
        //         SvcSoupBinTcpMsg::LoginRejected(LoginRejected::not_authorized()),
        //         SvcSoupBinTcpMsg::EndOfSession(EndOfSession::default()),
        //         SvcSoupBinTcpMsg::SPayload(SPayload::new(P::default())),
        //         SvcSoupBinTcpMsg::UPayload(UPayload::new(P::default())),
        //     ]
        // }

        // #[rustfmt::skip]
        pub fn clt_msgs_default<P>() -> Vec<CltOuchMsg>
        where P: SoupBinTcpPayload<P>+Default {
            vec![
                EnterOrder::default().into(),
                ReplaceOrder::from(&EnterOrder::default()).into(),
                CancelOrder::from((&EnterOrder::default(), 10.into())).into(),
                ModifyOrder::from((&EnterOrder::default(), Side::sell(), 10.into())).into(),
                // CltSoupBinTcpMsg::Hbeat(CltHeartbeat::default()),
                // CltSoupBinTcpMsg::Dbg(Debug::default()),
                // CltSoupBinTcpMsg::Login(LoginRequest::default()),
                // CltSoupBinTcpMsg::Logout(LogoutRequest::default()),
                // CltSoupBinTcpMsg::SPayload(SPayload::new(P::default())),
                // CltSoupBinTcpMsg::UPayload(UPayload::new(P::default())),
            ]
        }
    }
}


