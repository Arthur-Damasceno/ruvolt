pub use {content::*, edited::*, embed::*, masquerade::*};

mod content;
mod edited;
mod embed;
mod masquerade;

use crate::models::Attachment;

#[cfg(feature = "cache")]
use crate::{cache::UpdateCache, models::Channel, Context};

/// A message.
#[derive(Debug, Clone, PartialEq, Deserialize)]
#[non_exhaustive]
pub struct Message {
    /// Message id.
    #[serde(rename = "_id")]
    pub id: String,
    /// Message nonce.
    pub nonce: Option<String>,
    /// Message channel id.
    #[serde(rename = "channel")]
    pub channel_id: String,
    /// Message author id.
    #[serde(rename = "author")]
    pub author_id: String,
    /// Message content.
    pub content: Content,
    /// Message attachments.
    #[serde(default)]
    pub attachments: Vec<Attachment>,
    /// Message embeds.
    #[serde(default)]
    pub embeds: Vec<Embed>,
    /// Message mentions.
    #[serde(default)]
    pub mentions: Vec<String>,
    /// Message replies.
    #[serde(default)]
    pub replies: Vec<String>,
    /// Message masquerade.
    pub masquerade: Option<Masquerade>,
    /// Edition date.
    pub edited: Option<MessageEdited>,
}

#[cfg(feature = "cache")]
#[async_trait]
impl UpdateCache for Message {
    async fn update(&self, cx: &Context) {
        if let Some(channel) = cx.cache.channels.write().await.get_mut(&self.channel_id) {
            match channel {
                Channel::Text(channel) => channel.last_message_id = Some(self.id.clone()),
                Channel::Group(channel) => channel.last_message_id = Some(self.id.clone()),
                Channel::DirectMessage(channel) => channel.last_message_id = Some(self.id.clone()),
                _ => {}
            }
        }
    }
}
