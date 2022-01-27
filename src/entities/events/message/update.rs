use serde_json::Value as Json;

use crate::entities::ServerToClientEvent;

/// A message has been edited or otherwise updated.
#[derive(Debug)]
pub struct MessageUpdateEvent {
    id: String,
    data: Json,
}

impl MessageUpdateEvent {
    /// Returns the message id.
    pub fn id(&self) -> &str {
        &self.id
    }

    /// Returns a partial message object.
    pub fn data(&self) -> &Json {
        &self.data
    }
}

impl From<ServerToClientEvent> for MessageUpdateEvent {
    fn from(event: ServerToClientEvent) -> Self {
        if let ServerToClientEvent::MessageUpdate { id, data } = event {
            Self { id, data }
        } else {
            panic!("An incorrect event was provided: {:?}", event);
        }
    }
}
