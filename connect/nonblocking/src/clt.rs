use ouch_connect_core::prelude::{CltSoupBinTcp, OUCH_MAX_FRAME_SIZE};

pub type CltOuch<Protocol, CallbackRecvSend> = CltSoupBinTcp<Protocol, CallbackRecvSend, OUCH_MAX_FRAME_SIZE>;

#[cfg(test)]
mod test {

    use crate::prelude::*;
    use links_core::unittest::setup;
    use log::info;
    use ouch_connect_core::core::CltOuchProtocolManual;

    #[test]
    fn test_clt_not_connected() {
        setup::log::configure();

        let addr = setup::net::rand_avail_addr_port();
        let protocol = CltOuchProtocolManual::default();

        let res = CltOuch::connect(addr, setup::net::default_connect_timeout(), setup::net::default_connect_retry_after(), DevNullCallback::new_ref(), protocol, Some("ouch/unittest"));
        // let res = CltOuchManual::connect(addr, setup::net::default_connect_timeout(), setup::net::default_connect_retry_after(), DevNullCallback::new_ref(), protocol, Some("ouch/unittest"));
        info!("{:?} not connected", res);
        assert!(res.is_err());
    }
}
