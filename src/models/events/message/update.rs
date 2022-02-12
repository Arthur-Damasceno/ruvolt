use {serde::Deserialize, serde_json::Value as Json};

use crate::models::Id;

/// A message has been edited or otherwise updated.
#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct MessageUpdateEvent {
    /// Message id.
    #[serde(rename = "id")]
    pub message_id: Id,
    /// Message channel id.
    #[serde(rename = "channel")]
    pub channel_id: Id,
    /// A partial message object.
    pub data: Json,
}
