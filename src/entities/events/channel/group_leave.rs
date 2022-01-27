use crate::entities::ServerToClientEvent;

/// A user has left the group.
#[derive(Debug)]
pub struct ChannelGroupLeaveEvent {
    id: String,
    user_id: String,
}

impl ChannelGroupLeaveEvent {
    /// Returns the channel id.
    pub fn id(&self) -> &str {
        &self.id
    }

    /// Returns the user id.
    pub fn user_id(&self) -> &str {
        &self.user_id
    }
}

impl From<ServerToClientEvent> for ChannelGroupLeaveEvent {
    fn from(event: ServerToClientEvent) -> Self {
        if let ServerToClientEvent::ChannelGroupLeave { id, user } = event {
            Self { id, user_id: user }
        } else {
            panic!("An incorrect event was provided: {:?}", event);
        }
    }
}
