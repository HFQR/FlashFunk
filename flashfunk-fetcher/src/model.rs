use clickhouse_rs::types::{Block, Complex, Row};
use chrono::{Utc, NaiveDateTime, DateTime};
use chrono_tz::Tz;

/// todo：we need to add macro into Tick
#[derive(Clone, Debug)]
pub struct Tick {
    pub local_symbol: String,
    pub volume: i32,
    pub amount: f64,
    pub  open_interest: f64,
    pub low_price: f64,
    pub high_price: f64,
    pub limit_up: f64,
    pub limit_down: f64,
    pub last_price: f64,
    pub ask_price_1: f64,
    pub bid_price_1: f64,
    pub datetime: NaiveDateTime,
    pub ask_volume_1: i32,
    pub bid_volume_1: i32,
}

impl Tick {
    // 遍历column
    pub fn new_with_block(block: Block<Complex>) -> Vec<Tick> {
        block.rows().into_iter().map(|x| { Tick::new(x) }).collect::<Vec<Tick>>()
    }
    fn new(row: Row<Complex>) -> Tick {
        let x: DateTime<Tz> = row.get("datetime").unwrap();
        Tick {
            local_symbol: row.get("local_symbol").unwrap(),
            volume: row.get("volume").unwrap(),
            amount: row.get("amount").unwrap(),
            open_interest: row.get("open_interest").unwrap(),
            low_price: row.get("low_price").unwrap(),
            high_price: row.get("high_price").unwrap(),
            limit_up: row.get("limit_up").unwrap(),
            limit_down: row.get("limit_down").unwrap(),
            last_price: row.get("last_price").unwrap(),
            ask_price_1: row.get("ask_price_1").unwrap(),
            bid_price_1: row.get("bid_price_1").unwrap(),
            datetime: x.naive_local(),
            ask_volume_1: row.get("ask_volume_1").unwrap(),
            bid_volume_1: row.get("bid_volume_1").unwrap(),
        }
    }
}


