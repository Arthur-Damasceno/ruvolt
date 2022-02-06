use {
    super::super::ServerToClientEvent,
    crate::{
        models::{Channel, Id},
        Context, Result,
    },
};

/// A message has been deleted.
#[derive(Debug)]
pub struct MessageDeleteEvent {
    /// Message id.
    pub id: Id,
    /// Message channel id.
    pub channel_id: Id,
}

impl MessageDeleteEvent {
    /// Get the channel from the API.
    pub async fn fetch_channel(&self, cx: &Context) -> Result<Channel> {
        Channel::fetch(cx, &self.channel_id).await
    }
}

impl From<ServerToClientEvent> for MessageDeleteEvent {
    fn from(event: ServerToClientEvent) -> Self {
        if let ServerToClientEvent::MessageDelete { id, channel_id } = event {
            Self { id, channel_id }
        } else {
            panic!("An incorrect event was provided: {:?}", event);
        }
    }
}
