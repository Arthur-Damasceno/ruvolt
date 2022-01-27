use crate::entities::ServerToClientEvent;

/// A server role has been deleted.
#[derive(Debug)]
pub struct ServerRoleDeleteEvent {
    id: String,
    role_id: String,
}

impl ServerRoleDeleteEvent {
    /// Returns the server id.
    pub fn id(&self) -> &str {
        &self.id
    }

    /// Returns the role id.
    pub fn role_id(&self) -> &str {
        &self.role_id
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
