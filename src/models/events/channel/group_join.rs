use serde::Deserialize;

use crate::models::Id;

/// A user has joined the group.
#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct ChannelGroupJoinEvent {
    /// Channel id.
    #[serde(rename = "id")]
    pub channel_id: Id,
    /// User id.
    #[serde(rename = "user")]
    pub user_id: Id,
}
