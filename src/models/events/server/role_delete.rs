use crate::{
    models::{events::ServerToClientEvent, Id, Server},
    Context, Result,
};

/// A server role has been deleted.
#[derive(Debug)]
pub struct ServerRoleDeleteEvent {
    /// Server id.
    pub id: Id,
    /// Server role id.
    pub role_id: Id,
}

impl ServerRoleDeleteEvent {
    /// Get the server from the API.
    pub async fn fetch_server(&self, cx: &Context) -> Result<Server> {
        Server::fetch(cx, &self.id).await
    }
}

impl From<ServerToClientEvent> for ServerRoleDeleteEvent {
    fn from(event: ServerToClientEvent) -> Self {
        if let ServerToClientEvent::ServerRoleDelete { id, role_id } = event {
            Self { id, role_id }
        } else {
            panic!("An incorrect event was provided: {:?}", event);
        }
    }
}
