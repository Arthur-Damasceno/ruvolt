use serde::Deserialize;

use crate::models::Id;

/// A user has joined the server.
#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct ServerMemberJoinEvent {
    /// Server id.
    #[serde(rename = "id")]
    pub server_id: Id,
    /// User id.
    #[serde(rename = "user")]
    pub user_id: Id,
}
