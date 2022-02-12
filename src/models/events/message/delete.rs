use serde::Deserialize;

use crate::models::Id;

/// A message has been deleted.
#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct MessageDeleteEvent {
    /// Message id.
    #[serde(rename = "id")]
    pub message_id: Id,
    /// Message channel id.
    #[serde(rename = "channel")]
    pub channel_id: Id,
}
