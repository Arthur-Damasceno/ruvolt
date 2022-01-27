pub use content::*;

mod content;
mod edited;

use edited::Edited;
use serde::Deserialize;

/// A message.
#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct Message {
    #[serde(rename = "_id")]
    id: String,
    #[serde(default)]
    nonce: Option<String>,
    #[serde(rename = "channel")]
    channel_id: String,
    #[serde(rename = "author")]
    author_id: String,
    content: Content,
    #[serde(default)]
    edited: Option<Edited>,
    #[serde(default)]
    mentions: Option<Vec<String>>,
    #[serde(default)]
    replies: Option<Vec<String>>,
}

impl Message {
    /// Returns the message id.
    pub fn id(&self) -> &str {
        &self.id
    }

    /// Returns the message nonce.
    pub fn nonce(&self) -> Option<&str> {
        match self.nonce {
            Some(ref nonce) => Some(nonce),
            None => None,
        }
    }

    /// Returns the channel id.
    pub fn channel_id(&self) -> &str {
        &self.channel_id
    }

    /// Returns the message author id.
    pub fn author_id(&self) -> &str {
        &self.author_id
    }

    /// Returns the message content.
    pub fn content(&self) -> &Content {
        &self.content
    }

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

    /// Returns the message mentions.
    pub fn mentions(&self) -> Option<&[String]> {
        match self.mentions {
            Some(ref mentions) => Some(mentions),
            None => None,
        }
    }

    /// Returns the message replies.
    pub fn replies(&self) -> Option<&[String]> {
        match self.replies {
            Some(ref replies) => Some(replies),
            None => None,
        }
    }
}
