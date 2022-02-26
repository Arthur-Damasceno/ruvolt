use serde::{Deserialize, Serialize};

/// User status.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct UserStatus {
    /// User status text.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    /// User presence.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub presence: Option<Presence>,
}

impl UserStatus {
    /// Creates a new [UserStatus].
    pub fn new(text: impl Into<String>, presence: Presence) -> Self {
        Self {
            text: Some(text.into()),
            presence: Some(presence),
        }
    }

    /// Creates a new [UserStatus] with only the status text.
    pub fn text(text: impl Into<String>) -> Self {
        Self {
            text: Some(text.into()),
            presence: None,
        }
    }

    /// Creates a new [UserStatus] with only the presence.
    pub fn presence(presence: Presence) -> Self {
        Self {
            text: None,
            presence: Some(presence),
        }
    }
}

/// User presence.
#[derive(Debug, Clone, Copy, PartialEq, Deserialize, Serialize)]
pub enum Presence {
    /// `Online` presence.
    Online,
    /// `Idle` presence.
    Idle,
    /// `Busy` presence.
    Busy,
    /// `Invisible` presence.
    Invisible,
}
