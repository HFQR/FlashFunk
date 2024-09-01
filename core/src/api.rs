use core::{
    cmp::Eq,
    hash::{Hash, Hasher},
};

use super::builder::APIBuilder;
use super::strategy::Strategy;
use super::util::channel::{GroupReceiver, GroupSender};

pub trait API: Sized {
    /// symbol identifier for Strategy used for sending specific message to certain strategy.
    type Symbol: Hash + Eq + Clone;

    /// hasher used for storing strategy context with HashMap.
    ///
    /// flashfunk offers two utility hasher as: [`FxHasher`] and [`NoHasher`].
    /// it's also possible to bring your own hasher by implement [Hasher] and [Default] traits for it.
    ///
    /// [`FxHasher`]: crate::util::fx_hasher::FxHasher
    /// [`NoHasher`]: crate::util::no_hasher::NoHasher
    type Hasher: Hasher + Default;

    /// message type from server to API and would be sent to strategies.
    type SndMessage: Send;

    /// message type from strategies to API and sent to server.
    type RecvMessage: Send;

    /// builder type where an array of strategies will be prepared to be hooked into api type.
    /// see [APIBuilder] for more.
    fn into_builder<S, const N: usize>(self, strategies: [S; N]) -> APIBuilder<Self, S, N>
    where
        S: Strategy<Self> + 'static,
        Self: 'static,
    {
        APIBuilder::new(self, strategies)
    }

    /// callback function handles running of the api business logic.
    /// sender is used to send message to strategy with [Self::Symbol] type as identifier.
    /// receiver is used to receive message from all strategy with iteration
    fn run<const N: usize>(
        self,
        sender: GroupSender<Self::Symbol, Self::Hasher, Self::SndMessage, N>,
        receiver: GroupReceiver<Self::RecvMessage, N>,
    );
}
