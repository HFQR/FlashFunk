use chrono::{NaiveTime, Local};

pub struct Timer {
    restart_time: Vec<NaiveTime>
}

impl Timer {
    fn run() {
        let pro = process::spawn(move || {
            // do something here
        });


        loop {
            let current = Local::now().naive_local().time();
            println!("{}", pro.pid);
        }
    }
}