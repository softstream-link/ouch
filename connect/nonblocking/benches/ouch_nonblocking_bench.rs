use std::{num::NonZeroUsize, thread::Builder};

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use links_core::unittest::setup;
use log::info;
use ouch_connect_nonblocking::prelude::*;

fn soupbintcp_ouch_enter_order_send(c: &mut Criterion) {
    setup::log::configure_level(log::LevelFilter::Info);

    let addr = setup::net::rand_avail_addr_port();
    Builder::new()
        .name("Svc-Thread".to_string())
        .spawn(move || {
            let protocol = SvcOuchProtocolManual::default();
            let mut svc = SvcOuch::bind(addr, NonZeroUsize::new(1).unwrap(), DevNullCallback::new_ref(), protocol, Some("ouch/venue")).unwrap();
            info!("svc {}", svc);
            svc.accept_into_pool_busywait_timeout(setup::net::default_connect_timeout()).unwrap().unwrap_accepted();
            info!("svc {}", svc);

            while let Ok(opt) = svc.recv_busywait() {
                if let None = opt {
                    break;
                }
            }
        })
        .unwrap();

    let protocol = CltOuchProtocolManual::default();
    let mut clt = CltOuch::connect(addr, setup::net::default_connect_timeout(), setup::net::default_connect_retry_after(), DevNullCallback::new_ref(), protocol, Some("ouch/clt")).unwrap();

    info!("clt {}", clt);

    let mut enter_order = EnterOrder::default().into();
    c.bench_function("soupbintcp_ouch_enter_order_send", |b| {
        b.iter(|| {
            black_box({
                clt.send_busywait(&mut enter_order).unwrap();
            })
        })
    });
}

fn soupbintcp_ouch_order_accepted_recv(c: &mut Criterion) {
    setup::log::configure_level(log::LevelFilter::Info);

    let addr = setup::net::rand_avail_addr_port();
    Builder::new()
        .name("Svc-Thread".to_string())
        .spawn(move || {
            let protocol = SvcOuchProtocolManual::default();
            let svc = SvcOuch::bind(addr, NonZeroUsize::new(1).unwrap(), DevNullCallback::new_ref(), protocol, Some("ouch/venue")).unwrap();
            info!("svc {}", svc);

            let mut clt = svc.accept_busywait_timeout(setup::net::default_connect_timeout()).unwrap().unwrap_accepted();
            info!("svc {}", svc);

            let mut order_accepted: SvcOuchMsg = OrderAccepted::from((&EnterOrder::default(), OrderReferenceNumber::new(1), OrderState::live())).into();

            // while let Ok(_status) = svc.send_nonblocking(&mut order_accepted) {}
            loop {
                match clt.send_busywait(&mut order_accepted) {
                    Ok(_) => continue,
                    Err(_) => break,
                }
            }
        })
        .unwrap();

    let protocol = CltOuchProtocolManual::default();
    let mut clt = CltOuch::connect(addr, setup::net::default_connect_timeout(), setup::net::default_connect_retry_after(), DevNullCallback::new_ref(), protocol, Some("ouch/clt")).unwrap();

    info!("clt {}", clt);

    let _res = clt.recv_busywait(); // establish connection
    c.bench_function("soupbintcp_ouch_order_accepted_recv", |b| {
        b.iter(|| {
            black_box({
                let _order_accepter = clt.recv_busywait().unwrap();
            })
        })
    });
}

fn soupbintcp_ouch_enter_order_accepted_round_trip(c: &mut Criterion) {
    setup::log::configure_level(log::LevelFilter::Info);

    let addr = setup::net::rand_avail_addr_port();
    Builder::new()
        .name("Svc-Thread".to_string())
        .spawn(move || {
            let protocol = SvcOuchProtocolManual::default();
            let svc = SvcOuch::bind(addr, NonZeroUsize::new(1).unwrap(), DevNullCallback::new_ref(), protocol, Some("ouch/venue")).unwrap();

            let mut clt = svc.accept_busywait_timeout(setup::net::default_connect_timeout()).unwrap().unwrap_accepted();

            let mut order_accepted: SvcOuchMsg = OrderAccepted::from((&EnterOrder::default(), OrderReferenceNumber::new(1), OrderState::live())).into();

            loop {
                match clt.recv_busywait().unwrap() {
                    None => break,
                    Some(_) => clt.send_busywait(&mut order_accepted).unwrap(),
                }
            }
        })
        .unwrap();

    let protocol = CltOuchProtocolManual::default();
    let mut clt = CltOuch::connect(addr, setup::net::default_connect_timeout(), setup::net::default_connect_retry_after(), DevNullCallback::new_ref(), protocol, Some("ouch/clt")).unwrap();

    info!("clt {}", clt);
    let mut enter_order: CltOuchMsg = EnterOrder::default().into();

    c.bench_function("soupbintcp_ouch_enter_order_accepted_round_trip", |b| {
        b.iter(|| {
            black_box({
                clt.send_busywait(&mut enter_order).unwrap();
                let _order_accepted = clt.recv_busywait().unwrap().unwrap();
            })
        })
    });
}

// criterion_group!(benches, ouch_enter_order_accepted_round_trip);
criterion_group!(benches, soupbintcp_ouch_enter_order_send, soupbintcp_ouch_order_accepted_recv, soupbintcp_ouch_enter_order_accepted_round_trip);
criterion_main!(benches);
