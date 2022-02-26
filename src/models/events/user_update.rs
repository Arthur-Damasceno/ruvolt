use {
    serde::{Deserialize, Serialize},
    serde_json::Value as Json,
};

use crate::models::Id;

/// Specifies a field to remove on user update.
#[derive(Debug, Clone, Copy, PartialEq, Deserialize, Serialize)]
pub enum RemoveUserField {
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
    /// A partial user object.
    pub data: Json,
    /// A specified field to remove on user update.
    pub clear: Option<RemoveUserField>,
}
