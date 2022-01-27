use {serde::Deserialize, serde_json::Value as Json};

use crate::entities::ServerToClientEvent;

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
    id: String,
    data: Json,
    clear: Option<RemoveChannelField>,
}

impl ChannelUpdateEvent {
    /// Returns the channel id.
    pub fn id(&self) -> &str {
        &self.id
    }

    /// Returns a partial channel object.
    pub fn data(&self) -> &Json {
        &self.data
    }

    /// Returns a specified field to remove on channel update.
    pub fn clear(&self) -> Option<RemoveChannelField> {
        self.clear
    }
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
