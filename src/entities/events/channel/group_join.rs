use super::super::ServerToClientEvent;

/// A user has joined the group.
#[derive(Debug)]
pub struct ChannelGroupJoinEvent {
    /// Channel id.
    pub id: String,
    /// User id
    pub user_id: String,
}

impl From<ServerToClientEvent> for ChannelGroupJoinEvent {
    fn from(event: ServerToClientEvent) -> Self {
        if let ServerToClientEvent::ChannelGroupJoin { id, user_id } = event {
            Self { id, user_id }
        } else {
            panic!("An incorrect event was provided: {:?}", event);
        }
    }
}
