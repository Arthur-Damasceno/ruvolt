use super::super::ServerToClientEvent;

/// A user has stopped typing in a channel.
#[derive(Debug)]
pub struct ChannelStopTypingEvent {
    /// Channel id.
    pub id: String,
    /// User id.
    pub user_id: String,
}

impl From<ServerToClientEvent> for ChannelStopTypingEvent {
    fn from(event: ServerToClientEvent) -> Self {
        if let ServerToClientEvent::ChannelStopTyping { id, user_id } = event {
            Self { id, user_id }
        } else {
            panic!("An incorrect event was provided: {:?}", event);
        }
    }
}
