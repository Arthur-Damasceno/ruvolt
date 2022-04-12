use crate::{
    models::{Channel, Embed, Id, Message, MessageEdited},
    Context, Result,
};

/// A message has been edited or otherwise updated.
#[derive(Debug, Clone, PartialEq, Deserialize)]
#[non_exhaustive]
pub struct MessageUpdateEvent {
    /// Message id.
    #[serde(rename = "id")]
    pub message_id: Id,
    /// Message channel id.
    #[serde(rename = "channel")]
    pub channel_id: Id,
    /// A partial message.
    pub data: PartialMessage,
}

impl MessageUpdateEvent {
    /// Fetch the message.
    pub async fn message(&self, cx: &Context) -> Result<Message> {
        Message::fetch(cx, &self.channel_id, &self.message_id).await
    }

    /// Fetch the channel.
    pub async fn channel(&self, cx: &Context) -> Result<Channel> {
        Channel::fetch(cx, &self.channel_id).await
    }
}

/// A partial message.
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct PartialMessage {
    /// Message content.
    pub content: Option<String>,
    /// Message embeds.
    #[serde(default)]
    pub embeds: Vec<Embed>,
    /// Message edition date.
    pub edited: MessageEdited,
}
