use super::ServerToClientEvent;

/// A channel has been deleted.
#[derive(Debug)]
pub struct ChannelDeleteEvent {
    id: String,
}

impl ChannelDeleteEvent {
    /// Returns the channel id.
    pub fn id(&self) -> &str {
        &self.id
    }
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
