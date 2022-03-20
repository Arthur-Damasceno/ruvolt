use serde::Deserialize;

use crate::models::Attachment;

/// User profile.
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct UserProfile {
    /// Profile content.
    pub content: Option<String>,
    /// Profile background.
    pub background: Option<Attachment>,
}
