use {serde::Deserialize, std::collections::HashMap};

use crate::{
    models::{Attachment, Channel, ChannelPermissionsRaw, Id, Server},
    Context, Result,
};

/// A voice channel.
#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct VoiceChannel {
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

impl VoiceChannel {
    /// Get the server from the API.
    pub async fn fetch_server(&self, cx: &Context) -> Result<Server> {
        Server::fetch(cx, &self.server_id).await
    }

    /// Delete the channel.
    pub async fn delete(&self, cx: &Context) -> Result {
        Channel::delete(cx, &self.id).await
    }
}
