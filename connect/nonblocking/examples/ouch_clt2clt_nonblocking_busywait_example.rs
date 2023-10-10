use std::{error::Error, num::NonZeroUsize, thread::Builder, time::Instant};

use links_core::{
    fmt_num,
    unittest::setup::{self},
};
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
            let svc = SvcOuch::bind(
                addr,
                DevNullCallback::new_ref(),
                NonZeroUsize::new(1).unwrap(),
                Some("ouch/clt"),
            )
            .unwrap();

            info!("svc: {}", svc);
            let mut clt = svc
                .accept_busywait_timeout(setup::net::default_connect_timeout())
                .unwrap()
                .unwrap();
            info!("clt: {}", clt);

            let mut order_accepted: SvcOuchMsg = OrderAccepted::from(&EnterOrder::default()).into();
            let mut msg_recv_count = 0_usize;
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
        Some("ouch/venue"),
    )
    .unwrap();
    info!("clt {}", clt);

    let mut enter_order: CltOuchMsg = EnterOrder::default().into();

    // send the first message to the server to establish connection
    clt.send_busywait_timeout(&mut enter_order, setup::net::default_connect_timeout())?;
    let now = Instant::now();
    for _ in 0..WRITE_N_TIMES {
        clt.send_busywait(&mut enter_order)?;
        let _msg = clt.recv_busywait().unwrap().unwrap();
    }
    let elapsed = now.elapsed();

    drop(clt); // close the connection and allow the acceptor to exit
    let msg_recv_count = svc_jh.join().unwrap();
    info!(
        "msg_send_count: {}, msg_recv_count: {}, per/write {:?}, total: {:?}, round-trips/sec: ~{}",
        fmt_num!(WRITE_N_TIMES),
        fmt_num!(msg_recv_count),
        elapsed / WRITE_N_TIMES as u32,
        elapsed,
        fmt_num!((1_f64 / (elapsed / WRITE_N_TIMES as u32).as_secs_f64()).round() as usize)
    );
    assert_eq!(msg_recv_count, WRITE_N_TIMES); // +1 for the first message to connect
    Ok(())
}
