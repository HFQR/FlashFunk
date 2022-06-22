//! An example async API for driving strategy with websocket message.

use std::io;

use flashfunk_core::{
    api::API,
    strategy::{Context, Strategy},
    util::channel::{GroupReceiver, GroupSender},
};
use futures_util::{SinkExt, StreamExt};
use xitca_client::{bytes::Bytes, error::Error, http::Version, ws::Message, Client};

struct WsAPI;

struct StrategyMessage(String);

impl API for WsAPI {
    type SndMessage = Bytes;
    type RecvMessage = StrategyMessage;

    fn run<const N: usize>(
        self,
        mut sender: GroupSender<Self::SndMessage, N>,
        mut receiver: GroupReceiver<Self::RecvMessage, N>,
    ) {
        let res = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .unwrap()
            .block_on(async move {
                let client = Client::builder()
                    .rustls()
                    .set_max_http_version(Version::HTTP_11)
                    .set_pool_capacity(8)
                    .finish();

                let mut ws = client.ws("wss://ws.kraken.com/")?.send().await?.ws()?;

                let msg = ws.next().await.unwrap()?;
                println!("Connected: {:?}", msg);

                ws.send(Message::Text(Bytes::from("{\"event\":\"subscribe\", \"subscription\":{\"name\":\"ticker\"}, \"pair\":[\"BTC/USD\"]}"))).await?;

                loop {
                    tokio::select! {
                        res = ws.next() => {
                            let msg = res.ok_or(io::Error::from(io::ErrorKind::UnexpectedEof))??;
                            match msg {
                                Message::Text(bytes) | Message::Binary(bytes) => sender.send_all(bytes),
                                Message::Ping(bytes) => ws.send(Message::Pong(bytes)).await?,
                                Message::Close(reason) => {
                                    ws.send(Message::Close(reason)).await?;
                                    return Ok::<_, Error>(())
                                },
                                _ => {}
                            }
                        }
                        res = receiver.recv() => {
                            let msg = res.unwrap();
                            println!("Message from WsStrategy: {}", msg.0);
                        }
                    }
                }
            });

        if let Err(e) = res {
            println!("API exit with error: {:?}", e);
        }
    }
}

struct WsStrategy {
    symbols: [&'static str; 1],
}

impl Strategy<WsAPI> for WsStrategy {
    fn symbol(&self) -> &[&'static str] {
        self.symbols.as_slice()
    }

    fn call(&mut self, msg: Bytes, ctx: &mut Context<WsAPI>) {
        println!("Message from WsAPI: {}\r\n", String::from_utf8_lossy(&msg));
        ctx.sender().send(StrategyMessage(
            self.symbol()
                .first()
                .map(|s| s.to_string())
                .unwrap_or_else(String::new),
        ))
    }
}

fn main() {
    let st = WsStrategy {
        symbols: ["da_gong_ren"],
    };

    WsAPI
        .into_builder([st])
        .disable_pin_to_core()
        .message_capacity(128)
        .build();
}
