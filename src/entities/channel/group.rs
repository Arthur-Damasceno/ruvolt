use {serde::Deserialize, serde_json::json};

use crate::{
    entities::{Message, User},
    Context, Result,
};

/// A group channel.
#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct GroupChannel {
    /// Channel id.
    #[serde(rename = "_id")]
    pub id: String,
    /// Channel owner id.
    #[serde(rename = "owner")]
    pub owner_id: String,
    /// Channel name.
    pub name: String,
    /// Channel description.
    pub description: Option<String>,
    /// Id of last message in the channel.
    pub last_message_id: Option<String>,
    /// Channel is not safe for work.
    #[serde(default)]
    pub nsfw: bool,
}

impl GroupChannel {
    /// Get the group owner from the API.
    pub async fn fetch_owner(&self, cx: &Context) -> Result<User> {
        User::fetch(cx, &self.owner_id).await
    }

    /// Get the last message in the channel from the API.
    pub async fn fetch_last_msg(&self, cx: &Context) -> Result<Option<Message>> {
        match self.last_message_id {
            Some(ref msg_id) => {
                let msg = Message::fetch(cx, &self.id, msg_id).await?;

                Ok(Some(msg))
            }
            None => Ok(None),
        }
    }

    /// Send a message in this channel.
    pub async fn send(&self, cx: &Context, content: &str) -> Result<Message> {
        let path = format!("channels/{}/messages", self.id);
        let body = json!({ "content": content });
        let msg = cx.http_client.post(&path, body).await?;

        Ok(msg)
    }
}
