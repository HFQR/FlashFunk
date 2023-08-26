extern crate alloc;
extern crate core;

mod worker;

pub mod api;
pub mod builder;
pub mod strategy;
pub mod util;

#[cfg(test)]
mod test {
    use super::api::API;
    use super::strategy::{Context, Strategy};
    use super::util::channel::{BroadcastSender, GroupReceiver};

    use alloc::vec::Vec;

    use std::sync::mpsc;

    struct Rem;

    #[derive(Default)]
    struct RemContext;

    struct APIMessage(mpsc::Sender<u32>);

    struct StrategyMessage(u32);

    impl API for Rem {
        type SndMessage = APIMessage;
        type RecvMessage = StrategyMessage;

        fn run<const N: usize>(
            self,
            mut sender: BroadcastSender<Self::SndMessage>,
            mut receiver: GroupReceiver<Self::RecvMessage, N>,
        ) {
            let (tx, rx) = mpsc::channel();

            sender.send(APIMessage(tx));

            #[cfg(not(feature = "async"))]
            {
                loop {
                    if let Ok(item) = rx.recv() {
                        assert_eq!(996, item);
                        break;
                    }
                }

                receiver.iter_mut().for_each(|r| {
                    if let Ok(m) = r.recv() {
                        assert_eq!(m.0, 251);
                    }
                });
            }

            #[cfg(feature = "async")]
            {
                crate::util::async_runtime::StdRuntime::new().block_on(async move {
                    assert_eq!(996, rx.recv().await.unwrap());

                    for r in receiver.iter_mut() {
                        if let Ok(m) = r.recv().await {
                            assert_eq!(m.0, 251);
                        }
                    }
                });
            }
        }
    }

    struct RemStrategy {
        symbols: Vec<&'static str>,
    }

    impl Strategy<Rem> for RemStrategy {
        fn symbol(&self) -> &[&'static str] {
            self.symbols.as_slice()
        }

        fn call(&mut self, msg: &<Rem as API>::SndMessage, ctx: &mut Context<Rem>) {
            ctx.sender().send(StrategyMessage(251));
            msg.0.send(996u32).unwrap();
        }
    }

    #[test]
    fn build() {
        let st = RemStrategy {
            symbols: vec!["dgr123"],
        };
        let api = Rem;
        api.into_builder([st])
            .disable_pin_to_core()
            .message_capacity(128)
            .build();
    }
}
