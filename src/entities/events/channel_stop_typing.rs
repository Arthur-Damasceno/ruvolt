use super::ServerToClientEvent;

/// A user has stopped typing in a channel.
#[derive(Debug)]
pub struct ChannelStopTypingEvent {
    id: String,
    user_id: String,
}

impl ChannelStopTypingEvent {
    /// Returns the channel id.
    pub fn id(&self) -> &str {
        &self.id
    }

    /// Returns the user id.
    pub fn user_id(&self) -> &str {
        &self.user_id
    }
}

impl From<ServerToClientEvent> for ChannelStopTypingEvent {
    fn from(event: ServerToClientEvent) -> Self {
        if let ServerToClientEvent::ChannelStopTyping { id, user } = event {
            Self { id, user_id: user }
        } else {
            panic!("An incorrect event was provided: {:?}", event);
        }
    }
}
