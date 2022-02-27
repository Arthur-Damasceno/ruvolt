use serde::{Deserialize, Serialize};

use crate::models::{Attachment, Id};

/// Specifies a field to remove on channel update.
#[derive(Debug, Clone, Copy, PartialEq, Deserialize, Serialize)]
pub enum ChannelField {
    /// Channel icon.
    Icon,
    /// Channel description,
    Description,
}

/// A channel details were updated.
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct ChannelUpdateEvent {
    /// Channel id.
    #[serde(rename = "id")]
    pub channel_id: Id,
    /// A partial channel.
    pub data: PartialChannel,
    /// A specified field to remove on channel update.
    pub clear: Option<ChannelField>,
}

/// A partial channel.
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct PartialChannel {
    /// Channel name.
    pub name: Option<String>,
    /// Channel description.
    pub description: Option<String>,
    /// Channel icon.
    pub icon: Option<Attachment>,
    /// Whether channel is not safe for work.
    pub nsfw: Option<bool>,
}
