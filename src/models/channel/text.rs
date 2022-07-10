use crate::models::Attachment;

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
