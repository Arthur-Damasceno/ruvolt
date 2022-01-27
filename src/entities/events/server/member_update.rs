use {serde::Deserialize, serde_json::Value as Json};

use crate::entities::ServerToClientEvent;

/// Specifies a field to remove on server member update.
#[derive(Debug, Deserialize, Clone, Copy, PartialEq)]
pub enum RemoveServerMemberField {
    /// Server member nickname.
    Nickname,
    /// Server member avatar.
    Avatar,
}

/// A server member details were updated.
#[derive(Debug)]
pub struct ServerMemberUpdateEvent {
    id: String,
    server_id: String,
    data: Json,
    clear: Option<RemoveServerMemberField>,
}

impl ServerMemberUpdateEvent {
    /// Returns the server member id.
    pub fn id(&self) -> &str {
        &self.id
    }

    /// Returns the server id.
    pub fn server_id(&self) -> &str {
        &self.server_id
    }

    /// Returns a partial server member object.
    pub fn data(&self) -> &Json {
        &self.data
    }

    /// Returns a specified field to remove on server member update.
    pub fn clear(&self) -> Option<RemoveServerMemberField> {
        self.clear
    }
}

impl From<ServerToClientEvent> for ServerMemberUpdateEvent {
    fn from(event: ServerToClientEvent) -> Self {
        if let ServerToClientEvent::ServerMemberUpdate { id, data, clear } = event {
            Self {
                id: id.user,
                server_id: id.server,
                data,
                clear,
            }
        } else {
            panic!("An incorrect event was provided: {:?}", event);
        }
    }
}
