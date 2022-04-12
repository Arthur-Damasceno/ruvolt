use serde_json::Value as Json;

use crate::{
    models::{Id, Server},
    Context, Result,
};

/// Specifies a field to remove on server role update.
#[derive(Debug, Deserialize, Clone, Copy, PartialEq)]
#[non_exhaustive]
pub enum RoleField {
    /// Role color.
    #[serde(rename = "Colour")]
    Color,
}

/// A server role details were updated.
#[derive(Debug, Deserialize, Clone, PartialEq)]
#[non_exhaustive]
pub struct ServerRoleUpdateEvent {
    /// Server id.
    #[serde(rename = "id")]
    pub server_id: Id,
    /// Server role id.
    pub role_id: Id,
    /// A partial server role object.
    pub data: Json,
    /// A specified field to remove on server role update.
    pub clear: Option<RoleField>,
}

impl ServerRoleUpdateEvent {
    /// Fetch the server.
    pub async fn server(&self, cx: &Context) -> Result<Server> {
        Server::fetch(cx, &self.server_id).await
    }
}
