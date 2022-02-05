use {serde::Deserialize, serde_json::Value as Json};

use {super::super::ServerToClientEvent, crate::entities::Id};

/// Specifies a field to remove on channel update.
#[derive(Debug, Deserialize, Clone, Copy, PartialEq)]
pub enum RemoveChannelField {
    /// Channel icon.
    Icon,
    /// Channel description,
    Description,
}

/// A channel details were updated.
#[derive(Debug)]
pub struct ChannelUpdateEvent {
    /// Channel id.
    pub id: Id,
    /// A partial channel object.
    pub data: Json,
    /// A specified field to remove on channel update.
    pub clear: Option<RemoveChannelField>,
}

impl From<ServerToClientEvent> for ChannelUpdateEvent {
    fn from(event: ServerToClientEvent) -> Self {
        if let ServerToClientEvent::ChannelUpdate { id, data, clear } = event {
            Self { id, data, clear }
        } else {
            panic!("An incorrect event was provided: {:?}", event);
        }
    }
}
