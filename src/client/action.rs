use tokio::sync::mpsc::{unbounded_channel, UnboundedReceiver, UnboundedSender};

#[derive(Debug)]
pub enum Action {}

pub type ActionRx = UnboundedReceiver<Action>;

#[derive(Debug, Clone)]
pub struct ActionMessenger(UnboundedSender<Action>);

impl ActionMessenger {
    pub fn new() -> (Self, ActionRx) {
        let (tx, rx) = unbounded_channel();

        (Self(tx), rx)
    }
}
