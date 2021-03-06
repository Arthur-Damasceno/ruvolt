pub use {content::*, edited::*, embed::*, masquerade::*};

mod content;
mod edited;
mod embed;
mod masquerade;

use serde::Deserialize;

use crate::{
    builders::{CreateMessage, EditMessage},
    models::{Attachment, Id},
    Context, Result,
};

#[cfg(feature = "cache")]
use crate::{cache::UpdateCache, models::Channel};

/// A message.
#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct Message {
    /// Message id.
    #[serde(rename = "_id")]
    pub id: Id,
    /// Message nonce.
    pub nonce: Option<String>,
    /// Message channel id.
    #[serde(rename = "channel")]
    pub channel_id: Id,
    /// Message author id.
    #[serde(rename = "author")]
    pub author_id: Id,
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
    pub mentions: Vec<Id>,
    /// Message replies.
    #[serde(default)]
    pub replies: Vec<Id>,
    /// Message masquerade.
    pub masquerade: Option<Masquerade>,
    /// Edition date.
    pub edited: Option<MessageEdited>,
}

impl Message {
    /// Get a message from the API.
    pub async fn fetch(cx: &Context, channel_id: &Id, id: &Id) -> Result<Self> {
        let path = format!("channels/{}/messages/{}", channel_id, id);
        let msg = cx.http_client.get(&path).await?;

        Ok(msg)
    }

    pub(crate) async fn create(
        cx: &Context,
        channel_id: &Id,
        builder: CreateMessage,
    ) -> Result<Self> {
        let path = format!("channels/{}/messages", channel_id);
        let msg = cx.http_client.post(&path, builder).await?;

        Ok(msg)
    }

    /// Returns whether the message has been edited.
    pub fn is_edited(&self) -> bool {
        self.edited.is_some()
    }

    /// Reply the message.
    pub async fn reply(
        &self,
        cx: &Context,
        builder: impl Into<CreateMessage>,
        mention: bool,
    ) -> Result<Self> {
        Self::create(
            cx,
            &self.channel_id,
            builder.into().reply(&self.id, mention),
        )
        .await
    }

    /// Edit the message.
    pub async fn edit(&mut self, cx: &Context, builder: impl Into<EditMessage>) -> Result {
        // TODO: Update local message.
        let path = format!("channels/{}/messages/{}", self.channel_id, self.id);
        cx.http_client.patch(&path, builder.into()).await?;

        Ok(())
    }

    /// Delete the message.
    pub async fn delete(&self, cx: &Context) -> Result {
        let path = format!("channels/{}/messages/{}", self.channel_id, self.id);
        cx.http_client.delete(&path).await?;

        Ok(())
    }
}

#[cfg(feature = "cache")]
#[async_trait::async_trait]
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
