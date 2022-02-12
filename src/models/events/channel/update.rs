use {serde::Deserialize, serde_json::Value as Json};

use crate::models::Id;

/// Specifies a field to remove on channel update.
#[derive(Debug, Deserialize, Clone, Copy, PartialEq)]
pub enum RemoveChannelField {
    /// Channel icon.
    Icon,
    /// Channel description,
    Description,
}

/// A channel details were updated.
#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct ChannelUpdateEvent {
    /// Channel id.
    #[serde(rename = "id")]
    pub channel_id: Id,
    /// A partial channel object.
    pub data: Json,
    /// A specified field to remove on channel update.
    pub clear: Option<RemoveChannelField>,
}
