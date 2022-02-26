use serde::Serialize;

use crate::models::Id;

/// Builder for create a message.
#[derive(Debug, Clone, Serialize)]
pub struct CreateMessage {
    content: String,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    attachments: Vec<Id>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    replies: Vec<Reply>,
}

#[derive(Debug, Clone, Serialize)]
struct Reply {
    id: Id,
    mention: bool,
}

impl CreateMessage {
    /// Creates a new builder.
    pub fn new(content: impl Into<String>) -> Self {
        Self {
            content: content.into(),
            attachments: Vec::new(),
            replies: Vec::new(),
        }
    }

    /// Set a attachment to include in the message.
    pub fn attachment(mut self, id: &Id) -> Self {
        self.attachments.push(id.clone());
        self
    }

    /// Set a message to reply to.
    pub fn reply(mut self, id: &Id, mention: bool) -> Self {
        self.replies.push(Reply {
            id: id.clone(),
            mention,
        });
        self
    }
}

impl<T: Into<String>> From<T> for CreateMessage {
    fn from(content: T) -> Self {
        Self::new(content)
    }
}
