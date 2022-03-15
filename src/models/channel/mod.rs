pub use {direct_message::*, group::*, text::*, voice::*};

mod direct_message;
mod group;
mod text;
mod voice;

use serde::Deserialize;

use crate::{builders::EditChannel, models::Id, Context, Result};

#[cfg(feature = "cache")]
use crate::cache::UpdateCache;

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
    /// Get a channel from the cache or API.
    pub async fn fetch(cx: &Context, id: &Id) -> Result<Self> {
        #[cfg(feature = "cache")]
        if let Some(channel) = cx.cache.channel(id).await {
            return Ok(channel);
        }

        cx.http_client.get(format!("channels/{}", id)).await
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
        cx.http_client
            .patch(format!("channels/{}", channel_id), builder)
            .await
    }

    async fn delete(cx: &Context, channel_id: &Id) -> Result {
        cx.http_client
            .delete(format!("channels/{}", channel_id))
            .await
    }
}

#[cfg(feature = "cache")]
#[async_trait::async_trait]
impl UpdateCache for Channel {
    async fn update(&self, cx: &Context) {
        cx.cache
            .channels
            .write()
            .await
            .insert(self.id().clone(), self.clone());
    }
}
