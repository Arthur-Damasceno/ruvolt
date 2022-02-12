use serde::Deserialize;

use crate::{
    models::{Id, Server},
    Context, Result,
};

/// A user has left the server.
#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct ServerMemberLeaveEvent {
    /// Server id.
    #[serde(rename = "id")]
    pub server_id: Id,
    /// User id.
    #[serde(rename = "user")]
    pub user_id: Id,
}

impl ServerMemberLeaveEvent {
    /// Get the server from the API.
    pub async fn fetch_server(&self, cx: &Context) -> Result<Server> {
        Server::fetch(cx, &self.server_id).await
    }
}
