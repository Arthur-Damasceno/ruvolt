pub use {category::*, flags::*, member::*, system_message_channels::*};

mod category;
mod flags;
mod member;
mod system_message_channels;

use crate::models::Attachment;

/// A server.
#[derive(Debug, Clone, PartialEq, Deserialize)]
#[non_exhaustive]
pub struct Server {
    /// Server id.
    #[serde(rename = "_id")]
    pub id: String,
    /// Server owner id.
    #[serde(rename = "owner")]
    pub owner_id: String,
    /// Server name.
    pub name: String,
    /// Server description.
    pub description: Option<String>,
    /// Server channels ids.
    pub channels: Vec<String>,
    /// Server categories.
    #[serde(default)]
    pub categories: Vec<Category>,
    /// Server system message channels.
    #[serde(default)]
    pub system_messages: SystemMessageChannels,
    /// Server icon.
    pub icon: Option<Attachment>,
    /// Server banner.
    pub banner: Option<Attachment>,
    /// Server flags.
    #[serde(default)]
    pub flags: ServerFlags,
    /// Server is not safe for work.
    #[serde(default)]
    pub nsfw: bool,
}
