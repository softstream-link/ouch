use ouch_connect_core::prelude::{SvcSoupBinTcp, OUCH_MAX_FRAME_SIZE};

pub type SvcOuch<Protocol, CallbackRecvSend> = SvcSoupBinTcp<Protocol, CallbackRecvSend, OUCH_MAX_FRAME_SIZE>;

#[cfg(test)]
mod test {

    use std::num::NonZeroUsize;

    use crate::prelude::*;
    use links_core::unittest::setup;
    use log::info;

    #[test]
    fn test_clt_svc_connected() {
        setup::log::configure_level(log::LevelFilter::Info);

        let addr = setup::net::rand_avail_addr_port();
        let protocol = SvcOuchProtocolManual::default();
        let mut svc = SvcOuch::bind(addr, NonZeroUsize::new(1).unwrap(), LoggerCallback::new_ref(), protocol, Some("ouch/unittest")).unwrap();
        info!("svc: {}", svc);

        let protocol = CltOuchProtocolManual::default();
        let mut clt = CltOuch::connect(addr, setup::net::default_connect_timeout(), setup::net::default_connect_retry_after(), LoggerCallback::new_ref(), protocol, Some("ouch/unittest")).unwrap();
        info!("clt: {}", clt);

        // svc.pool_accept_busywait_timeout(setup::net::default_connect_timeout()).unwrap().unwrap_accepted();
        svc.accept_into_pool_busywait_timeout(setup::net::default_connect_timeout()).unwrap().unwrap_accepted();
        info!("svc: {}", svc);

        let mut clt_msg = EnterOrder::default().into();
        // let mut clt_msg = CltSoupBinTcpMsg::Login(LoginRequest::default());
        clt.send_busywait_timeout(&mut clt_msg, setup::net::default_connect_timeout()).unwrap().unwrap_completed();

        let svc_msg = svc.recv_busywait_timeout(setup::net::default_connect_timeout()).unwrap().unwrap_completed_some();

        assert_eq!(clt_msg, svc_msg);
    }
}
