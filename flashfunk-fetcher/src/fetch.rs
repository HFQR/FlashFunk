use std::any::Any;
use std::error::Error;
use std::sync::mpsc::{sync_channel, Receiver, SyncSender};

use async_trait::async_trait;
use clickhouse_rs::Pool;
use futures::channel::mpsc::{unbounded, UnboundedSender};
use futures::StreamExt;
use once_cell::sync::Lazy;

use crate::model::Tick;

// query类型，储存所有的sql params
pub struct TickQuery {
    code: String,
    start: String,
    end: String,
}

// 数据库接受的sql格式
type SqlFormat = String;

// 定义sql语句和逻辑的trait
#[async_trait]
pub trait SqlQuery: Send + Sync {
    type SqlResult: Send + 'static;

    fn sql(&self) -> SqlFormat;

    async fn query(&mut self, pool: &Pool) -> Self::SqlResult;
}

#[async_trait]
impl SqlQuery for TickQuery {
    type SqlResult = Result<Vec<Tick>, Box<dyn Error + Send + Sync + 'static>>;

    fn sql(&self) -> SqlFormat {
        let mut sql = SqlFormat::new();

        // 常见sql字段应打包作为const
        sql.push_str("SELECT * ");
        sql.push_str("FROM tick ");
        sql.push_str("WHERE ");

        // 分段format。sql的性能开销重要性低于可读性和正确性。
        sql.push_str(&format!("(local_symbol=='{}')", self.code));

        sql.push_str(" AND ");

        sql.push_str(&format!("(datetime>='{}')", self.start));

        sql.push_str(" AND ");

        sql.push_str(&format!("(datetime<='{}')", self.end));

        // 可省略
        let base = format!("SELECT * FROM tick WHERE (local_symbol=='{}') AND (datetime>='{}') AND (datetime<='{}')", self.code, self.start, self.end);
        assert_eq!(base, sql);

        sql
    }

    // 暂时使用trait object错误类型，应当使用固定类型错误
    async fn query(
        &mut self,
        pool: &Pool,
    ) -> Result<Vec<Tick>, Box<dyn Error + Send + Sync + 'static>> {
        let sql = self.sql();

        let ticks = pool
            .get_handle()
            .await?
            .query(sql)
            .fetch_all()
            .await?
            .rows()
            .into_iter()
            .map(Tick::new)
            .collect::<Vec<_>>();

        Ok(ticks)
    }
}

// 中间trait，用来消除类型
#[async_trait]
trait SqlQueryMessageHandler: Send + Sync {
    // SyncSender使用any类型返回结果，用来解放类型绑定。
    async fn handle(&mut self, pool: &Pool, sender: SyncSender<Box<dyn Any + Send>>);
}

// impl 中间trait. 使类型不在关联<TickQuery as SqlQuery>::SqlResult
#[async_trait]
impl<Q: SqlQuery> SqlQueryMessageHandler for Q {
    async fn handle(&mut self, pool: &Pool, sender: SyncSender<Box<dyn Any + Send>>) {
        let res = self.query(pool).await;
        // 在转换为any类型前用Option包裹结果，方便后面取出所有权。
        let _ = sender.send(Box::new(Some(res)));
    }
}

// 此类型存放box的中间trait和发送sql结果的通道发送端
struct SqlQueryMessage {
    query: Box<dyn SqlQueryMessageHandler>,
    tx: SyncSender<Box<dyn Any + Send>>,
}

impl SqlQueryMessage {
    // 构造 回程通道和消息
    fn new_with_recv(
        query: impl SqlQueryMessageHandler + 'static,
    ) -> (Self, Receiver<Box<dyn Any + Send>>) {
        let (tx, rx) = sync_channel(1);

        let this = Self {
            query: Box::new(query),
            tx,
        };

        (this, rx)
    }
}

// 静态tokio handle。第一次调用的时候会惰性生成 clickhouse客户端和tokio运行时。并返回通道入口。
static HANDLE: Lazy<UnboundedSender<SqlQueryMessage>> = Lazy::new(|| {
    let url = std::env::var("CLICK_HOUSE_URI").unwrap_or("tcp://127.0.0.1:9001/tick".parse().unwrap());
    let pool = Pool::new(url);

    let (tx, mut rx) = unbounded::<SqlQueryMessage>();

    std::thread::spawn(move || {
        tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .unwrap()
            .block_on(async move {
                while let Some(msg) = rx.next().await {
                    let mut handle = msg.query;
                    let sender = msg.tx;
                    handle.handle(&pool, sender).await;
                }
            })
    });

    tx
});

pub fn fetch_tick(
    code: impl Into<String>,
    start: impl Into<String>,
    end: impl Into<String>,
    // 这里使用TickQuery的关联类型，避免后面的downcast类型错误
) -> <TickQuery as SqlQuery>::SqlResult {
    // 构造 query
    let query = TickQuery {
        code: code.into(),
        start: start.into(),
        end: end.into(),
    };

    // 打包消息
    let (msg, rx) = SqlQueryMessage::new_with_recv(query);

    // 发送消息， 这里应该处理错误
    HANDLE.unbounded_send(msg)?;

    // downcast类型是运行时错误，用<TickQuery as SqlQuery>::SqlResult的关联类型确保一致性
    rx.recv()?
        // downcast为Option的原因在 SqlQueryMessageHandler::handle 方法中
        .downcast_mut::<Option<<TickQuery as SqlQuery>::SqlResult>>()
        .unwrap()
        .take()
        .unwrap()
}

#[cfg(test)]
mod test {
    use crate::fetch_tick;

    #[test]
    fn test_fetch_tick() {
        let ticks =
            fetch_tick("rb2101.SHFE", "2020-11-05 09:00:00", "2020-11-05 11:00:00").unwrap();
    }
}
