use core::sync::atomic::{AtomicBool, Ordering};
use core::time::Duration;

use std::borrow::Cow;
use std::io::prelude::*;
use std::rc::Rc;
use std::sync::Arc;
use std::thread::JoinHandle;

use crate::account::Account;
use crate::get_ticks;
use chrono::{NaiveDate, Utc};
use flashfunk_level::constant::{Direction, Status};
use flashfunk_level::data_type::{
    CancelRequest, LoginForm, OrderData, OrderRequest, Tick, TickData, TradeData,
};
use flashfunk_level::interface::Interface;
use flashfunk_level::types::message::{MdApiMessage, TdApiMessage};
use flashfunk_level::util::channel::GroupSender;
use flashfunk_level::util::hash::HashMap;

const QUEUE_INIT: i32 = 888_888;

pub struct MockMdApi {
    symbols: Vec<&'static str>,
    sender: Option<GroupSender<MdApiMessage>>,
    shutdown: Arc<AtomicBool>,
    handle: Option<JoinHandle<()>>,
}

impl Drop for MockMdApi {
    fn drop(&mut self) {
        self.shutdown.store(true, Ordering::SeqCst);
        if let Some(handle) = self.handle.take() {
            handle.join().unwrap();
        }
    }
}

impl Interface for MockMdApi {
    type Message = MdApiMessage;

    fn new(
        id: impl Into<Vec<u8>>,
        pwd: impl Into<Vec<u8>>,
        symbols: Vec<&'static str>,
        req: &LoginForm,
        sender: GroupSender<Self::Message>,
    ) -> Self {
        MockMdApi {
            symbols,
            sender: Option::from(sender),
            shutdown: Arc::new(AtomicBool::new(false)),
            handle: None,
        }
    }

    fn connect(&mut self) {
        self.subscribe();
    }

    fn subscribe(&mut self) {
        // read config
        let mut config_file = std::fs::File::open(
            std::env::var("CONFIG_FILE").expect("please input env:CONFIG_FILE"),
        )
        .expect("can not open config file");
        let mut content = String::new();
        config_file
            .read_to_string(&mut content)
            .expect("can not read config");
        let config_line = content
            .lines()
            .nth(1)
            .expect("can not get second line (config data line)");
        let mut iter = config_line.split(",");
        let symbol = iter.next().expect("can not find symbol");
        let start = iter.next().expect("can not find start");
        let end = iter.next().expect("can not find end");

        //let mut ticks: Vec<Tick> = fetch_tick("ni2102.SHFE", "2020-11-04 09:00:00", "2020-11-04 10:00:00").unwrap();
        let mut ticks: Vec<Tick> = get_ticks(symbol, start, end).unwrap();
        println!("{} tick num: {}", symbol, ticks.len());
        let sender = self.sender.take().unwrap();
        let shutdown = self.shutdown.clone();
        let handle = std::thread::spawn(move || {
            let rt = tokio::runtime::Builder::new_current_thread()
                .enable_all()
                .build()
                .unwrap();

            tokio::task::LocalSet::new().block_on(&rt, async move {
                let sender = Rc::new(sender);

                // 每个单独的回调任务都可以参照这个task.
                let a = tokio::task::spawn_local({
                    let sender = sender.clone();
                    async move {
                        loop {
                            if shutdown.load(Ordering::SeqCst) {
                                return;
                            }

                            // 每500ms发送一个tick data;
                            tokio::time::sleep(Duration::from_millis(500)).await;

                            // 在这里修改tick data数据;
                            let tick = TickData::from(&ticks.remove(0));
                            let msg = Arc::new(tick);
                            //let msg: &'static TickData = Box::leak(Box::new(TickData::default()));

                            sender.send_all(msg);
                        }
                    }
                });

                a.await.unwrap();
            });
        });

        self.handle = Some(handle);
    }
}

pub struct MockTdApi {
    acc: Account,
    date: NaiveDate,
    sender: GroupSender<TdApiMessage>,
    current_tick: TickData,
    old_tick: TickData,
    active_order_map: HashMap<String, OrderData>,
    queue_num_map: HashMap<String, i32>,
    req_id: i32,
    trade_id: i32,
}

impl MockTdApi {
    fn change_req_to_data(&self, idx: usize, req_id: i32, order_req: OrderRequest) -> OrderData {
        OrderData {
            symbol: order_req.symbol.clone(),
            exchange: order_req.exchange,
            datetime: self.current_tick.datetime,
            orderid: format!("{}_{}", idx, req_id),
            order_type: order_req.order_type,
            direction: order_req.direction,
            offset: order_req.offset,
            price: order_req.price,
            volume: order_req.volume,
            traded: 0.0,
            status: Status::NOTTRADED,
            is_local: true,
        }
    }

    // 估计盘口成交分布 -> (vol_on_ask,vol_on_bid)
    fn calc_volume(&self, size: f64) -> (f64, f64) {
        let current_volume = self.current_tick.volume - self.old_tick.volume;
        let current_turnover = self.current_tick.amount - self.old_tick.amount;
        let old_ask = self.old_tick.ask_price(0);
        let old_bid = self.old_tick.bid_price(0);

        if current_volume > 0 {
            let avg_price = current_turnover / current_volume as f64 / size;
            let ratio = (avg_price - old_bid) / (old_ask - old_bid);
            let ratio = ratio.max(0.0);
            let ratio = ratio.min(1.0);
            let vol_on_ask = ratio * current_volume as f64;
            let vol_on_bid = current_volume as f64 - vol_on_ask;
            return (vol_on_ask, vol_on_bid);
        } else {
            return (0.0, 0.0);
        }
    }

    // 获取订单价格对应的订单簿挂单量
    fn get_vol_form_orderbook(&self, order: &OrderData, tick: &TickData) -> i32 {
        if order.direction == Direction::LONG {
            if order.price == tick.bid_price(0) {
                return tick.bid_volume(0) as i32;
            } else if order.price == tick.bid_price(1) {
                return tick.bid_volume(1) as i32;
            } else if order.price == tick.bid_price(2) {
                return tick.bid_volume(2) as i32;
            } else if order.price == tick.bid_price(3) {
                return tick.bid_volume(3) as i32;
            } else if order.price == tick.bid_price(4) {
                return tick.bid_volume(4) as i32;
            } else {
                return QUEUE_INIT;
            }
        } else {
            if order.price == tick.ask_price(0) {
                return tick.ask_volume(0) as i32;
            } else if order.price == tick.ask_price(1) {
                return tick.ask_volume(1) as i32;
            } else if order.price == tick.ask_price(2) {
                return tick.ask_volume(2) as i32;
            } else if order.price == tick.ask_price(3) {
                return tick.ask_volume(3) as i32;
            } else if order.price == tick.ask_price(4) {
                return tick.ask_volume(4) as i32;
            } else {
                return QUEUE_INIT;
            }
        }
    }

    // 撮合逻辑（一档排在前面的都不撤单）：
    fn match_order(&mut self) {
        for (id, order) in self.active_order_map.clone() {
            let order_price = order.price;
            let order_dir = order.direction;
            let order_vol: i32 = order.volume as i32;
            let mut trade_price = 0.0;
            let mut trade_vol = 0;

            let queue_num = self.queue_num_map.get(&id).unwrap().clone();
            let mut new_queue_num_head = 0;

            let current_tick: &TickData = &self.current_tick;
            let old_tick: &TickData = &self.old_tick;

            if order.symbol != current_tick.symbol {
                continue;
            }
            // 没有出现在订单簿上（这时已经是下一个tick），认为已经成交
            if (order_dir == Direction::LONG) && (order_price >= self.current_tick.bid_price(0)) {
                trade_price = order_price.min(self.current_tick.ask_price(0));
                trade_vol = order_vol;
            } else if (order_dir == Direction::SHORT)
                && (order_price <= self.current_tick.ask_price(0))
            {
                trade_price = order_price.max(self.current_tick.bid_price(0));
                trade_vol = order_vol;
            } else {
                if queue_num > 0 {
                    // 获取该订单价格在订单簿上的挂单量，考虑自身长度
                    new_queue_num_head =
                        self.get_vol_form_orderbook(&order, current_tick) - order_vol;
                    new_queue_num_head = new_queue_num_head.min(queue_num);
                    let size = self.acc.get_size_map(current_tick.symbol.as_ref());
                    assert_ne!(size, 0.0, "size can not be 0.0!");
                    let ab_tuple: (f64, f64) = self.calc_volume(size);
                    // 连续在一档，才可能将位置更新到：当前位置 - 期间一档成交
                    if order_dir == Direction::LONG
                        && current_tick.bid_price(0) == order_price
                        && old_tick.bid_price(0) == order_price
                    {
                        new_queue_num_head = new_queue_num_head.min(queue_num - ab_tuple.1 as i32);
                    }
                    if order_dir == Direction::SHORT
                        && current_tick.ask_price(0) == order_price
                        && old_tick.ask_price(0) == order_price
                    {
                        new_queue_num_head = new_queue_num_head.min(queue_num - ab_tuple.0 as i32);
                    }
                    if new_queue_num_head > 0 {
                        self.queue_num_map.insert(id.clone(), new_queue_num_head);
                        continue;
                    } else {
                        // 计算全成还是部成
                        trade_vol = 1 - new_queue_num_head;
                        trade_price = order_price;
                    }
                } else {
                    // 不应该出现这种情况
                    println!("请检查排队撮合逻辑");
                    trade_vol = order_vol;
                    trade_price = order_price;
                }
            }
            self.trade_id += 1;
            let trade_data = TradeData {
                symbol: order.symbol,
                exchange: order.exchange,
                datetime: current_tick.datetime,
                orderid: id.clone(),
                direction: order.direction,
                offset: order.offset,
                price: trade_price,
                volume: trade_vol as i32,
                tradeid: self.trade_id.to_string(),
                is_local: true,
            };
            // 处理 order
            let mut order_data = self
                .active_order_map
                .remove(&id)
                .expect("can not find the order");

            if trade_vol == order_vol {
                order_data.status = Status::ALLTRADED;
                order_data.traded = trade_vol.into();
                self.queue_num_map.remove(&id);
            } else {
                order_data.status = Status::PARTTRADED;
                order_data.traded = trade_vol.into();
                order_data.volume = (order_vol - trade_vol) as f64;
                self.queue_num_map.insert(id.clone(), 1);
                self.active_order_map.insert(id.clone(), order_data.clone());
            }
            let temp_vec: Vec<&str> = id.split("_").collect();
            let idx = temp_vec[0].to_string().parse::<usize>().unwrap();

            self.acc.update_order(order_data.clone());
            self.sender.try_send_to(order_data, idx).unwrap_or(());

            // 处理 trade
            self.acc.update_trade(trade_data.clone());
            self.sender.try_send_to(trade_data, idx).unwrap_or(());
        }
    }

    // 驱动Td运行
    fn check(&mut self) {
        self.match_order();
    }
}

/// todo: we need to build a Local TdApi with Account\
/// It should provide a settle check
impl Interface for MockTdApi {
    type Message = TdApiMessage;

    fn new(
        _id: impl Into<Vec<u8>>,
        _pwd: impl Into<Vec<u8>>,
        _symbols: Vec<&'static str>,
        _req: &LoginForm,
        sender: GroupSender<Self::Message>,
    ) -> Self {
        MockTdApi {
            acc: Account::new(),
            // fixme: 需要将这个更换为回测过程中的交易日， 每日阶段后进行切换
            date: Utc::today().naive_local(),
            sender: sender,
            current_tick: Default::default(),
            old_tick: Default::default(),
            active_order_map: Default::default(),
            queue_num_map: Default::default(),
            req_id: 0,
            trade_id: 0,
        }
    }

    /// 发单
    fn send_order(&mut self, idx: usize, order: OrderRequest) {
        self.req_id += 1;
        // 判断可用保证金是否充足
        let need_forzen = order.volume
            * order.price
            * self.acc.get_size_map(order.symbol.as_str())
            * self.acc.get_commission_ratio(order.symbol.as_str());
        let ava = self.acc.available();
        if ava >= need_forzen {
            // 下单成功
            let order_data = self.change_req_to_data(idx, self.req_id, order);
            // 冻结账户保证金
            self.acc.update_order(order_data.clone());
            self.active_order_map
                .insert(order_data.orderid.clone(), order_data.clone());
            self.queue_num_map
                .insert(order_data.orderid.clone(), QUEUE_INIT);
            self.sender.try_send_to(order_data, idx).unwrap_or(());
        } else {
            // 拒单
            let mut order_data = self.change_req_to_data(idx, self.req_id, order);
            order_data.status = Status::REJECTED;
            self.sender.try_send_to(order_data, idx).unwrap_or(());
        }
    }

    /// 撤单
    fn cancel_order(&mut self, req: CancelRequest) {
        self.req_id += 1;
        if self.active_order_map.contains_key(&req.order_id) {
            let mut order_data = self.active_order_map.remove(&req.order_id).unwrap();
            self.queue_num_map.remove(&req.order_id);
            let temp_vec: Vec<&str> = req.order_id.split("_").collect();
            let idx: usize = temp_vec[0].to_string().parse().unwrap();
            order_data.status = Status::CANCELLED;
            // 解冻账户保证金
            self.acc.update_order(order_data.clone());
            self.sender.try_send_to(order_data, idx).unwrap_or(());
        } else {
            println!("撤单失败");
        }
    }

    /// 登录接口
    fn connect(&mut self) {}

    /// 订阅行情
    fn subscribe(&mut self) {
        unimplemented!()
    }

    /// 取消订阅行情
    fn unsubscribe(&mut self, symbol: String) {
        unimplemented!()
    }

    /// 释放退出接口
    fn exit(&mut self) {
        unimplemented!()
    }

    /// 更新行情
    fn update_quote(&mut self, quote: &TickData) {
        self.old_tick = self.current_tick.clone();
        self.current_tick = quote.clone();
        self.check();
    }
}
