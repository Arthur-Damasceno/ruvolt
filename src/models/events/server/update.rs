use {serde::Deserialize, serde_json::Value as Json};

use {super::super::ServerToClientEvent, crate::models::Id};

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
    /// Server id.
    pub id: Id,
    /// A partial server object.
    pub data: Json,
    /// A specified field to remove on server update.
    pub clear: Option<RemoveServerField>,
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
