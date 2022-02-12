use serde::Deserialize;

use crate::models::Id;

/// A user has left the group.
#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct ChannelGroupLeaveEvent {
    /// Channel id.
    #[serde(rename = "id")]
    pub channel_id: Id,
    /// User id.
    #[serde(rename = "user")]
    pub user_id: Id,
}
