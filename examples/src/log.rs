#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};

use core_affinity::{set_for_current, CoreId};
use flashfunk_core::util::channel::{channel, Sender};
use owned_log::{OwnedLog, Value};

fn main() {
    MyLogger::with_handler(
        |mut value| {
            value.display();
        },
        Some(1),
    );

    struct MyValue;

    impl Value for MyValue {
        fn display(&mut self) {
            println!("are we slow??");
        }
    }

    for _ in 0..99 {
        owned_log::log!(Box::new(MyValue));
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
            let value = Box::new(MyValue) as _;
            let now = std::time::Instant::now();
            owned_log::log!(value);
            total += now.elapsed().as_nanos();
            time += 1;
        }
    }

    println!("average time is {:?} ns", total / 8);

    handle.join().unwrap();
}

struct MyLogger(Sender<Box<dyn Value>>);

impl OwnedLog for MyLogger {
    fn log(&self, value: Box<dyn Value>) {
        self.0.send(value)
    }
}

impl MyLogger {
    pub fn with_handler<F>(mut func: F, id: Option<usize>)
    where
        F: FnMut(Box<dyn Value>) -> () + Send + 'static,
    {
        let (tx, rx) = channel(256);

        owned_log::OWNED_LOGGER
            .with(|logger| logger.set(Box::new(MyLogger(tx)) as _))
            .ok()
            .unwrap();

        std::thread::spawn(move || loop {
            set_for_current(CoreId {
                id: id.unwrap_or(0),
            });
            match rx.recv() {
                Ok(msg) => func(msg),
                Err(_) => std::thread::sleep(std::time::Duration::from_millis(100)),
            }
        });
    }
}
