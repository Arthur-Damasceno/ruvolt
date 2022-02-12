use serde::Deserialize;

use crate::models::Id;

/// Server system message channels.
#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct SystemMessageChannels {
    /// User joined channel id.
    pub user_joined: Option<Id>,
    /// User left channel id.
    pub user_left: Option<Id>,
    /// User kicked channel id.
    pub user_kicked: Option<Id>,
    /// User banned channel id.
    pub user_banned: Option<Id>,
}
