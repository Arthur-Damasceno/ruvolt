use serde::Deserialize;

use crate::models::Id;

/// A user has started typing in a channel.
#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct ChannelStartTypingEvent {
    /// Channel id.
    #[serde(rename = "id")]
    pub channel_id: Id,
    /// User id.
    #[serde(rename = "user")]
    pub user_id: Id,
}
