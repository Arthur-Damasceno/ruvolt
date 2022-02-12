use serde::Deserialize;

use crate::models::Id;

/// A user has stopped typing in a channel.
#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct ChannelStopTypingEvent {
    /// Channel id.
    #[serde(rename = "id")]
    pub channel_id: Id,
    /// User id.
    #[serde(rename = "user")]
    pub user_id: Id,
}
