#[macro_use]
extern crate criterion;

use criterion::Criterion;

fn send_recv(c: &mut Criterion) {
    let (mut tx, mut rx) = flashfunk_core::util::channel::channel::<u128>(128);
    c.bench_function("util::channel_send_recv_128", move |b| {
        b.iter(|| {
            for i in 0..128u128 {
                let _ = tx.send(i);
                rx.recv().unwrap();
            }
        })
    });
}

criterion_group!(bench, send_recv);

criterion_main!(bench);
