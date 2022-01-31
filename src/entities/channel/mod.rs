pub use {group::*, text::*, voice::*};

mod group;
mod text;
mod voice;

use serde::Deserialize;

use crate::{Context, Result};

/// A channel.
#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(tag = "channel_type")]
pub enum Channel {
    /// A text channel.
    #[serde(rename = "TextChannel")]
    Text(TextChannel),
    /// A voice channel.
    #[serde(rename = "VoiceChannel")]
    Voice(VoiceChannel),
    /// A group channel.
    Group(GroupChannel),
}

impl Channel {
    /// Get a channel from the API.
    pub async fn fetch(cx: &Context, id: &str) -> Result<Self> {
        let path = format!("channels/{}", id);
        let channel = cx.http_client.get(&path).await?;

        Ok(channel)
    }

    /// Returns the channel id.
    pub fn id(&self) -> &str {
        match self {
            Self::Text(TextChannel { id, .. }) => id,
            Self::Voice(VoiceChannel { id, .. }) => id,
            Self::Group(GroupChannel { id, .. }) => id,
        }
    }
}
