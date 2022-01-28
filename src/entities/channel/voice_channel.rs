use serde::Deserialize;

/// A voice channel.
#[derive(Debug, Deserialize, Clone, PartialEq)]
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
    #[serde(default)]
    pub description: Option<String>,
    /// Channel is not safe for work.
    #[serde(default)]
    pub nsfw: bool,
}
