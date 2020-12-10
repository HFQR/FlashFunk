use chrono::{NaiveTime, Local, Timelike};
use std::process::{Command, Child};
use std::thread::sleep;
use std::time::Duration;

pub struct Timer {
    rest: Vec<NaiveTime>,
    rost: Vec<NaiveTime>,
}


impl Timer {
    pub fn new(stop: &str, start: &str) -> Timer {
        let st = Self::parse(stop);
        let sd = Self::parse(start);


        Timer {
            rest: st,
            rost: sd,
        }
    }

    fn parse(n: &str) -> Vec<NaiveTime> {
        n.split(",").map(|x| {
            NaiveTime::parse_from_str(x, "%H:%M:%S").expect("TIME STR FORMATTER ERROR")
        }).collect::<Vec<NaiveTime>>()
    }

    pub fn run(&mut self, mut command: Command) {
        let mut n = command.spawn().expect("Process run failed, please check your code or command");
        loop {
            let current = Local::now().naive_local().time();

            let alive = n.try_wait().is_ok() && n.try_wait().unwrap().is_none();
            for t in self.rest.iter() {
                if t.minute().eq(&current.minute()) && t.second().eq(&current.second()) && alive {
                    println!("===> Time is: {}, process should be kill", current);
                    n.kill();
                }
            }

            for s in self.rost.iter() {
                if s.minute().eq(&current.minute()) && s.second().eq(&current.second()) {
                    if alive {
                        n.kill();
                    }
                    n = command.spawn().expect("Process run failed, please check your code or command");
                    println!("===> Time is: {}, process should be start", current);
                }
            }


            sleep(Duration::from_secs(1));
            println!("===> Now T am checking it {}", current);
        }
    }
}
