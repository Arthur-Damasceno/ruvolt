use serde::{Deserialize, Serialize};

use crate::models::{Id, UserProfile, UserStatus};

/// Specifies a field to remove on user update.
#[derive(Debug, Clone, Copy, PartialEq, Deserialize, Serialize)]
pub enum UserField {
    /// User profile content.
    ProfileContent,
    /// User profile background.
    ProfileBackground,
    /// User status text.
    StatusText,
    /// User avatar.
    Avatar,
}

/// A user has been updated.
#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct UserUpdateEvent {
    /// User id.
    #[serde(rename = "id")]
    pub user_id: Id,
    /// A partial user.
    pub data: PartialUser,
    /// A specified field to remove on user update.
    pub clear: Option<UserField>,
}

/// A partial user
#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct PartialUser {
    /// User status.
    pub status: Option<UserStatus>,
    /// User profile.
    pub profile: Option<UserProfile>,
    /// User avatar.
    pub avatar: Option<Id>,
}
