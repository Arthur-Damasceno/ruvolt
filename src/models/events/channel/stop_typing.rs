/// A user has stopped typing in a channel.
#[derive(Debug, Clone, PartialEq, Deserialize)]
#[non_exhaustive]
pub struct ChannelStopTypingEvent {
    /// Channel id.
    #[serde(rename = "id")]
    pub channel_id: String,
    /// User id.
    #[serde(rename = "user")]
    pub user_id: String,
}
