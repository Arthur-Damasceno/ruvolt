use serde::Deserialize;

/// User status object.
#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct UserStatus {
    /// User status text.
    #[serde(default)]
    pub text: Option<String>,
    /// User presence.
    #[serde(default)]
    pub presence: Option<Presence>,
}

/// User presence.
#[allow(missing_docs)]
#[derive(Debug, Deserialize, Clone, Copy, PartialEq)]
pub enum Presence {
    Online,
    Idle,
    Busy,
    Invisible,
}
