use crate::entities::ServerToClientEvent;

/// A user has left the group.
#[derive(Debug)]
pub struct ChannelGroupLeaveEvent {
    /// Channel id.
    pub id: String,
    /// User id.
    pub user_id: String,
}

impl From<ServerToClientEvent> for ChannelGroupLeaveEvent {
    fn from(event: ServerToClientEvent) -> Self {
        if let ServerToClientEvent::ChannelGroupLeave { id, user_id } = event {
            Self { id, user_id }
        } else {
            panic!("An incorrect event was provided: {:?}", event);
        }
    }
}
