use ctpbee_rs::app::CtpbeeR;

#[actix_rt::main]
async fn main() {
    let account = CtpbeeR::new("ctpbee".to_string());
    let (addr, x) = account.run_forever();
    x.await;
}