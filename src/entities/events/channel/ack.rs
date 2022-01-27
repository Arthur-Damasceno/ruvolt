use crate::entities::ServerToClientEvent;

/// You have acknowledged new messages in the channel up to the message id.
#[derive(Debug)]
pub struct ChannelAckEvent {
    id: String,
    user_id: String,
    message_id: String,
}

impl ChannelAckEvent {
    /// Returns the channel id.
    pub fn id(&self) -> &str {
        &self.id
    }

    /// Returns the user id.
    pub fn user_id(&self) -> &str {
        &self.user_id
    }

    /// Returns the message id.
    pub fn message_id(&self) -> &str {
        &self.message_id
    }
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
