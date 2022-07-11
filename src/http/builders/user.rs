use crate::models::{events::UserField, UserStatus};

/// A builder to edit the current user.
#[derive(Debug, Clone, Default, Serialize)]
pub struct EditUser {
    #[serde(skip_serializing_if = "Option::is_none")]
    status: Option<UserStatus>,
    #[serde(skip_serializing_if = "Option::is_none")]
    profile: Option<EditUserProfile>,
    #[serde(skip_serializing_if = "Option::is_none")]
    avatar: Option<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    remove: Vec<UserField>,
}

impl EditUser {
    /// Set the user's active status.
    pub fn status(mut self, status: UserStatus) -> Self {
        self.status = Some(status);
        self
    }

    /// Set the user profile data.
    pub fn profile(mut self, profile: EditUserProfile) -> Self {
        self.profile = Some(profile);
        self
    }

    /// Set the attachment id for avatar.
    pub fn avatar(mut self, avatar: impl Into<String>) -> Self {
        self.avatar = Some(avatar.into());
        self
    }

    /// Set the fields to remove from the user.
    pub fn remove(mut self, field: UserField) -> Self {
        self.remove.push(field);
        self
    }
}

/// A builder to edit the current user’s profile.
#[derive(Debug, Clone, Default, Serialize)]
pub struct EditUserProfile {
    #[serde(skip_serializing_if = "Option::is_none")]
    content: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    background: Option<String>,
}

impl EditUserProfile {
    /// Set the description of the user's profile.
    pub fn content(mut self, content: impl Into<String>) -> Self {
        self.content = Some(content.into());
        self
    }

    /// Set the attachment id for background.
    pub fn background(mut self, background: impl Into<String>) -> Self {
        self.background = Some(background.into());
        self
    }
}
