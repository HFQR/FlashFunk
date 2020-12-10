use timer::Timer;
use std::process::Command;

fn main() {
    let mut tim = Timer::new("14:52:00", "14:53:00");
    let command = Command::new("cmd");
    tim.run(command)
}