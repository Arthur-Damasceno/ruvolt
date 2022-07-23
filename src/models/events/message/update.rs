use crate::models::message::Embed;

/// A message has been edited or otherwise updated.
#[derive(Debug, Clone, Deserialize)]
#[non_exhaustive]
pub struct MessageUpdateEvent {
    /// Message id.
    #[serde(rename = "id")]
    pub message_id: String,
    /// Message channel id.
    #[serde(rename = "channel")]
    pub channel_id: String,
    /// A partial message.
    pub data: PartialMessage,
}

/// A partial message.
#[derive(Debug, Clone, Deserialize)]
pub struct PartialMessage {
    /// Message content.
    pub content: Option<String>,
    /// Message embeds.
    #[serde(default)]
    pub embeds: Vec<Embed>,
    /// Message edition date.
    pub edited: String,
}
