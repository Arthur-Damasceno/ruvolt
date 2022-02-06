use serde_json::Value as Json;

use {super::super::ServerToClientEvent, crate::models::Id};

/// A message has been edited or otherwise updated.
#[derive(Debug)]
pub struct MessageUpdateEvent {
    /// Message id.
    pub id: Id,
    /// A partial message object.
    pub data: Json,
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
