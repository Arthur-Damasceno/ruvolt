use crate::entities::ServerToClientEvent;

/// A message has been deleted.
#[derive(Debug)]
pub struct MessageDeleteEvent {
    id: String,
    channel_id: String,
}

impl MessageDeleteEvent {
    /// Returns the message id.
    pub fn id(&self) -> &str {
        &self.id
    }

    /// Returns the channel id.
    pub fn channel_id(&self) -> &str {
        &self.channel_id
    }
}

impl From<ServerToClientEvent> for MessageDeleteEvent {
    fn from(event: ServerToClientEvent) -> Self {
        if let ServerToClientEvent::MessageDelete { id, channel } = event {
            Self {
                id,
                channel_id: channel,
            }
        } else {
            panic!("An incorrect event was provided: {:?}", event);
        }
    }
}
