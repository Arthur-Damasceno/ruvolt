/// You have acknowledged new messages in the channel up to the message id.
#[derive(Debug, Clone, PartialEq, Deserialize)]
#[non_exhaustive]
pub struct ChannelAckEvent {
    /// Channel id.
    #[serde(rename = "id")]
    pub channel_id: String,
    /// User id.
    #[serde(rename = "user")]
    pub user_id: String,
    /// Message id.
    pub message_id: String,
}
