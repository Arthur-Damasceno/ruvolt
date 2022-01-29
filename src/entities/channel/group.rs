use serde::Deserialize;

use crate::{entities::User, Context, Result};

/// A group channel.
#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct GroupChannel {
    /// Channel id.
    #[serde(rename = "_id")]
    pub id: String,
    /// Channel owner id.
    #[serde(rename = "owner")]
    pub owner_id: String,
    /// Channel name.
    pub name: String,
    /// Channel description.
    pub description: Option<String>,
    /// Id of last message in the channel.
    pub last_message_id: Option<String>,
    /// Channel is not safe for work.
    #[serde(default)]
    pub nsfw: bool,
}

impl GroupChannel {
    /// Get the group owner from the API.
    pub async fn fetch_owner(&self, cx: &Context) -> Result<User> {
        User::fetch(cx, &self.owner_id).await
    }
}
