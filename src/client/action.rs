use tokio::sync::{
    mpsc::{unbounded_channel, UnboundedReceiver, UnboundedSender},
    oneshot,
};

use crate::{models::events::ClientEvent, Result};

type OneshotTx<T = ()> = oneshot::Sender<Result<T>>;

#[derive(Debug)]
pub enum Action {
    SendEvent { event: ClientEvent, tx: OneshotTx },
}

pub type ActionRx = UnboundedReceiver<Action>;

#[derive(Debug, Clone)]
pub struct ActionMessenger(UnboundedSender<Action>);

impl ActionMessenger {
    pub fn new() -> (Self, ActionRx) {
        let (tx, rx) = unbounded_channel();

        (Self(tx), rx)
    }

    pub async fn send(&self, event: ClientEvent) -> Result {
        let (tx, rx) = oneshot::channel();

        self.0.send(Action::SendEvent { event, tx }).unwrap();

        rx.await.unwrap()
    }
}
