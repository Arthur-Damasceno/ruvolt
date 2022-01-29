use {
    super::super::ServerToClientEvent,
    crate::{entities::User, Context, Result},
};

/// A user has left the group.
#[derive(Debug)]
pub struct ChannelGroupLeaveEvent {
    /// Channel id.
    pub id: String,
    /// User id.
    pub user_id: String,
}

impl ChannelGroupLeaveEvent {
    /// Get the user from the API.
    pub async fn fetch_user(&self, cx: &Context) -> Result<User> {
        User::fetch(cx, &self.user_id).await
    }
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
