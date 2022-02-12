use serde::Deserialize;

use crate::{
    models::{Id, Server},
    Context, Result,
};

/// A user has joined the server.
#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct ServerMemberJoinEvent {
    /// Server id.
    #[serde(rename = "id")]
    pub server_id: Id,
    /// User id.
    #[serde(rename = "user")]
    pub user_id: Id,
}

impl ServerMemberJoinEvent {
    /// Get the server from the API.
    pub async fn fetch_server(&self, cx: &Context) -> Result<Server> {
        Server::fetch(cx, &self.server_id).await
    }
}
