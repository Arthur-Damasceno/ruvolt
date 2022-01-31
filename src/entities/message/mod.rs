pub use content::*;

mod content;
mod edited;

use {serde::Deserialize, serde_json::json};

use {
    crate::{
        entities::{Channel, User},
        Context, Result,
    },
    edited::Edited,
};

/// A message.
#[derive(Debug, Deserialize, Clone, PartialEq)]
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
    edited: Option<Edited>,
    /// Message mentions.
    #[serde(default)]
    pub mentions: Vec<String>,
    /// Message replies.
    #[serde(default)]
    pub replies: Vec<String>,
}

impl Message {
    /// Get a message from the API.
    pub async fn fetch(cx: &Context, channel_id: &str, id: &str) -> Result<Self> {
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

    /// Reply the message.
    pub async fn reply(&self, cx: &Context, content: &str) -> Result<Self> {
        let path = format!("channels/{}/messages", self.channel_id);
        let body = json!({
            "content": content,
            "replies": [{
                "id": self.id,
                "mention": true,
            }]
        });
        let msg = cx.http_client.post(&path, body).await?;

        Ok(msg)
    }
}
