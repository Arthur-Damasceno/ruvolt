use {super::super::ServerToClientEvent, crate::entities::Id};

/// A channel has been deleted.
#[derive(Debug)]
pub struct ChannelDeleteEvent {
    /// Channel id.
    pub id: Id,
}

impl From<ServerToClientEvent> for ChannelDeleteEvent {
    fn from(event: ServerToClientEvent) -> Self {
        if let ServerToClientEvent::ChannelDelete { id } = event {
            Self { id }
        } else {
            panic!("An incorrect event was provided: {:?}", event);
        }
    }
}
