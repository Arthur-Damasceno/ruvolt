use crate::entities::ServerToClientEvent;

/// A user has left the group.
#[derive(Debug)]
pub struct ServerMemberLeaveEvent {
    /// Server id.
    pub id: String,
    /// Server member id.
    pub user_id: String,
}

impl From<ServerToClientEvent> for ServerMemberLeaveEvent {
    fn from(event: ServerToClientEvent) -> Self {
        if let ServerToClientEvent::ServerMemberLeave { id, user_id } = event {
            Self { id, user_id }
        } else {
            panic!("An incorrect event was provided: {:?}", event);
        }
    }
}
