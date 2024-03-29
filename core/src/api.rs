use super::builder::APIBuilder;
use super::strategy::Strategy;
use super::util::channel::{GroupReceiver, GroupSender};

pub trait API: Sized {
    /// message type from server to API and would be sent to strategies.
    type SndMessage: Send;

    /// message type from strategies to API and sent to server.
    type RecvMessage: Send;

    fn into_builder<S, const N: usize>(self, strategies: [S; N]) -> APIBuilder<Self, S, N>
    where
        S: Strategy<Self> + 'static,
        Self: 'static,
    {
        APIBuilder::new(self, strategies)
    }

    fn run<const N: usize>(
        self,
        sender: GroupSender<Self::SndMessage, N>,
        receiver: GroupReceiver<Self::RecvMessage, N>,
    );
}
