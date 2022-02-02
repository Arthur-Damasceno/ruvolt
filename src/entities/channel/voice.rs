use serde::Deserialize;

use crate::{entities::Server, Context, Result};

/// A voice channel.
#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct VoiceChannel {
    /// Channel id.
    #[serde(rename = "_id")]
    pub id: String,
    /// Channel server id.
    #[serde(rename = "server")]
    pub server_id: String,
    /// Channel name.
    pub name: String,
    /// Channel description.
    pub description: Option<String>,
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
        let path = format!("channels/{}", self.id);
        cx.http_client.delete(&path).await?;

        Ok(())
    }
}
