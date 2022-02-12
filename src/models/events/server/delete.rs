use serde::Deserialize;

use crate::models::Id;

/// A server has been deleted.
#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct ServerDeleteEvent {
    /// Server id.
    #[serde(rename = "id")]
    pub server_id: Id,
}
