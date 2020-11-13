# FlashFunk-fetcher


## description
this project are used to provide tick data  in backtest model.

Just rewrite the MockMdApi


## sample usage

```rust
use flashfunk_fetcher::fetch_tick;


fn main(){
    let url = "tcp://10.0.0.34:9002/cy";
    let pool = Pool::new(url);
    let data = fetch_tick("rb2101.SHFE", "2020-11-05 09:00:00", "2020-11-05 11:00:00", &pool);
    for tick in data{
        println!("{:?}", tick);
    }   
}
```