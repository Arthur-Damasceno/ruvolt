use serde::Deserialize;

/// User status object.
#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct UserStatus {
    /// User status text.
    pub text: Option<String>,
    /// User presence.
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
