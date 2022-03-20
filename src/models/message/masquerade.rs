use serde::{Deserialize, Serialize};

/// Masquerade displayed for a message.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct Masquerade {
    /// Displayed name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Displayed avatar.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avatar: Option<String>,
}

impl Masquerade {
    /// Creates a new [Masquerade].
    pub fn new(name: impl Into<String>, avatar: impl Into<String>) -> Self {
        Self {
            name: Some(name.into()),
            avatar: Some(avatar.into()),
        }
    }

    /// Creates a new [Masquerade] with only the name.
    pub fn name(name: impl Into<String>) -> Self {
        Self {
            name: Some(name.into()),
            avatar: None,
        }
    }

    /// Creates a new [Masquerade] with only the avatar.
    pub fn avatar(avatar: impl Into<String>) -> Self {
        Self {
            name: None,
            avatar: Some(avatar.into()),
        }
    }
}
