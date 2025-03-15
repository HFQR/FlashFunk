#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};

use core_affinity::{set_for_current, CoreId};
use crossbeam_queue::ArrayQueue;
use owned_log::{OwnedLog, Value};

fn main() {
    MyLogger::with_handler(
        |mut value| {
            let value = value.downcast_mut::<MyValue>().unwrap();
            println!("value is {:?}", value.0);
            value.display();
        },
        Some(1),
    );

    struct MyValue(usize);

    impl Value for MyValue {
        fn display(&mut self) {
            println!("are we slow??");
        }
    }

    for _ in 0..99 {
        owned_log::log!(MyValue(1));
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
            let value = MyValue(2);
            let now = std::time::Instant::now();
            owned_log::log!(value);
            total += now.elapsed().as_nanos();
            time += 1;
        }
    }

    println!("average time is {:?} ns", total / 8);

    handle.join().unwrap();
}

struct MyLogger(Arc<ArrayQueue<Box<dyn Value>>>);

impl OwnedLog for MyLogger {
    fn log(&self, value: Box<dyn Value>) {
        self.0.push(value).ok().unwrap();
    }
}

impl MyLogger {
    pub fn with_handler<F>(mut func: F, id: Option<usize>)
    where
        F: FnMut(Box<dyn Value>) -> () + Send + 'static,
    {
        let queue = Arc::new(ArrayQueue::new(256));

        owned_log::OWNED_LOGGER
            .set(Arc::new(MyLogger(queue.clone())) as _)
            .ok()
            .unwrap();

        std::thread::spawn(move || {
            set_for_current(CoreId {
                id: id.unwrap_or(0),
            });
            loop {
                match queue.pop() {
                    Some(msg) => func(msg),
                    None => std::thread::sleep(std::time::Duration::from_millis(100)),
                }
            }
        });
    }
}
