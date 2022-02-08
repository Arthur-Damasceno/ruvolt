use serde::Deserialize;

use crate::{
    models::{Attachment, Channel, ChannelPermissionsRaw, Id, Message, User},
    Context, Result,
};

/// A group channel.
#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct GroupChannel {
    /// Channel id.
    #[serde(rename = "_id")]
    pub id: Id,
    /// Group owner id.
    #[serde(rename = "owner")]
    pub owner_id: Id,
    /// Group name.
    pub name: String,
    /// Group description.
    pub description: Option<String>,
    /// Group icon.
    pub icon: Option<Attachment>,
    /// List of user ids who are participating in this group.
    pub recipients: Vec<Id>,
    /// Id of the last message in the channel.
    pub last_message_id: Option<Id>,
    /// Permissions given to group members.
    #[serde(default)]
    pub permissions: ChannelPermissionsRaw,
    /// Group is not safe for work.
    #[serde(default)]
    pub nsfw: bool,
}

impl GroupChannel {
    /// Get the group owner from the API.
    pub async fn fetch_owner(&self, cx: &Context) -> Result<User> {
        User::fetch(cx, &self.owner_id).await
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

    /// Leave the group.
    pub async fn leave(&self, cx: &Context) -> Result {
        Channel::delete(cx, &self.id).await
    }
}
