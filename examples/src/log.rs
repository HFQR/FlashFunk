use std::sync::{
    atomic::{AtomicUsize, Ordering},
    Arc,
};

use owned_log::{async_impl::TokioLogger, Value};

fn main() {
    struct MyValue(Arc<AtomicUsize>);

    impl Value for MyValue {
        fn display(&mut self) {
            self.0.store(996, Ordering::SeqCst);
        }
    }

    TokioLogger::builder().build().unwrap();

    let value = Arc::new(AtomicUsize::new(0));

    owned_log::log!(MyValue(value.clone()));

    std::thread::sleep(std::time::Duration::from_secs(1));

    assert_eq!(value.load(Ordering::Relaxed), 996);
}
