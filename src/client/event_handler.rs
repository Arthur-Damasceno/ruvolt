use async_trait::async_trait;

use crate::{entities::ServerToClientEvent, error::Error};

/// Define handlers for supported events.
#[async_trait]
pub trait EventHandler: Send + Sync + 'static {
    /// An error has occurred.
    async fn error(&self, error: Error);
}

#[async_trait]
pub(crate) trait EventHandlerExt: EventHandler {
    async fn handle(&self, event: ServerToClientEvent) {
        match event {
            ServerToClientEvent::Pong { .. } => return,
            event => {
                let error = Error::Unknown(format!("Unexpected event from server: {:?}", event));
                self.error(error).await;
            }
        }
    }
}

impl<T: EventHandler> EventHandlerExt for T {}
