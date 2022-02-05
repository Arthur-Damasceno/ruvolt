use {
    super::super::ServerToClientEvent,
    crate::{
        entities::{Channel, Id, User},
        Context, Result,
    },
};

/// A user has stopped typing in a channel.
#[derive(Debug)]
pub struct ChannelStopTypingEvent {
    /// Channel id.
    pub id: Id,
    /// User id.
    pub user_id: Id,
}

impl ChannelStopTypingEvent {
    /// Get the channel from the API.
    pub async fn fetch_channel(&self, cx: &Context) -> Result<Channel> {
        Channel::fetch(cx, &self.id).await
    }

    /// Get the user from the API.
    pub async fn fetch_user(&self, cx: &Context) -> Result<User> {
        User::fetch(cx, &self.user_id).await
    }
}

impl From<ServerToClientEvent> for ChannelStopTypingEvent {
    fn from(event: ServerToClientEvent) -> Self {
        if let ServerToClientEvent::ChannelStopTyping { id, user_id } = event {
            Self { id, user_id }
        } else {
            panic!("An incorrect event was provided: {:?}", event);
        }
    }
}
