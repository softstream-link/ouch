use byteserde::prelude::*;
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use ouch_model::prelude::*;

fn ouch_enter_order(c: &mut Criterion) {
    // let msg_inp = EnterOrder::default();
    // c.bench_function("ouch_enter_order_ser", |b| {
    //     b.iter(|| {
    //         black_box({
    //             let _: ([u8; CLT_OUCH_MAX_FRAME_SIZE], usize) = to_bytes_stack(&msg_inp).unwrap();
    //         })
    //     })
    // });

    // let (buf, size): ([u8; CLT_OUCH_MAX_FRAME_SIZE], usize) = to_bytes_stack(&msg_inp).unwrap();
    // c.bench_function("ouch_enter_order_des", |b| {
    //     b.iter(|| {
    //         black_box({
    //             let _: EnterOrder = from_slice(&buf[..size]).unwrap();
    //         })
    //     })
    // });

    // TODO why commenting everything below this line make ser of ouch_order 9ns vs 33ns if below is also included
    let msg_inp: CltSoupBinTcpMsg<CltOuchPayload> = EnterOrder::default().into();
    c.bench_function("soupbintcp_ouch_enter_order_ser", |b| {
        b.iter(|| {
            black_box({
                let _: ([u8; CLT_OUCH_MAX_FRAME_SIZE], usize) = to_bytes_stack(&msg_inp).unwrap();
            })
        })
    });

    let (buf, size): ([u8; CLT_OUCH_MAX_FRAME_SIZE], usize) = to_bytes_stack(&msg_inp).unwrap();
    c.bench_function("soupbintcp_ouch_enter_order_des", |b| {
        b.iter(|| {
            black_box({
                let _: CltSoupBinTcpMsg<CltOuchPayload> = from_slice(&buf[..size]).unwrap();
            })
        })
    });
}

fn ouch_order_accepted(c: &mut Criterion) {
    // let msg_inp = OrderAccepted::from(&EnterOrder::default());
    // c.bench_function("ouch_order_accepted_ser", |b| {
    //     b.iter(|| {
    //         black_box({
    //             let _: ([u8; CLT_OUCH_MAX_FRAME_SIZE], usize) = to_bytes_stack(&msg_inp).unwrap();
    //         })
    //     })
    // });

    // let (buf, size): ([u8; CLT_OUCH_MAX_FRAME_SIZE], usize) = to_bytes_stack(&msg_inp).unwrap();
    // c.bench_function("ouch_order_accepted_des", |b| {
    //     b.iter(|| {
    //         black_box({
    //             let _: OrderAccepted = from_slice(&buf[..size]).unwrap();
    //         })
    //     })
    // });

    let msg_inp: SvcSoupBinTcpMsg<SvcOuchPayload> = OrderAccepted::from(&EnterOrder::default()).into();
    c.bench_function("soupbintcp_ouch_order_accepted_ser", |b| {
        b.iter(|| {
            black_box({
                let _: ([u8; CLT_OUCH_MAX_FRAME_SIZE], usize) = to_bytes_stack(&msg_inp).unwrap();
            })
        })
    });

    let (buf, size): ([u8; CLT_OUCH_MAX_FRAME_SIZE], usize) = to_bytes_stack(&msg_inp).unwrap();
    c.bench_function("soupbintcp_ouch_order_accepted_des", |b| {
        b.iter(|| {
            black_box({
                let _: SvcSoupBinTcpMsg<SvcOuchPayload> = from_slice(&buf[..size]).unwrap();
            })
        })
    });
}

criterion_group!(benches, ouch_enter_order, ouch_order_accepted);
criterion_main!(benches);
