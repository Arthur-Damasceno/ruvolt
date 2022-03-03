use serde::{Deserialize, Serialize};

use crate::{
    models::{Attachment, Category, Id, Server, SystemMessageChannels},
    Context, Result,
};

/// Specifies a field to remove on server update.
#[derive(Debug, Clone, Copy, PartialEq, Deserialize, Serialize)]
pub enum ServerField {
    /// Server icon.
    Icon,
    /// Server banner.
    Banner,
    /// Server description.
    Description,
}

/// A server details were updated.
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct ServerUpdateEvent {
    /// Server id.
    #[serde(rename = "id")]
    pub server_id: Id,
    /// A partial server object.
    pub data: PartialServer,
    /// A specified field to remove on server update.
    pub clear: Option<ServerField>,
}

impl ServerUpdateEvent {
    /// Fetch the server.
    pub async fn server(&self, cx: &Context) -> Result<Server> {
        Server::fetch(cx, &self.server_id).await
    }
}

/// A partial server.
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct PartialServer {
    /// Server name.
    pub name: Option<String>,
    /// Server description.
    pub description: Option<String>,
    /// Server icon.
    pub icon: Option<Attachment>,
    /// Server banner.
    pub banner: Option<Attachment>,
    /// Server categories.
    #[serde(default)]
    pub categories: Vec<Category>,
    /// Server system message channels.
    pub system_messages: Option<SystemMessageChannels>,
    /// Whether server is not safe for work.
    pub nsfw: Option<bool>,
}
