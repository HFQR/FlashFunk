use ctpbee_rs::app::CtpbeeR;
use actix::Actor;

fn main() {
    let account = CtpbeeR::new("ctpbee".to_string());

    account.start();
}