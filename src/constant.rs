// 报单
pub struct Order {
    order_id: String,
    // 单号
    direction: String,
    // 价格趋势  多/空
    offset: String,
    // 开平方向
    local_symbol: String,
    // 本地代码
    price: f64,
    // 下单价格
    volume: f64,
    // 下单手数
    status: String, //单子状态
}

// 交易单
pub struct Trade {}

// 日志
pub struct Log {
    level: i32,
    msg: String,
    owner: String,
    time: String,
}