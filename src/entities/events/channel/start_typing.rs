use crate::entities::ServerToClientEvent;

/// A user has started typing in a channel.
#[derive(Debug)]
pub struct ChannelStartTypingEvent {
    /// Channel id.
    pub id: String,
    /// User id.
    pub user_id: String,
}

impl From<ServerToClientEvent> for ChannelStartTypingEvent {
    fn from(event: ServerToClientEvent) -> Self {
        if let ServerToClientEvent::ChannelStartTyping { id, user_id } = event {
            Self { id, user_id }
        } else {
            panic!("An incorrect event was provided: {:?}", event);
        }
    }
}
