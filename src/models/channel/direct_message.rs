/// A DM channel.
#[derive(Debug, Clone, PartialEq, Deserialize)]
#[non_exhaustive]
pub struct DirectMessageChannel {
    /// Channel id.
    #[serde(rename = "_id")]
    pub id: String,
    /// Whether this DM is active.
    pub active: bool,
    /// List of user ids who are participating in this DM.
    pub recipients: [String; 2],
    /// Id of the last message in the channel.
    pub last_message_id: Option<String>,
}
