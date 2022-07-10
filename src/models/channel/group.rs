use crate::models::Attachment;

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
