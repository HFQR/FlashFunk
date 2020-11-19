use clickhouse_rs::types::{Block, Complex, Row};
use chrono::{NaiveDateTime, DateTime};
use chrono_tz::Tz;

#[derive(Clone, Debug)]
pub struct Tick {
    pub local_symbol: String,
    pub volume: i32,
    pub amount: f64,
    pub open_interest: f64,
    pub low_price: f64,
    pub high_price: f64,
    pub limit_up: f64,
    pub limit_down: f64,
    pub last_price: f64,
    pub ask_price_1: f64,
    pub ask_price_2: f64,
    pub ask_price_3: f64,
    pub ask_price_4: f64,
    pub ask_price_5: f64,
    pub bid_price_1: f64,
    pub bid_price_2: f64,
    pub bid_price_3: f64,
    pub bid_price_4: f64,
    pub bid_price_5: f64,
    pub datetime: NaiveDateTime,
    pub ask_volume_1: i32,
    pub ask_volume_2: i32,
    pub ask_volume_3: i32,
    pub ask_volume_4: i32,
    pub ask_volume_5: i32,
    pub bid_volume_1: i32,
    pub bid_volume_2: i32,
    pub bid_volume_3: i32,
    pub bid_volume_4: i32,
    pub bid_volume_5: i32,
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
            ask_price_2: row.get("ask_price_2").unwrap(),
            ask_price_3: row.get("ask_price_3").unwrap(),
            ask_price_4: row.get("ask_price_4").unwrap(),
            ask_price_5: row.get("ask_price_5").unwrap(),
            bid_price_1: row.get("bid_price_1").unwrap(),
            bid_price_2: row.get("bid_price_2").unwrap(),
            bid_price_3: row.get("bid_price_3").unwrap(),
            bid_price_4: row.get("bid_price_4").unwrap(),
            bid_price_5: row.get("bid_price_5").unwrap(),
            datetime: x.naive_local(),
            ask_volume_1: row.get("ask_volume_1").unwrap(),
            ask_volume_2: row.get("ask_volume_2").unwrap(),
            ask_volume_3: row.get("ask_volume_3").unwrap(),
            ask_volume_4: row.get("ask_volume_4").unwrap(),
            ask_volume_5: row.get("ask_volume_5").unwrap(),
            bid_volume_1: row.get("bid_volume_1").unwrap(),
            bid_volume_2: row.get("bid_volume_2").unwrap(),
            bid_volume_3: row.get("bid_volume_3").unwrap(),
            bid_volume_4: row.get("bid_volume_4").unwrap(),
            bid_volume_5: row.get("bid_volume_5").unwrap(),
        }
    }
}


