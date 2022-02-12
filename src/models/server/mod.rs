pub use {category::*, member::*, system_message_channels::*};

mod category;
mod member;
mod system_message_channels;

use serde::Deserialize;

use crate::{
    models::{Id, User},
    Context, Result,
};

/// A server.
#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct Server {
    /// Server id.
    #[serde(rename = "_id")]
    pub id: Id,
    /// Server owner id.
    #[serde(rename = "owner")]
    pub owner_id: Id,
    /// Server name.
    pub name: String,
    /// Server description.
    pub description: Option<String>,
    /// Server channels ids.
    pub channels: Vec<Id>,
    /// Server categories.
    #[serde(default)]
    pub categories: Vec<Category>,
    /// Server system message channels.
    #[serde(rename = "system_messages")]
    pub system_message_channels: Option<SystemMessageChannels>,
    /// Server flags.
    pub flags: Option<u32>,
    /// Server is not safe for work.
    #[serde(default)]
    pub nsfw: bool,
}

impl Server {
    /// Get a server from the API.
    pub async fn fetch(cx: &Context, id: &Id) -> Result<Self> {
        let path = format!("servers/{}", id);
        let server = cx.http_client.get(&path).await?;

        Ok(server)
    }

    /// Get the server owner from the API.
    pub async fn fetch_owner(&self, cx: &Context) -> Result<User> {
        User::fetch(cx, &self.owner_id).await
    }
}
