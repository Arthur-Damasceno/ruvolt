use crate::{
    builders::{CreateMessage, EditChannel},
    models::{Attachment, Message},
    Context, Result,
};

/// A text channel.
#[derive(Debug, Clone, PartialEq, Deserialize)]
#[non_exhaustive]
pub struct TextChannel {
    /// Channel id.
    #[serde(rename = "_id")]
    pub id: String,
    /// Channel server id.
    #[serde(rename = "server")]
    pub server_id: String,
    /// Channel name.
    pub name: String,
    /// Channel description.
    pub description: Option<String>,
    /// Channel icon.
    pub icon: Option<Attachment>,
    /// Id of last message in the channel.
    pub last_message_id: Option<String>,
    /// Channel is not safe for work.
    #[serde(default)]
    pub nsfw: bool,
}

impl TextChannel {
    /// Send a message in this channel.
    pub async fn send(&self, _cx: &Context, _builder: impl Into<CreateMessage>) -> Result<Message> {
        todo!()
    }

    /// Edit the channel.
    pub async fn edit(&self, _cx: &Context, _builder: EditChannel) -> Result {
        todo!()
    }

    /// Delete the channel.
    pub async fn delete(&self, _cx: &Context) -> Result {
        todo!()
    }
}
