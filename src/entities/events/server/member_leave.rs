use crate::entities::ServerToClientEvent;

/// A user has left the group.
#[derive(Debug)]
pub struct ServerMemberLeaveEvent {
    id: String,
    user_id: String,
}

impl ServerMemberLeaveEvent {
    /// Returns the server id.
    pub fn id(&self) -> &str {
        &self.id
    }

    /// Returns the user id.
    pub fn user_id(&self) -> &str {
        &self.user_id
    }
}

impl From<ServerToClientEvent> for ServerMemberLeaveEvent {
    fn from(event: ServerToClientEvent) -> Self {
        if let ServerToClientEvent::ServerMemberLeave { id, user } = event {
            Self { id, user_id: user }
        } else {
            panic!("An incorrect event was provided: {:?}", event);
        }
    }
}
