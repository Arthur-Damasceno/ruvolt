/// A message has been deleted.
#[derive(Debug, Clone, PartialEq, Deserialize)]
#[non_exhaustive]
pub struct MessageDeleteEvent {
    /// Message id.
    #[serde(rename = "id")]
    pub message_id: String,
    /// Message channel id.
    #[serde(rename = "channel")]
    pub channel_id: String,
}
