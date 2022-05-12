#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};

use owned_log::{OwnedLog, ValueType, Value};
use flashfunk_core::util::channel::{channel, Sender};

fn main() {
    struct MyLogger(Sender<Box<dyn Value>>);

    impl OwnedLog for MyLogger {
        fn log(&self, value: Box<dyn Value>) {
            self.0.send(value)
        }
    }

    struct MyValue;

    impl Value for MyValue {
        fn display(&mut self) {
            todo!()
        }
    }

    let (tx, rx) = channel(256);

    owned_log::OWNED_LOGGER.with(|logger| logger.set(Box::new(MyLogger(tx)) as _)).ok().unwrap();

    for _ in 0..99 {
        owned_log::log!(Box::new(ValueType::default()));
    }

    let flag = Arc::new(AtomicBool::new(false));

    let flag1 = flag.clone();
    let handle = std::thread::spawn(move || {
        for _ in 0..8 {
            std::thread::sleep(std::time::Duration::from_millis(500));
            flag1.store(true, Ordering::Relaxed);
        }
    });

    let mut total = 0u128;
    let mut time = 0;

    while time < 8 {
        if flag.swap(false, Ordering::SeqCst) {
            let value = ValueType::default();
            let now = std::time::Instant::now();
            let value = Box::new(value) as _;
            owned_log::log!(value);
            total += now.elapsed().as_nanos();
            time += 1;
        }
    }

    println!("average time is {:?} ns", total / 8);

    handle.join().unwrap();
}
