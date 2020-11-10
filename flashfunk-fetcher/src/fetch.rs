extern crate chrono;
extern crate chrono_tz;
extern crate clickhouse_rs;
extern crate tokio;

use clickhouse_rs::types::{Block};
use clickhouse_rs::{Pool};
use std::error::Error;
use crate::model::Tick;
use tokio::prelude::*;
use std::ops::{Deref, DerefMut};
use std::borrow::BorrowMut;


pub struct Client {
    pool: Pool
}

impl Client {
    pub fn new(uri: &str) -> Self {
        Client {
            pool: Pool::new(uri)
        }
    }
}

impl Deref for Client {
    type Target = Pool;

    fn deref(&self) -> &Self::Target {
        &self.pool
    }
}

impl DerefMut for Client {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.pool.borrow_mut()
    }
}

pub(crate) fn run<F, T, U>(future: F) -> Result<T, U>
    where
        F: Future<Item=T, Error=U> + Send + 'static,
        T: Send + 'static,
        U: Send + 'static,
{
    let mut runtime = tokio::runtime::Runtime::new().unwrap();
    let result = runtime.block_on(future);
    runtime.shutdown_on_idle().wait().unwrap();
    result
}

pub fn fetch_tick(code: &'static str, start: &'static str, end: &'static str, client: &Client) -> Vec<Tick> {
    let sql = format!("SELECT * FROM tick WHERE (local_symbol=='{}') AND (datetime>='{}') AND (datetime<='{}')", code, start, end);
    let data = client.get_handle()
        .and_then(move |c| c.query(sql
        ).fetch_all())
        .and_then(move |(_, block)| {
            Ok(Tick::new_with_block(block))
        });
    run(data).unwrap()
}


#[cfg(test)]
mod test {
    use super::clickhouse_rs::Pool;
    use crate::{fetch_tick, Client};

    #[test]
    fn test_fetch_tick() {
        let url = "tcp://10.0.0.34:9002/cy";
        let client = Client::new(url);
        let data = fetch_tick("rb2101.SHFE", "2020-11-05 09:00:00", "2020-11-05 11:00:00", &client);
        let length = data.len();
        assert_eq!(length, 12293usize)
    }
}