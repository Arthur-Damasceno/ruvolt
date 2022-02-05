use {serde::Deserialize, serde_json::Value as Json};

use {super::ServerToClientEvent, crate::entities::Id};

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
    /// User id.
    pub id: Id,
    /// A partial user object.
    pub data: Json,
    /// A specified field to remove on user update.
    pub clear: Option<RemoveUserField>,
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
