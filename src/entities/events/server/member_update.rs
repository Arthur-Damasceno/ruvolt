use {serde::Deserialize, serde_json::Value as Json};

use super::super::{ServerMemberUpdateId, ServerToClientEvent};

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
    /// Server id.
    pub id: String,
    /// Server member id.
    pub user_id: String,
    /// A partial server member object.
    pub data: Json,
    /// A specified field to remove on server member update.
    pub clear: Option<RemoveServerMemberField>,
}

impl From<ServerToClientEvent> for ServerMemberUpdateEvent {
    fn from(event: ServerToClientEvent) -> Self {
        if let ServerToClientEvent::ServerMemberUpdate { id, data, clear } = event {
            let ServerMemberUpdateId { server_id, user_id } = id;

            Self {
                id: server_id,
                user_id,
                data,
                clear,
            }
        } else {
            panic!("An incorrect event was provided: {:?}", event);
        }
    }
}
