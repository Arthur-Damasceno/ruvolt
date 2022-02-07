pub use {direct_message::*, group::*, permissions::*, text::*, voice::*};

mod direct_message;
mod group;
mod permissions;
mod text;
mod voice;

use serde::Deserialize;

use crate::{models::Id, Context, Result};

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
    /// A DM channel.
    DirectMessage(DirectMessageChannel),
}

impl Channel {
    /// Get a channel from the API.
    pub async fn fetch(cx: &Context, id: &Id) -> Result<Self> {
        let path = format!("channels/{}", id);
        let channel = cx.http_client.get(&path).await?;

        Ok(channel)
    }

    /// Returns the channel id.
    pub fn id(&self) -> &Id {
        match self {
            Self::Text(TextChannel { id, .. }) => id,
            Self::Voice(VoiceChannel { id, .. }) => id,
            Self::Group(GroupChannel { id, .. }) => id,
            Self::DirectMessage(DirectMessageChannel { id, .. }) => id,
        }
    }
}
