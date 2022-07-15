use crate::models::Attachment;

/// A server member id.
#[derive(Debug, Clone, PartialEq, Hash, Eq, Deserialize)]
#[non_exhaustive]
pub struct MemberId {
    /// Server id.
    #[serde(rename = "server")]
    pub server_id: String,
    /// User id.
    #[serde(rename = "user")]
    pub user_id: String,
}

impl MemberId {
    /// Create a [MemberId].
    pub fn new(server_id: impl Into<String>, user_id: impl Into<String>) -> Self {
        Self {
            server_id: server_id.into(),
            user_id: user_id.into(),
        }
    }
}

/// A server member.
#[derive(Debug, Clone, PartialEq, Deserialize)]
#[non_exhaustive]
pub struct Member {
    /// Member id.
    #[serde(rename = "_id")]
    pub id: MemberId,
    /// Member nickname.
    pub nickname: Option<String>,
    /// Member avatar.
    pub avatar: Option<Attachment>,
    /// Member roles ids.
    #[serde(default)]
    pub roles: Vec<String>,
}

/// A server member ban.
#[derive(Debug, Clone, Deserialize)]
pub struct ServerBan {
    /// Member id.
    #[serde(rename = "_id")]
    pub member_id: MemberId,
    /// Reason for the ban.
    pub reason: Option<String>,
}
