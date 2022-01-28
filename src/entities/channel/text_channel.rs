use serde::Deserialize;

/// A text channel.
#[derive(Debug, Deserialize, Clone, PartialEq)]
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
    #[serde(default)]
    pub description: Option<String>,
    /// Id of last message in the channel.
    #[serde(default)]
    pub last_message_id: Option<String>,
    /// Channel is not safe for work.
    #[serde(default)]
    pub nsfw: bool,
}
