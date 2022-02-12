use {
    super::super::ServerToClientEvent,
    crate::{
        models::{Channel, User},
        Context, Result,
    },
};

/// A user has joined the group.
#[derive(Debug)]
pub struct ChannelGroupJoinEvent {
    /// Channel id.
    pub id: String,
    /// User id
    pub user_id: String,
}

impl ChannelGroupJoinEvent {
    /// Get the channel from the API.
    pub async fn fetch_channel(&self, cx: &Context) -> Result<Channel> {
        Channel::fetch(cx, &self.id).await
    }

    /// Get the user from the API.
    pub async fn fetch_user(&self, cx: &Context) -> Result<User> {
        User::fetch(cx, &self.user_id).await
    }
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
