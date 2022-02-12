use {serde::Deserialize, serde_json::Value as Json};

use crate::{
    models::{events::ServerToClientEvent, Id, Server},
    Context, Result,
};

/// Specifies a field to remove on server role update.
#[derive(Debug, Deserialize, Clone, Copy, PartialEq)]
pub enum RemoveServerRoleField {
    /// Role color.
    #[serde(rename = "Colour")]
    Color,
}

/// A server role details were updated.
#[derive(Debug)]
pub struct ServerRoleUpdateEvent {
    /// Server id.
    pub id: Id,
    /// Server role id.
    pub role_id: Id,
    /// A partial server role object.
    pub data: Json,
    /// A specified field to remove on server role update.
    pub clear: Option<RemoveServerRoleField>,
}

impl ServerRoleUpdateEvent {
    /// Get the server from the API.
    pub async fn fetch_server(&self, cx: &Context) -> Result<Server> {
        Server::fetch(cx, &self.id).await
    }
}

impl From<ServerToClientEvent> for ServerRoleUpdateEvent {
    fn from(event: ServerToClientEvent) -> Self {
        if let ServerToClientEvent::ServerRoleUpdate {
            id,
            role_id,
            data,
            clear,
        } = event
        {
            Self {
                id,
                role_id,
                data,
                clear,
            }
        } else {
            panic!("An incorrect event was provided: {:?}", event);
        }
    }
}
