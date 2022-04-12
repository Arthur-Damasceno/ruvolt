use crate::{
    models::{Id, Server},
    Context, Result,
};

/// A server role has been deleted.
#[derive(Debug, Deserialize, Clone, PartialEq)]
#[non_exhaustive]
pub struct ServerRoleDeleteEvent {
    /// Server id.
    #[serde(rename = "id")]
    pub server_id: Id,
    /// Server role id.
    pub role_id: Id,
}

impl ServerRoleDeleteEvent {
    /// Fetch the server.
    pub async fn server(&self, cx: &Context) -> Result<Server> {
        Server::fetch(cx, &self.server_id).await
    }
}
