use crate::{builders::CreateMessage, models::Message, Context, Result};

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

impl DirectMessageChannel {
    /// Open a DM with another user.
    pub async fn open(_cx: &Context, _user_id: &str) -> Result<Self> {
        todo!()
    }

    /// Send a message in this channel.
    pub async fn send(&self, _cx: &Context, _builder: impl Into<CreateMessage>) -> Result<Message> {
        todo!()
    }

    /// Close the DM.
    pub async fn close(&self, _cx: &Context) -> Result {
        todo!()
    }
}
