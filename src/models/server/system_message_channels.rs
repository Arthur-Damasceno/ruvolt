use crate::models::Id;

/// Server system message channels.
#[derive(Debug, Clone, PartialEq, Default, Deserialize, Serialize)]
#[non_exhaustive]
pub struct SystemMessageChannels {
    /// User joined channel id.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_joined: Option<Id>,
    /// User left channel id.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_left: Option<Id>,
    /// User kicked channel id.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_kicked: Option<Id>,
    /// User banned channel id.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_banned: Option<Id>,
}

impl SystemMessageChannels {
    /// Set the user joined channel.
    pub fn user_joined(mut self, id: &Id) -> Self {
        self.user_joined = Some(id.clone());
        self
    }

    /// Set the user left channel.
    pub fn user_left(mut self, id: &Id) -> Self {
        self.user_left = Some(id.clone());
        self
    }

    /// Set the user kicked channel.
    pub fn user_kicked(mut self, id: &Id) -> Self {
        self.user_kicked = Some(id.clone());
        self
    }

    /// Set the user banned channel.
    pub fn user_banned(mut self, id: &Id) -> Self {
        self.user_banned = Some(id.clone());
        self
    }
}
