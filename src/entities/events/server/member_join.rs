use super::super::ServerToClientEvent;

/// A user has joined the group.
#[derive(Debug)]
pub struct ServerMemberJoinEvent {
    /// Server id.
    pub id: String,
    /// Server member id.
    pub user_id: String,
}

impl From<ServerToClientEvent> for ServerMemberJoinEvent {
    fn from(event: ServerToClientEvent) -> Self {
        if let ServerToClientEvent::ServerMemberJoin { id, user_id } = event {
            Self { id, user_id }
        } else {
            panic!("An incorrect event was provided: {:?}", event);
        }
    }
}
