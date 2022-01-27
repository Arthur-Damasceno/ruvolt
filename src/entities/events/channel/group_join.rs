use crate::entities::ServerToClientEvent;

/// A user has joined the group.
#[derive(Debug)]
pub struct ChannelGroupJoinEvent {
    id: String,
    user_id: String,
}

impl ChannelGroupJoinEvent {
    /// Returns the channel id.
    pub fn id(&self) -> &str {
        &self.id
    }

    /// Returns the user id.
    pub fn user_id(&self) -> &str {
        &self.user_id
    }
}

impl From<ServerToClientEvent> for ChannelGroupJoinEvent {
    fn from(event: ServerToClientEvent) -> Self {
        if let ServerToClientEvent::ChannelGroupJoin { id, user } = event {
            Self { id, user_id: user }
        } else {
            panic!("An incorrect event was provided: {:?}", event);
        }
    }
}
