use async_trait::async_trait;

use crate::{
    entities::{ChannelStartTypingEvent, ServerToClientEvent},
    error::Error,
};

/// Define handlers for supported events.
#[async_trait]
pub trait EventHandler: Send + Sync + 'static {
    /// An error has occurred.
    async fn error(&self, _error: Error) {}

    /// A user has started typing in a channel.
    async fn channel_start_typing(&self, _data: ChannelStartTypingEvent) {}
}

#[async_trait]
pub(crate) trait EventHandlerExt: EventHandler {
    async fn handle(&self, event: ServerToClientEvent) {
        match event {
            ServerToClientEvent::Pong { .. } => return,
            ServerToClientEvent::ChannelStartTyping { .. } => {
                self.channel_start_typing(ChannelStartTypingEvent::from(event))
                    .await;
            }
            event => {
                let error = Error::Unknown(format!("Unexpected event from server: {:?}", event));
                self.error(error).await;
            }
        }
    }
}

impl<T: EventHandler> EventHandlerExt for T {}
