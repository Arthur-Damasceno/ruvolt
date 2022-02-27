use serde::Deserialize;

use crate::models::{Embed, Id, MessageEdited};

/// A message has been edited or otherwise updated.
#[derive(Debug, Clone, PartialEq, Deserialize)]
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
