#[macro_use]
extern crate criterion;

use criterion::Criterion;

fn send_recv_async(c: &mut Criterion) {
    let (tx, rx) = flashfunk_core::util::channel::channel::<u128>(1);
    c.bench_function("util::channel_async_send_recv_128", move |b| {
        b.iter(|| {
            futures::executor::block_on(async {
                for _ in 0..128 {
                    let _ = tx.send(128u128);
                    rx.recv().await.unwrap();
                }
            })
        })
    });
}

criterion_group!(bench, send_recv_async);

criterion_main!(bench);
