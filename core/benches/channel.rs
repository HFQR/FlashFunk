#[macro_use]
extern crate criterion;

use std::sync::Mutex;

use criterion::Criterion;
use flashfunk_core::util::{channel::channel, spin::SpinLock};

fn send_recv(c: &mut Criterion) {
    let (mut tx, mut rx) = channel::<u128>(128);
    c.bench_function("channel_send_recv", move |b| {
        b.iter(|| {
            for i in 0..128u128 {
                let _ = tx.send(i);
                rx.recv().unwrap();
            }
        })
    });
}

fn send_recv_spin_uncontend(c: &mut Criterion) {
    let (tx, mut rx) = channel::<u128>(128);
    let tx = SpinLock::new(tx);
    c.bench_function("channel_send_recv_spin_uncontend", move |b| {
        b.iter(|| {
            for i in 0..128u128 {
                let _ = tx.lock().send(i);
                rx.recv().unwrap();
            }
        })
    });
}

fn send_recv_lock_uncontend(c: &mut Criterion) {
    let (tx, mut rx) = channel::<u128>(128);
    let tx = Mutex::new(tx);
    c.bench_function("channel_send_recv_lock_uncontend", move |b| {
        b.iter(|| {
            for i in 0..128u128 {
                let _ = tx.lock().unwrap().send(i);
                rx.recv().unwrap();
            }
        })
    });
}

criterion_group!(
    bench,
    send_recv,
    send_recv_spin_uncontend,
    send_recv_lock_uncontend
);

criterion_main!(bench);
