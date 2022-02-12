use {serde::Deserialize, serde_json::Value as Json};

use crate::models::MemberId;

/// Specifies a field to remove on server member update.
#[derive(Debug, Deserialize, Clone, Copy, PartialEq)]
pub enum RemoveServerMemberField {
    /// Server member nickname.
    Nickname,
    /// Server member avatar.
    Avatar,
}

/// A server member details were updated.
#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct ServerMemberUpdateEvent {
    /// Member id.
    #[serde(rename = "id")]
    pub member_id: MemberId,
    /// A partial server member object.
    pub data: Json,
    /// A specified field to remove on server member update.
    pub clear: Option<RemoveServerMemberField>,
}
