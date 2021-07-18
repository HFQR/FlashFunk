use super::builder::APIBuilder;
use super::strategy::Strategy;
use super::util::channel::{GroupReceiver, GroupSender};

pub trait API: Sized {
    /// message type from server to API and would be sent to strategies.
    type SndMessage: Send;

    /// message type from strategies to API and sent to server.
    type RecvMessage: Send;

    /// context type used by strategy. serves as state storage for strategy that
    /// can be reference separately from strategy's state.
    ///
    /// e.g: reference &mut context and &mut strategy at the same time.
    type Context: Default;

    fn into_builder<S: Strategy<Self>, const N: usize>(
        self,
        strategies: [S; N],
    ) -> APIBuilder<Self, S, N> {
        APIBuilder {
            api: self,
            strategies,
        }
    }

    fn run<const N: usize>(
        self,
        symbols: Vec<&str>,
        sender: GroupSender<Self::SndMessage>,
        receiver: GroupReceiver<Self::RecvMessage, N>,
    );
}
