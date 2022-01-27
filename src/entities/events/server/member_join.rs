use crate::entities::ServerToClientEvent;

/// A user has joined the group.
#[derive(Debug)]
pub struct ServerMemberJoinEvent {
    id: String,
    user_id: String,
}

impl ServerMemberJoinEvent {
    /// Returns the server id.
    pub fn id(&self) -> &str {
        &self.id
    }

    /// Returns the user id.
    pub fn user_id(&self) -> &str {
        &self.user_id
    }
}

impl From<ServerToClientEvent> for ServerMemberJoinEvent {
    fn from(event: ServerToClientEvent) -> Self {
        if let ServerToClientEvent::ServerMemberJoin { id, user } = event {
            Self { id, user_id: user }
        } else {
            panic!("An incorrect event was provided: {:?}", event);
        }
    }
}
