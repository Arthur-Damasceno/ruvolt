use {serde::Deserialize, std::collections::HashMap};

use crate::{
    models::{Attachment, Channel, ChannelPermissionsRaw, Id, Message, Server},
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
    #[serde(default)]
    pub default_permissions: ChannelPermissionsRaw,
    /// Permissions given to roles.
    #[serde(default)]
    pub role_permissions: HashMap<Id, ChannelPermissionsRaw>,
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

    /// Send a message in the channel.
    pub async fn send(&self, cx: &Context, content: &str) -> Result<Message> {
        Channel::send(cx, &self.id, content).await
    }

    /// Delete the channel.
    pub async fn delete(&self, cx: &Context) -> Result {
        Channel::delete(cx, &self.id).await
    }
}
