use super::super::ServerToClientEvent;

/// A server role has been deleted.
#[derive(Debug)]
pub struct ServerRoleDeleteEvent {
    /// Server id.
    pub id: String,
    /// Server role id.
    pub role_id: String,
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
