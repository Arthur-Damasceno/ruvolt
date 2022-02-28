use serde::{Deserialize, Serialize};

use crate::models::{Attachment, Id, MemberId};

/// Specifies a field to remove on server member update.
#[derive(Debug, Clone, Copy, PartialEq, Deserialize, Serialize)]
pub enum MemberField {
    /// Server member nickname.
    Nickname,
    /// Server member avatar.
    Avatar,
}

/// A server member details were updated.
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct ServerMemberUpdateEvent {
    /// Member id.
    #[serde(rename = "id")]
    pub member_id: MemberId,
    /// A partial server member.
    pub data: PartialMember,
    /// A specified field to remove on server member update.
    pub clear: Option<MemberField>,
}

/// A partial server member.
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct PartialMember {
    /// Member nickname.
    pub nickname: Option<String>,
    /// Member avatar.
    pub avatar: Option<Attachment>,
    /// Member roles.
    #[serde(default)]
    pub roles: Vec<Id>,
}
