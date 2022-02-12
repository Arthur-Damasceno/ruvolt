use {
    super::super::ServerToClientEvent,
    crate::{
        models::{Channel, User},
        Context, Result,
    },
};

/// A user has started typing in a channel.
#[derive(Debug)]
pub struct ChannelStartTypingEvent {
    /// Channel id.
    pub id: String,
    /// User id.
    pub user_id: String,
}

impl ChannelStartTypingEvent {
    /// Get the channel from the API.
    pub async fn fetch_channel(&self, cx: &Context) -> Result<Channel> {
        Channel::fetch(cx, &self.id).await
    }

    /// Get the user from the API.
    pub async fn fetch_user(&self, cx: &Context) -> Result<User> {
        User::fetch(cx, &self.user_id).await
    }
}

impl From<ServerToClientEvent> for ChannelStartTypingEvent {
    fn from(event: ServerToClientEvent) -> Self {
        if let ServerToClientEvent::ChannelStartTyping { id, user_id } = event {
            Self { id, user_id }
        } else {
            panic!("An incorrect event was provided: {:?}", event);
        }
    }
}
