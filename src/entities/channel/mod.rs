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
            Self::TextChannel(TextChannel { id, .. }) => id,
            Self::VoiceChannel(VoiceChannel { id, .. }) => id,
        }
    }

    /// Returns the channel server id.
    pub fn server_id(&self) -> &str {
        match self {
            Channel::TextChannel(TextChannel { server_id, .. }) => server_id,
            Channel::VoiceChannel(VoiceChannel { server_id, .. }) => server_id,
        }
    }
}
