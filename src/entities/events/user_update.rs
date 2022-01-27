use {serde::Deserialize, serde_json::Value as Json};

use crate::entities::ServerToClientEvent;

/// Specifies a field to remove on user update.
#[derive(Debug, Deserialize, Clone, Copy, PartialEq)]
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
#[derive(Debug)]
pub struct UserUpdateEvent {
    id: String,
    data: Json,
    clear: Option<RemoveUserField>,
}

impl UserUpdateEvent {
    /// Returns the user id.
    pub fn id(&self) -> &str {
        &self.id
    }

    /// Returns a partial user object.
    pub fn data(&self) -> &Json {
        &self.data
    }

    /// Returns a specified field to remove on user update.
    pub fn clear(&self) -> Option<RemoveUserField> {
        self.clear
    }
}

impl From<ServerToClientEvent> for UserUpdateEvent {
    fn from(event: ServerToClientEvent) -> Self {
        if let ServerToClientEvent::UserUpdate { id, data, clear } = event {
            Self { id, data, clear }
        } else {
            panic!("An incorrect event was provided: {:?}", event);
        }
    }
}
