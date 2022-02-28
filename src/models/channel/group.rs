use serde::Deserialize;

use crate::{
    builders::{CreateMessage, EditChannel},
    models::{Attachment, Channel, Id, Message},
    Context, Result,
};

/// A group channel.
#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct GroupChannel {
    /// Group id.
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
    /// Id of last message in the group.
    pub last_message_id: Option<Id>,
    /// Group is not safe for work.
    #[serde(default)]
    pub nsfw: bool,
}

impl GroupChannel {
    /// Send a message in the group.
    pub async fn send(&self, cx: &Context, builder: impl Into<CreateMessage>) -> Result<Message> {
        Message::create(cx, &self.id, builder.into()).await
    }

    /// Edit the group.
    pub async fn edit(&self, cx: &Context, builder: EditChannel) -> Result {
        Channel::edit(cx, &self.id, builder).await
    }

    /// Leave the group.
    pub async fn leave(&self, cx: &Context) -> Result {
        Channel::delete(cx, &self.id).await
    }
}
