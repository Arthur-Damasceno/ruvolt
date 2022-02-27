use {
    serde::{Deserialize, Serialize},
    serde_json::Value as Json,
};

use crate::models::Id;

/// Specifies a field to remove on server update.
#[derive(Debug, Clone, Copy, PartialEq, Deserialize, Serialize)]
pub enum RemoveServerField {
    /// Server icon.
    Icon,
    /// Server banner.
    Banner,
    /// Server description.
    Description,
}

/// A server details were updated.
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct ServerUpdateEvent {
    /// Server id.
    #[serde(rename = "id")]
    pub server_id: Id,
    /// A partial server object.
    pub data: Json,
    /// A specified field to remove on server update.
    pub clear: Option<RemoveServerField>,
}
