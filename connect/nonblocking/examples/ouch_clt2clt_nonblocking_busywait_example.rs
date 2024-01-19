use std::{error::Error, num::NonZeroUsize, thread::Builder, time::Instant};

use links_nonblocking::prelude::{unittest::setup, *};
use log::info;
use ouch_connect_nonblocking::prelude::*;

fn main() -> Result<(), Box<dyn Error>> {
    run()
}
#[test]
fn test() -> Result<(), Box<dyn Error>> {
    run()
}
fn run() -> Result<(), Box<dyn Error>> {
    setup::log::configure_level(log::LevelFilter::Info);

    let addr = setup::net::rand_avail_addr_port();
    const WRITE_N_TIMES: usize = 100_000;

    let svc_jh = Builder::new()
        .name("Acceptor-Thread".to_owned())
        .spawn(move || {
            let svc = SvcOuch::bind(addr, NonZeroUsize::new(1).unwrap(), DevNullCallback::new_ref(), SvcOuchProtocolManual::default(), Some("ouch/clt")).unwrap();

            info!("svc: {}", svc);
            let mut clt = svc.accept_busywait_timeout(setup::net::default_connect_timeout()).unwrap().unwrap_accepted();
            info!("clt: {}", clt);

            let mut order_accepted: SvcOuchMsg = OrderAccepted::from((&EnterOrder::default(), OrderReferenceNumber::new(1), OrderState::live())).into();
            let mut msg_recv_count = 0;
            loop {
                if let Ok(Some(_msg)) = clt.recv_busywait() {
                    msg_recv_count += 1;
                    clt.send_busywait(&mut order_accepted).unwrap();
                    continue;
                } else {
                    break;
                }
            }
            msg_recv_count
        })
        .unwrap();

    let mut clt = CltOuch::connect(
        addr,
        setup::net::default_connect_timeout(),
        setup::net::default_connect_retry_after(),
        DevNullCallback::new_ref(),
        CltOuchProtocolManual::default(),
        Some("ouch/venue"),
    )
    .unwrap();
    info!("clt {}", clt);

    let mut enter_order: CltOuchMsg = EnterOrder::default().into();

    let now = Instant::now();
    for _ in 0..WRITE_N_TIMES {
        clt.send_busywait_timeout(&mut enter_order, setup::net::default_connect_timeout())?;
        let _msg = clt.recv_busywait().unwrap().unwrap();
    }
    let elapsed = now.elapsed();

    drop(clt); // close the connection and allow the acceptor to exit
    let msg_recv_count = svc_jh.join().unwrap();
    info!(
        "msg_send_count: {}, msg_recv_count: {}, per/write {:?}, total: {:?}, round-trips/sec: ~{}",
        WRITE_N_TIMES,
        msg_recv_count,
        elapsed / WRITE_N_TIMES as u32,
        elapsed,
        (1_f64 / (elapsed / WRITE_N_TIMES as u32).as_secs_f64()).round()
    );
    assert_eq!(msg_recv_count, WRITE_N_TIMES); // +1 for the first message to connect
    Ok(())
}
