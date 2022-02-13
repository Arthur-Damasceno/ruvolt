use serde::Deserialize;

use crate::models::Id;

/// A DM channel.
#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct DirectMessageChannel {
    /// Channel id.
    #[serde(rename = "_id")]
    pub id: Id,
    /// Whether this DM is active.
    pub active: bool,
    /// List of user ids who are participating in this DM.
    pub recipients: Vec<Id>,
    /// Id of the last message in the channel.
    pub last_message_id: Option<Id>,
}
