use crate::entities::ServerToClientEvent;

/// A user has started typing in a channel.
#[derive(Debug)]
pub struct ChannelStartTypingEvent {
    id: String,
    user_id: String,
}

impl ChannelStartTypingEvent {
    /// Returns the channel id.
    pub fn id(&self) -> &str {
        &self.id
    }

    /// Returns the user id.
    pub fn user_id(&self) -> &str {
        &self.user_id
    }
}

impl From<ServerToClientEvent> for ChannelStartTypingEvent {
    fn from(event: ServerToClientEvent) -> Self {
        if let ServerToClientEvent::ChannelStartTyping { id, user } = event {
            Self { id, user_id: user }
        } else {
            panic!("An incorrect event was provided: {:?}", event);
        }
    }
}
