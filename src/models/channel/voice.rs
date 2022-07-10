use crate::models::Attachment;

/// A voice channel.
#[derive(Debug, Clone, PartialEq, Deserialize)]
#[non_exhaustive]
pub struct VoiceChannel {
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
    /// Channel is not safe for work.
    #[serde(default)]
    pub nsfw: bool,
}
