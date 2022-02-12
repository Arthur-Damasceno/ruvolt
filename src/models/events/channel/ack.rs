use crate::{
    models::{events::ServerToClientEvent, Channel, Id, Message, User},
    Context, Result,
};

/// You have acknowledged new messages in the channel up to the message id.
#[derive(Debug)]
pub struct ChannelAckEvent {
    /// Channel id.
    pub id: Id,
    /// User id.
    pub user_id: Id,
    /// Message id.
    pub message_id: Id,
}

impl ChannelAckEvent {
    /// Get the channel from the API.
    pub async fn fetch_channel(&self, cx: &Context) -> Result<Channel> {
        Channel::fetch(cx, &self.id).await
    }

    /// Get the user from the API.
    pub async fn fetch_user(&self, cx: &Context) -> Result<User> {
        User::fetch(cx, &self.user_id).await
    }

    /// Get the message from the API.
    pub async fn fetch_msg(&self, cx: &Context) -> Result<Message> {
        Message::fetch(cx, &self.id, &self.message_id).await
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
