use core::sync::atomic::{AtomicBool, Ordering};
use core::time::Duration;

use std::rc::Rc;
use std::sync::Arc;
use std::thread::JoinHandle;

use crate::interface::Interface;
use crate::structs::{LoginForm, TickData, CancelRequest, OrderRequest, OrderData, TradeData};
use crate::types::message::{MdApiMessage, TdApiMessage};
use crate::util::channel::{GroupSender};
use crate::account::Account;
use chrono::{NaiveDate, Utc};
use crate::util::hash::HashMap;
use crate::constants::{Status, Direction};
use std::borrow::Cow;
use flashfunk_fetcher::{Tick, fetch_tick};

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

    fn connect(&mut self) {}

    fn subscribe(&mut self) {
        let mut ticks: Vec<Tick> = fetch_tick("rb2101.SHFE", "2020-11-05 09:00:00", "2020-11-05 11:00:00").unwrap();
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
                            let msg: &'static TickData = Box::leak(Box::new(tick));
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
    backtest_mode: u8,
}

impl MockTdApi {
    fn change_req_to_data(& self, idx: usize, req_id:i32, order_req: OrderRequest) -> OrderData {
        OrderData {
            symbol: order_req.symbol.clone(),
            exchange: order_req.exchange,
            datetime: self.current_tick.datetime,
            orderid: Some(format!("{}_{}", idx, req_id)),
            order_type: order_req.order_type,
            direction: Some(order_req.direction),
            offset: order_req.offset,
            price: order_req.price,
            volume: order_req.volume,
            traded: 0.0,
            status: Status::NOTTRADED,
        }
    }

    // 估计盘口成交分布 -> (vol_on_ask,vol_on_bid)
    fn calc_volume(&self, size:f64) -> (f64, f64){
        let current_volume = self.current_tick.volume - self.old_tick.volume;
        let current_turnOver = self.current_tick.amount - self.old_tick.amount;
        let old_ask = self.old_tick.ask_price(0);
        let old_bid = self.old_tick.bid_price(0);

        if current_volume > 0{
            let avg_price = current_turnOver / current_volume as f64 / size;
            let ratio = (avg_price - old_bid) / (old_ask - old_bid);
            let ratio = ratio.max(0.0);
            let ratio = ratio.min(1.0);
            let vol_on_ask = ratio * current_volume as f64;
            let vol_on_bid = current_volume as f64 - vol_on_ask;
            return (vol_on_ask, vol_on_bid);
        } 
        else{
            return (0.0, 0.0);
        }
    }

    // 获取订单价格对应的订单簿挂单量
    fn get_vol_form_orderbook(&self, order: &OrderData, tick: &TickData) -> i32 {
        if order.direction.unwrap() == Direction::LONG{
            if order.price == tick.bid_price(0){
                return tick.bid_volume(0) as i32;
            }
            else if order.price == tick.bid_price(1){
                return tick.bid_volume(1) as i32;
            }
            else if order.price == tick.bid_price(2){
                return tick.bid_volume(2) as i32;
            }
            else if order.price == tick.bid_price(3){
                return tick.bid_volume(3) as i32;
            }
            else if order.price == tick.bid_price(4){
                return tick.bid_volume(4) as i32;
            }
            else{
                return QUEUE_INIT;
            }
        }
        else {
            if order.price == tick.ask_price(0){
                return tick.ask_volume(0) as i32;
            }
            else if order.price == tick.ask_price(1){
                return tick.ask_volume(1) as i32;
            }
            else if order.price == tick.ask_price(2){
                return tick.ask_volume(2) as i32;
            }
            else if order.price == tick.ask_price(3){
                return tick.ask_volume(3) as i32;
            }
            else if order.price == tick.ask_price(4){
                return tick.ask_volume(4) as i32;
            }
            else {
                return QUEUE_INIT;
            }
        }
    }
        
    // 见价
    fn match_order_mode_0(&mut self) {
        for (id, order) in self.active_order_map.clone(){
            let mut trade_price: f64 = 0.0;
            if order.symbol != self.current_tick.symbol{
                continue;
            }
            if (order.direction.unwrap() == Direction::LONG) && (order.price >= self.current_tick.bid_price(0)){
                trade_price = order.price.min(self.current_tick.ask_price(0));
            }
            else if (order.direction.unwrap() == Direction::SHORT) && (order.price <= self.current_tick.ask_price(0)){
                trade_price = order.price.max(self.current_tick.bid_price(0));
            }
            else{
                continue;
            }
            self.trade_id += 1;
            // 生成成交数据
            let trade_data = TradeData{
                symbol: Cow::from(order.symbol.clone()),
                exchange: Some(order.exchange),
                datetime: self.current_tick.datetime,
                orderid: Option::from(id.clone()),
                direction: order.direction,
                offset: Some(order.offset),
                price: trade_price,
                volume: order.volume as i32,
                tradeid: Some(self.trade_id.to_string()),
            };
            // 处理 order
            let mut order_data = self.active_order_map.remove(&id).unwrap();
            self.queue_num_map.remove(&id);
            let temp_vec: Vec<&str> = id.split("_").collect();
            let idx = temp_vec[0].to_string().parse::<usize>().unwrap();
            order_data.status = Status::ALLTRADED;
            self.acc.update_order(order_data.clone());
            self.sender.try_send_to(order_data, idx).unwrap_or(());
            // 处理 trade
            self.acc.update_trade(trade_data.clone());
            self.sender.try_send_to(trade_data, idx).unwrap_or(());
        }
    }

    // 排队最差（排在前方订单不撤单，仅在一档分离成交量，仅支持全部成交）
    fn match_order_mode_1(&mut self) {
        for (id, order) in self.active_order_map.clone(){
            let order_price: f64 = order.price;
            let order_dir: Direction = order.direction.unwrap();
            let mut trade_price: f64 = 0.0;

            let queue_num: i32 = self.queue_num_map.get(&id).unwrap().clone();
            let mut new_queue_num: i32 = 0;
        
            let current_tick: &TickData = &self.current_tick;
            let old_tick: &TickData = &self.old_tick;

            if order.symbol != current_tick.symbol{
                continue;
            }
            if (order.direction.unwrap() == Direction::LONG) && (order.price >= self.current_tick.bid_price(0)){
                trade_price = order.price.min(self.current_tick.ask_price(0));
            }
            else if (order.direction.unwrap() == Direction::SHORT) && (order.price <= self.current_tick.ask_price(0)){
                trade_price = order.price.max(self.current_tick.bid_price(0));
            }
            else {
                // 排队位置向前变化只有两种情况：
                // 1.前面的订单成交（只可能存在于一档）
                // 2.撤单导致位置向前（目前挂单量比之前排队位置还要少的情况）
                if queue_num > 0{
                    // 获取该订单价格在订单簿上的挂单量
                    new_queue_num = self.get_vol_form_orderbook(&order, current_tick);
                    new_queue_num = new_queue_num.min(queue_num);
                    // 估算上一个时间段在买卖一档的成交量
                    // fixme 这里需要acc支持获取合约乘数
                    //let ab_tuple: (f64, f64) = self.calc_volume(self.acc.get_size_map(current_tick.symbol.as_ref()));
                    let ab_tuple: (f64, f64) = self.calc_volume(10.0);
                    // 只有在发生上述成交时，订单持续维持在一档，才认为位置前移
                    // 前移后的值和当前挂单量取最优
                    if order_dir == Direction::LONG 
                        && current_tick.bid_price(0) == order_price && old_tick.bid_price(0) == order_price{
                        new_queue_num = new_queue_num.min(queue_num - ab_tuple.1 as i32);
                    }
                    if order_dir == Direction::SHORT 
                        && current_tick.ask_price(0) == order_price && old_tick.ask_price(0) == order_price{
                        new_queue_num = new_queue_num.min(queue_num - ab_tuple.0 as i32);
                    }
                    if new_queue_num > 0{
                        // 更新排队位置
                        self.queue_num_map.insert(id, new_queue_num);
                        continue;
                    }
                    else{
                        // 成交
                        trade_price = order_price;
                    }
                }
                else{
                    // 不应该出现这种情况
                    println!("请检查排队撮合逻辑");
                    trade_price = order_price;
                }
            }
            self.trade_id += 1;
            // 生成成交数据
            let trade_data = TradeData{
                symbol: Cow::from(order.symbol),
                exchange: Some(order.exchange),
                datetime: current_tick.datetime,
                orderid: Option::from(id.clone()),
                direction: order.direction,
                offset: Some(order.offset),
                price: trade_price,
                volume: order.volume as i32,
                tradeid: Some(self.trade_id.to_string()),
            };
            // 处理 order
            let mut order_data = self.active_order_map.remove(&id).unwrap();
            self.queue_num_map.remove(&id);
            let temp_vec: Vec<&str> = id.split("_").collect();
            let idx = temp_vec[0].to_string().parse::<usize>().unwrap();
            order_data.status = Status::ALLTRADED;
            self.acc.update_order(order_data.clone());
            self.sender.try_send_to(order_data, idx).unwrap_or(());
            // 处理 trade
            self.acc.update_trade(trade_data.clone());
            self.sender.try_send_to(trade_data, idx).unwrap_or(());
        }
    }

    // todo 排队精细（尽可能分离成交量，支持部分成交）
    fn match_order_mode_2(&mut self) {
        unimplemented!();
    }
    
    // 驱动Td运行
    fn check(&mut self) {
        // 
        // 根据回测模式选择撮合逻辑
        match self.backtest_mode{
            0 => {
                self.match_order_mode_0();
            }
            1 => {
                self.match_order_mode_1();
            }
            2 => {
                self.match_order_mode_2();
            }
            _ => {
                self.match_order_mode_0();
            }
        }
    }
}

/// todo: we need to build a Local TdApi with Account\
/// It should provide a settle check
impl Interface for MockTdApi {
    type Message = TdApiMessage;

    fn new(
        id: impl Into<Vec<u8>>,
        pwd: impl Into<Vec<u8>>,
        symbols: Vec<&'static str>,
        req: &LoginForm,
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
            backtest_mode: 0,
        }
    }

    /// 发单
    fn send_order(&mut self, idx: usize, order: OrderRequest) {
        self.req_id += 1;
        // 判断可用保证金是否充足
        let need_forzen = order.volume * order.price * self.acc.get_size_map(order.symbol.as_str()) * self.acc.get_commission_ratio(order.symbol.as_str());
        let ava = self.acc.available();
        if ava > need_forzen {
            // 下单成功
            let order_data = self.change_req_to_data(idx, self.req_id, order);
            // 冻结账户保证金
            self.acc.update_order(order_data.clone());
            self.active_order_map.insert(order_data.orderid.clone().unwrap(), order_data.clone());
            self.queue_num_map.insert(order_data.orderid.clone().unwrap().clone(), QUEUE_INIT);
            self.sender.try_send_to(order_data, idx).unwrap_or(());
        }
        else{
            // 拒单
            let mut order_data = self.change_req_to_data(idx, self.req_id, order);
            order_data.status = Status::REJECTED;
            self.sender.try_send_to(order_data, idx).unwrap_or(());
        }
    }

    /// 撤单
    fn cancel_order(&mut self, req: CancelRequest) {
        self.req_id += 1;
        if self.active_order_map.contains_key(&req.order_id){
            let mut order_data = self.active_order_map.remove(&req.order_id).unwrap();
            self.queue_num_map.remove(&req.order_id);
            let temp_vec: Vec<&str> = req.order_id.split("_").collect();
            let idx = temp_vec[0].to_string().parse::<usize>().unwrap();
            order_data.status = Status::CANCELLED;
            // 解冻账户保证金
            self.acc.update_order(order_data.clone());
            self.sender.try_send_to(order_data, idx).unwrap_or(());
        }
        else{
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