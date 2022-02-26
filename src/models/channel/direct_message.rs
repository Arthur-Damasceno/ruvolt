use serde::Deserialize;

use crate::{
    builders::CreateMessage,
    models::{Id, Message},
    Context, Result,
};

/// A DM channel.
#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct DirectMessageChannel {
    /// Channel id.
    #[serde(rename = "_id")]
    pub id: Id,
    /// Whether this DM is active.
    pub active: bool,
    /// List of user ids who are participating in this DM.
    pub recipients: Vec<Id>,
    /// Id of the last message in the channel.
    pub last_message_id: Option<Id>,
}

impl DirectMessageChannel {
    /// Send a message in this channel.
    pub async fn send(&self, cx: &Context, builder: impl Into<CreateMessage>) -> Result<Message> {
        Message::create(cx, &self.id, builder.into()).await
    }
}
