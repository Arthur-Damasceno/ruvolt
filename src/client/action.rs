use {
    std::time::Duration,
    tokio::sync::{
        mpsc::{unbounded_channel, UnboundedReceiver, UnboundedSender},
        oneshot::{self, Sender as OneshotTx},
    },
};

use crate::{models::events::ClientEvent, Result};

#[derive(Debug)]
pub enum Action {
    SendEvent {
        event: ClientEvent,
        tx: OneshotTx<Result>,
    },
    GetLatency {
        tx: OneshotTx<Option<Duration>>,
    },
    Close {
        tx: OneshotTx<Result>,
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

    pub async fn send(&self, event: ClientEvent) -> Result {
        let (tx, rx) = oneshot::channel();

        self.0.send(Action::SendEvent { event, tx }).unwrap();

        rx.await.unwrap()
    }

    pub async fn latency(&self) -> Option<Duration> {
        let (tx, rx) = oneshot::channel();

        self.0.send(Action::GetLatency { tx }).unwrap();

        rx.await.unwrap()
    }

    pub async fn close(&self) -> Result {
        let (tx, rx) = oneshot::channel();

        self.0.send(Action::Close { tx }).unwrap();

        rx.await.unwrap()
    }
}
