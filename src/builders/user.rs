use crate::models::{events::UserField, Presence, UserStatus};

/// Builder for edit the current user.
#[derive(Debug, Clone, Default, Serialize)]
pub struct EditUser {
    #[serde(skip_serializing_if = "Option::is_none")]
    status: Option<UserStatus>,
    #[serde(skip_serializing_if = "Option::is_none")]
    profile: Option<EditUserProfile>,
    #[serde(skip_serializing_if = "Option::is_none")]
    avatar: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    remove: Option<UserField>,
}

impl EditUser {
    /// Creates a new builder.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the user status.
    pub fn status(mut self, status: UserStatus) -> Self {
        self.status = Some(status);
        self
    }

    /// Set the user profile.
    pub fn profile(mut self, build: impl Fn(EditUserProfile) -> EditUserProfile) -> Self {
        self.profile = Some(build(EditUserProfile::default()));
        self
    }

    /// Set the avatar.
    pub fn avatar(mut self, id: impl Into<String>) -> Self {
        self.avatar = Some(id.into());
        self
    }

    /// Set a user field to remove.
    pub fn remove(mut self, field: UserField) -> Self {
        self.remove = Some(field);
        self
    }
}

impl From<Presence> for EditUser {
    fn from(presence: Presence) -> Self {
        Self::new().status(UserStatus::presence(presence))
    }
}

impl From<UserStatus> for EditUser {
    fn from(status: UserStatus) -> Self {
        Self::new().status(status)
    }
}

impl From<EditUserProfile> for EditUser {
    fn from(profile: EditUserProfile) -> Self {
        Self {
            profile: Some(profile),
            ..Default::default()
        }
    }
}

/// Builder for edit the current user profile.
#[derive(Debug, Clone, Default, Serialize)]
pub struct EditUserProfile {
    #[serde(skip_serializing_if = "Option::is_none")]
    content: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    background: Option<String>,
}

impl EditUserProfile {
    /// Set the content.
    pub fn content(mut self, content: impl Into<String>) -> Self {
        self.content = Some(content.into());
        self
    }

    /// Set the background.
    pub fn background(mut self, id: impl Into<String>) -> Self {
        self.background = Some(id.into());
        self
    }
}
