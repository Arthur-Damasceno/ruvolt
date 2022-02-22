use tokio::sync::{
    mpsc::{unbounded_channel, UnboundedReceiver, UnboundedSender},
    oneshot,
};

use crate::{models::events::ClientToServerEvent, Result};

type OneshotTx<T = ()> = oneshot::Sender<Result<T>>;

#[derive(Debug)]
pub enum Action {
    SendEvent {
        event: ClientToServerEvent,
        tx: OneshotTx,
    },
}

pub type ActionRx = UnboundedReceiver<Action>;

#[derive(Debug, Clone)]
pub struct ActionMessenger(UnboundedSender<Action>);

impl ActionMessenger {
    pub fn new() -> (Self, ActionRx) {
        let (tx, rx) = unbounded_channel();

        (Self(tx), rx)
    }

    pub async fn send(&self, event: ClientToServerEvent) -> Result {
        let (tx, rx) = oneshot::channel();

        self.0.send(Action::SendEvent { event, tx }).unwrap();

        rx.await.unwrap()
    }
}
