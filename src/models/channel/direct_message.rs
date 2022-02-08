use serde::Deserialize;

use crate::{
    models::{Channel, Id, Message},
    Context, Result,
};

/// A DM channel.
#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct DirectMessageChannel {
    /// Channel id.
    pub id: Id,
    /// Whether this DM is active.
    pub active: bool,
    /// List of user ids who are participating in this DM.
    pub recipients: Vec<Id>,
    /// Id of the last message in the channel.
    pub last_message_id: Option<Id>,
}

impl DirectMessageChannel {
    /// Open a DM with another user.
    pub async fn open(cx: &Context, user_id: &Id) -> Result<Self> {
        let path = format!("users/{}/dm", user_id);
        let dm = cx.http_client.get(&path).await?;

        Ok(dm)
    }

    /// Get the last message in the channel from the API.
    pub async fn fetch_last_msg(&self, cx: &Context) -> Result<Option<Message>> {
        match self.last_message_id {
            Some(ref msg_id) => {
                let msg = Message::fetch(cx, &self.id, msg_id).await?;

                Ok(Some(msg))
            }
            None => Ok(None),
        }
    }

    /// Send a message in the channel.
    pub async fn send(&self, cx: &Context, content: &str) -> Result<Message> {
        Channel::send(cx, &self.id, content).await
    }

    /// Close this DM.
    pub async fn close(&self, cx: &Context) -> Result {
        Channel::delete(cx, &self.id).await
    }
}
