use {serde::Deserialize, serde_json::json, std::collections::HashMap};

use crate::{
    models::{Attachment, Id, Message, Server},
    Context, Result,
};

/// A text channel.
#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct TextChannel {
    /// Channel id.
    #[serde(rename = "_id")]
    pub id: Id,
    /// Channel server id.
    #[serde(rename = "server")]
    pub server_id: Id,
    /// Channel name.
    pub name: String,
    /// Channel description.
    pub description: Option<String>,
    /// Channel icon.
    pub icon: Option<Attachment>,
    /// Id of the last message in the channel.
    pub last_message_id: Option<Id>,
    /// Permissions given to all users.
    pub default_permissions: Option<u32>,
    /// Permissions given to roles.
    #[serde(default)]
    pub role_permissions: HashMap<Id, u32>,
    /// Channel is not safe for work.
    #[serde(default)]
    pub nsfw: bool,
}

impl TextChannel {
    /// Get the server from the API.
    pub async fn fetch_server(&self, cx: &Context) -> Result<Server> {
        Server::fetch(cx, &self.server_id).await
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

    /// Delete the channel.
    pub async fn delete(&self, cx: &Context) -> Result {
        let path = format!("channels/{}", self.id);
        cx.http_client.delete(&path).await?;

        Ok(())
    }
}