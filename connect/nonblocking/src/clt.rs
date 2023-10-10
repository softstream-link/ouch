use ouch_connect_core::prelude::{CltOuchMessenger, CltSoupBinTcp, CLT_OUCH_MAX_FRAME_SIZE};

pub type CltOuch<CallbackRecvSend> =
    CltSoupBinTcp<CltOuchMessenger, CallbackRecvSend, CLT_OUCH_MAX_FRAME_SIZE>;

#[cfg(test)]
mod test {

    use crate::prelude::*;
    use links_core::unittest::setup;
    use log::info;

    #[test]
    fn test_clt_not_connected() {
        setup::log::configure();

        let addr = setup::net::rand_avail_addr_port();

        let res = CltOuch::connect(
            addr,
            setup::net::default_connect_timeout(),
            setup::net::default_connect_retry_after(),
            DevNullCallback::new_ref(),
            Some("ouch/unittest"),
        );
        info!("{:?} not connected", res);
        assert!(res.is_err());
    }
}
