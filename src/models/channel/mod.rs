pub use {direct_message::*, group::*, text::*, voice::*};

mod direct_message;
mod group;
mod text;
mod voice;

#[cfg(feature = "cache")]
use crate::{cache::UpdateCache, Context};

/// A channel.
#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(tag = "channel_type")]
#[non_exhaustive]
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
    /// Returns the channel id.
    pub fn id(&self) -> &str {
        match self {
            Self::Text(TextChannel { id, .. })
            | Self::Voice(VoiceChannel { id, .. })
            | Self::Group(GroupChannel { id, .. })
            | Self::DirectMessage(DirectMessageChannel { id, .. }) => id,
        }
    }
}

#[cfg(feature = "cache")]
#[async_trait]
impl UpdateCache for Channel {
    async fn update(&self, cx: &Context) {
        cx.cache
            .channels
            .write()
            .await
            .insert(self.id().into(), self.clone());
    }
}
