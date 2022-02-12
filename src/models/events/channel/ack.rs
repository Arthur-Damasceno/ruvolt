use serde::Deserialize;

use crate::models::Id;

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
