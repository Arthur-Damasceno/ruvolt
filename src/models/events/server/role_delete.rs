use serde::Deserialize;

use crate::{
    models::{Id, Server},
    Context, Result,
};

/// A server role has been deleted.
#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct ServerRoleDeleteEvent {
    /// Server id.
    #[serde(rename = "id")]
    pub server_id: Id,
    /// Server role id.
    pub role_id: Id,
}

impl ServerRoleDeleteEvent {
    /// Get the server from the API.
    pub async fn fetch_server(&self, cx: &Context) -> Result<Server> {
        Server::fetch(cx, &self.server_id).await
    }
}
