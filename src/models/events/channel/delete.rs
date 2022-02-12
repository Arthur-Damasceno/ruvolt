use serde::Deserialize;

use crate::models::Id;

/// A channel has been deleted.
#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct ChannelDeleteEvent {
    /// Channel id.
    #[serde(rename = "id")]
    pub channel_id: Id,
}
