use serde::Deserialize;

use crate::models::Id;

/// A server role has been deleted.
#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct ServerRoleDeleteEvent {
    /// Server id.
    #[serde(rename = "id")]
    pub server_id: Id,
    /// Server role id.
    pub role_id: Id,
}
