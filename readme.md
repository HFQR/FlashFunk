## ctpbee-rs 

> 专为高频交易设计的程序化交易框架


### 功能模块

- `data_support` 

数据支持器, 借助数据库，提供快速的历史数据提取

- `position`

根据`ctpbee`里面的仓位管理器，快速更新`ctpbee-rs`里面的仓位数据

- `interface` 
实现一个`Interface`的`Trait`，使得能够实现标准的报单撤单以及查询持仓和查询账户功能
