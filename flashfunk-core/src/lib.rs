extern crate alloc;

mod worker;

pub mod api;
pub mod builder;
pub mod strategy;
pub mod util;

#[cfg(test)]
mod test {
    use super::api::API;
    use super::strategy::{Strategy, StrategyCtx};
    use super::util::channel::{GroupReceiver, GroupSender};

    use std::sync::mpsc::{sync_channel, SyncSender};

    struct Rem;

    #[derive(Default)]
    struct RemContext;

    struct APIMessage(SyncSender<u32>);

    struct StrategyMessage(u32);

    impl API for Rem {
        type SndMessage = APIMessage;
        type RecvMessage = StrategyMessage;
        type Context = RemContext;

        fn run<const N: usize>(
            self,
            symbols: Vec<&str>,
            sender: GroupSender<Self::SndMessage, N>,
            receiver: GroupReceiver<Self::RecvMessage, N>,
        ) {
            assert_eq!(*symbols.first().unwrap(), "da_gong_ren");
            let (tx, rx) = sync_channel(1);

            sender.send_to(APIMessage(tx), 0);

            assert_eq!(996, rx.recv().unwrap());

            receiver.iter().for_each(|r| {
                if let Ok(m) = r.recv() {
                    assert_eq!(m.0, 251);
                }
            });
        }
    }

    struct RemStrategy {
        symbols: Vec<&'static str>,
    }

    impl Strategy<Rem> for RemStrategy {
        fn symbol(&self) -> &[&'static str] {
            self.symbols.as_slice()
        }

        fn call(
            &mut self,
            msg: <Rem as API>::SndMessage,
            ctx: &mut StrategyCtx<<Rem as API>::RecvMessage, <Rem as API>::Context>,
        ) {
            let tx = msg.0;

            ctx.sender().send(StrategyMessage(251));

            tx.send(996).unwrap();
        }
    }

    #[test]
    fn build() {
        let st = RemStrategy {
            symbols: vec!["da_gong_ren"],
        };
        let api = Rem;
        api.into_builder([st]).build();
    }
}
