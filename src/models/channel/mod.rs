pub use {direct_message::*, group::*, permissions::*, text::*, voice::*};

mod direct_message;
mod group;
mod permissions;
mod text;
mod voice;

use {serde::Deserialize, serde_json::json};

use crate::{
    models::{Id, Message},
    Context, Result,
};

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
            Self::Text(TextChannel { id, .. })
            | Self::Voice(VoiceChannel { id, .. })
            | Self::Group(GroupChannel { id, .. })
            | Self::DirectMessage(DirectMessageChannel { id, .. }) => id,
        }
    }

    /// Send a message in a channel.
    pub async fn send(cx: &Context, id: &Id, content: &str) -> Result<Message> {
        let path = format!("channels/{}/messages", id);
        let body = json!({ "content": content });
        let msg = cx.http_client.post(&path, body).await?;

        Ok(msg)
    }

    /// Delete the channel.
    pub async fn delete(cx: &Context, id: &Id) -> Result {
        let path = format!("channels/{}", id);
        cx.http_client.delete(&path).await?;

        Ok(())
    }
}
