use serde::Deserialize;

/// Server system message channels.
#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct SystemMessageChannels {
    /// User joined channel id.
    pub user_joined: Option<String>,
    /// User left channel id.
    pub user_left: Option<String>,
    /// User kicked channel id.
    pub user_kicked: Option<String>,
    /// User banned channel id.
    pub user_banned: Option<String>,
}
