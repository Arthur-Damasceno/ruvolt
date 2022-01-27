use {serde::Deserialize, serde_json::Value as Json};

use crate::entities::ServerToClientEvent;

/// Specifies a field to remove on server update.
#[derive(Debug, Deserialize, Clone, Copy, PartialEq)]
pub enum RemoveServerField {
    /// Server icon.
    Icon,
    /// Server banner.
    Banner,
    /// Server description.
    Description,
}

/// A server details were updated.
#[derive(Debug)]
pub struct ServerUpdateEvent {
    id: String,
    data: Json,
    clear: Option<RemoveServerField>,
}

impl ServerUpdateEvent {
    /// Returns the server id.
    pub fn id(&self) -> &str {
        &self.id
    }

    /// Returns a partial server object.
    pub fn data(&self) -> &Json {
        &self.data
    }

    /// Returns a specified field to remove on server update.
    pub fn clear(&self) -> Option<RemoveServerField> {
        self.clear
    }
}

impl From<ServerToClientEvent> for ServerUpdateEvent {
    fn from(event: ServerToClientEvent) -> Self {
        if let ServerToClientEvent::ServerUpdate { id, data, clear } = event {
            Self { id, data, clear }
        } else {
            panic!("An incorrect event was provided: {:?}", event);
        }
    }
}
