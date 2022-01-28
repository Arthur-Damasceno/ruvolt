pub use content::*;

mod content;
mod edited;

use edited::Edited;
use serde::Deserialize;

/// A message.
#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct Message {
    /// Message id.
    #[serde(rename = "_id")]
    pub id: String,
    /// Message nonce.
    #[serde(default)]
    pub nonce: Option<String>,
    /// Message channel id.
    #[serde(rename = "channel")]
    pub channel_id: String,
    /// Message author id.
    #[serde(rename = "author")]
    pub author_id: String,
    /// Message content.
    pub content: Content,
    #[serde(default)]
    edited: Option<Edited>,
    /// Message mentions.
    #[serde(default)]
    pub mentions: Vec<String>,
    /// Message replies.
    #[serde(default)]
    pub replies: Vec<String>,
}

impl Message {
    /// Returns the message edit date.
    pub fn edited(&self) -> Option<&str> {
        match self.edited {
            Some(Edited { ref date }) => Some(date),
            None => None,
        }
    }

    /// Returns whether the message has been edited.
    pub fn is_edited(&self) -> bool {
        self.edited.is_some()
    }
}
