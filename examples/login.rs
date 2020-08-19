use ctpbee_rs::app::CtpbeeR;
use actix::Actor;

#[actix_rt::main]
async fn main() {
    let mut account = CtpbeeR::new("ctpbee".to_string());
    // let (addr, x) = account.run_forever();
    // x.await;
}