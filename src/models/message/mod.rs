pub use {content::*, embed::*};

mod content;
mod edited;
mod embed;

use {serde::Deserialize, serde_json::json};

use {
    crate::{
        models::{Attachment, Channel, Id, User},
        Context, Result,
    },
    edited::Edited,
};

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
    edited: Option<Edited>,
    /// Message mentions.
    #[serde(default)]
    pub mentions: Vec<Id>,
    /// Message replies.
    #[serde(default)]
    pub replies: Vec<Id>,
}

impl Message {
    /// Get a message from the API.
    pub async fn fetch(cx: &Context, channel_id: &Id, id: &Id) -> Result<Self> {
        let path = format!("channels/{}/messages/{}", channel_id, id);
        let msg = cx.http_client.get(&path).await?;

        Ok(msg)
    }

    /// Returns the message edit date.
    pub fn edited(&self) -> Option<&str> {
        match self.edited {
            Some(Edited { ref date }) => Some(date),
            None => None,
        }
    }

    /// Returns whether the message has been edited.
    pub fn is_edited(&self) -> bool {
        self.edited.is_some()
    }

    /// Get the message channel from the API.
    pub async fn fetch_channel(&self, cx: &Context) -> Result<Channel> {
        Channel::fetch(cx, &self.channel_id).await
    }

    /// Get the message author from the API.
    pub async fn fetch_author(&self, cx: &Context) -> Result<User> {
        User::fetch(cx, &self.author_id).await
    }

    /// Send a message in the same channel.
    pub async fn send_in_channel(&self, cx: &Context, content: &str) -> Result<Self> {
        let path = format!("channels/{}/messages", self.channel_id);
        let body = json!({ "content": content });
        let msg = cx.http_client.post(&path, body).await?;

        Ok(msg)
    }

    /// Edit the message.
    pub async fn edit(&mut self, cx: &Context, content: &str) -> Result {
        let path = format!("channels/{}/messages/{}", self.channel_id, self.id);
        let body = json!({ "content": content });

        cx.http_client.patch(&path, body).await?;
        self.content = Content::Text(content.into());

        Ok(())
    }

    /// Delete the message.
    pub async fn delete(&self, cx: &Context) -> Result {
        let path = format!("channels/{}/messages/{}", self.channel_id, self.id);
        cx.http_client.delete(&path).await?;

        Ok(())
    }
}
