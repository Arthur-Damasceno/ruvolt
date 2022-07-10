use crate::{
    builders::{CreateMessage, EditChannel},
    models::{Attachment, Message, User},
    Context, Result,
};

/// A group channel.
#[derive(Debug, Clone, PartialEq, Deserialize)]
#[non_exhaustive]
pub struct GroupChannel {
    /// Group id.
    #[serde(rename = "_id")]
    pub id: String,
    /// Group owner id.
    #[serde(rename = "owner")]
    pub owner_id: String,
    /// Group name.
    pub name: String,
    /// Group description.
    pub description: Option<String>,
    /// List of user ids who are participating in this group.
    pub recipients: Vec<String>,
    /// Group icon.
    pub icon: Option<Attachment>,
    /// Id of last message in the group.
    pub last_message_id: Option<String>,
    /// Group is not safe for work.
    #[serde(default)]
    pub nsfw: bool,
}

impl GroupChannel {
    /// Fetch users who are part of the group.
    pub async fn members(&self, _cx: &Context) -> Result<Vec<User>> {
        todo!()
    }

    /// Send a message in the group.
    pub async fn send(&self, _cx: &Context, _builder: impl Into<CreateMessage>) -> Result<Message> {
        todo!()
    }

    /// Edit the group.
    pub async fn edit(&self, _cx: &Context, _builder: EditChannel) -> Result {
        todo!()
    }

    /// Leave the group.
    pub async fn leave(&self, _cx: &Context) -> Result {
        todo!()
    }
}
