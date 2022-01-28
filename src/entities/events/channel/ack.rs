use super::super::ServerToClientEvent;

/// You have acknowledged new messages in the channel up to the message id.
#[derive(Debug)]
pub struct ChannelAckEvent {
    /// Channel id.
    pub id: String,
    /// User id.
    pub user_id: String,
    /// Message id.
    pub message_id: String,
}

impl From<ServerToClientEvent> for ChannelAckEvent {
    fn from(event: ServerToClientEvent) -> Self {
        if let ServerToClientEvent::ChannelAck {
            id,
            user_id,
            message_id,
        } = event
        {
            Self {
                id,
                user_id,
                message_id,
            }
        } else {
            panic!("An incorrect event was provided: {:?}", event);
        }
    }
}
