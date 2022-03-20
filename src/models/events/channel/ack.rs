use serde::Deserialize;

use crate::{
    models::{Channel, Id, Message, User},
    Context, Result,
};

/// You have acknowledged new messages in the channel up to the message id.
#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct ChannelAckEvent {
    /// Channel id.
    #[serde(rename = "id")]
    pub channel_id: Id,
    /// User id.
    #[serde(rename = "user")]
    pub user_id: Id,
    /// Message id.
    pub message_id: Id,
}

impl ChannelAckEvent {
    /// Fetch the channel.
    pub async fn channel(&self, cx: &Context) -> Result<Channel> {
        Channel::fetch(cx, &self.channel_id).await
    }

    /// Fetch the user.
    pub async fn user(&self, cx: &Context) -> Result<User> {
        User::fetch(cx, &self.user_id).await
    }

    /// Fetch the message.
    pub async fn message(&self, cx: &Context) -> Result<Message> {
        Message::fetch(cx, &self.channel_id, &self.message_id).await
    }
}
