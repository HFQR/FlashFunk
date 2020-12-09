use crate::structs::{AccountData, CancelRequest, ContractData, OrderData, OrderRequest, PositionData, QueryRequest, SubscribeRequest, TickData, TradeData, ExtraOrder, ExtraTrade};


pub enum MdApiMessage {
    TickData(&'static TickData),
    SubscribeRequest(SubscribeRequest),
}

impl From<SubscribeRequest> for MdApiMessage {
    fn from(data: SubscribeRequest) -> Self {
        Self::SubscribeRequest(data)
    }
}

impl From<&'static TickData> for MdApiMessage {
    fn from(data: &'static TickData) -> Self {
        Self::TickData(data)
    }
}

pub enum TdApiMessage {
    OrderData(OrderData),
    TradeData(TradeData),
    AccountData(AccountData),
    PositionData(PositionData),
    ContractData(ContractData),
    ExtraOrder(ExtraOrder),
    ExtraTrade(ExtraTrade),
}


impl From<OrderData> for TdApiMessage {
    fn from(data: OrderData) -> Self {
        Self::OrderData(data)
    }
}

impl From<ExtraOrder> for TdApiMessage {
    fn from(data: ExtraOrder) -> Self {
        Self::ExtraOrder(data)
    }
}

impl From<ExtraTrade> for TdApiMessage {
    fn from(data: ExtraTrade) -> Self {
        Self::ExtraTrade(data)
    }
}

impl From<TradeData> for TdApiMessage {
    fn from(data: TradeData) -> Self {
        Self::TradeData(data)
    }
}

impl From<AccountData> for TdApiMessage {
    fn from(data: AccountData) -> Self {
        Self::AccountData(data)
    }
}

impl From<PositionData> for TdApiMessage {
    fn from(data: PositionData) -> Self {
        Self::PositionData(data)
    }
}

impl From<ContractData> for TdApiMessage {
    fn from(data: ContractData) -> Self {
        Self::ContractData(data)
    }
}


pub enum StrategyMessage {
    OrderRequest(OrderRequest),
    CancelRequest(CancelRequest),
    QueryReq(QueryRequest),
    MockTdTickData(TickData),
}

impl From<OrderRequest> for StrategyMessage {
    fn from(data: OrderRequest) -> Self {
        Self::OrderRequest(data)
    }
}

impl From<CancelRequest> for StrategyMessage {
    fn from(data: CancelRequest) -> Self {
        Self::CancelRequest(data)
    }
}

impl From<QueryRequest> for StrategyMessage {
    fn from(data: QueryRequest) -> Self {
        Self::QueryReq(data)
    }
}

impl From<TickData> for StrategyMessage {
    fn from(data: TickData) -> Self {
        Self::MockTdTickData(data)
    }
}
