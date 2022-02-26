use serde::Deserialize;

use crate::{
    builders::CreateMessage,
    models::{Attachment, Id, Message},
    Context, Result,
};

/// A text channel.
#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct TextChannel {
    /// Channel id.
    #[serde(rename = "_id")]
    pub id: Id,
    /// Channel server id.
    #[serde(rename = "server")]
    pub server_id: Id,
    /// Channel name.
    pub name: String,
    /// Channel description.
    pub description: Option<String>,
    /// Channel icon.
    pub icon: Option<Attachment>,
    /// Id of last message in the channel.
    pub last_message_id: Option<Id>,
    /// Channel is not safe for work.
    #[serde(default)]
    pub nsfw: bool,
}

impl TextChannel {
    /// Send a message in this channel.
    pub async fn send(&self, cx: &Context, builder: impl Into<CreateMessage>) -> Result<Message> {
        Message::create(cx, &self.id, builder.into()).await
    }
}
