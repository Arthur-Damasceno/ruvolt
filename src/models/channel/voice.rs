use crate::{
    builders::EditChannel,
    models::{Attachment, Channel, Id},
    Context, Result,
};

/// A voice channel.
#[derive(Debug, Deserialize, Clone, PartialEq)]
#[non_exhaustive]
pub struct VoiceChannel {
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
    /// Channel is not safe for work.
    #[serde(default)]
    pub nsfw: bool,
}

impl VoiceChannel {
    /// Edit the channel.
    pub async fn edit(&self, cx: &Context, builder: EditChannel) -> Result {
        Channel::edit(cx, &self.id, builder).await
    }

    /// Delete the channel.
    pub async fn delete(&self, cx: &Context) -> Result {
        Channel::delete(cx, &self.id).await
    }
}
