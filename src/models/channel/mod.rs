pub use {direct_message::*, group::*, text::*, voice::*};

mod direct_message;
mod group;
mod text;
mod voice;

use serde::Deserialize;

use crate::{builders::EditChannel, models::Id, Context, Result};

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
        cx.http_client.get(&path).await
    }

    /// Returns the channel id.
    pub fn id(&self) -> &Id {
        match self {
            Self::Text(TextChannel { id, .. })
            | Self::Voice(VoiceChannel { id, .. })
            | Self::Group(GroupChannel { id, .. })
            | Self::DirectMessage(DirectMessageChannel { id, .. }) => id,
        }
    }

    async fn edit(cx: &Context, channel_id: &Id, builder: EditChannel) -> Result {
        let path = format!("channels/{}", channel_id);
        cx.http_client.patch(&path, builder).await
    }
}
