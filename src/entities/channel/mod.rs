pub use {text_channel::*, voice_channel::*};

mod text_channel;
mod voice_channel;

use serde::Deserialize;

/// A channel.
#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(tag = "channel_type")]
pub enum Channel {
    /// A text channel.
    TextChannel(TextChannel),
    /// A voice channel.
    VoiceChannel(VoiceChannel),
}

impl Channel {
    /// Returns the channel id.
    pub fn id(&self) -> &str {
        match self {
            Self::TextChannel(channel) => &channel.id,
            Self::VoiceChannel(channel) => &channel.id,
        }
    }

    /// Returns the channel server id.
    pub fn server_id(&self) -> &str {
        match self {
            Channel::TextChannel(channel) => &channel.server_id,
            Channel::VoiceChannel(channel) => &channel.server_id,
        }
    }
}
