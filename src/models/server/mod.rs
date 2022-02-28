pub use {category::*, flags::*, member::*, system_message_channels::*};

mod category;
mod flags;
mod member;
mod system_message_channels;

use serde::Deserialize;

use crate::{
    builders::{CreateChannel, EditServer},
    models::{Attachment, Channel, Id},
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
    #[serde(default)]
    pub system_messages: SystemMessageChannels,
    /// Server icon.
    pub icon: Option<Attachment>,
    /// Server banner.
    pub banner: Option<Attachment>,
    /// Server flags.
    #[serde(default)]
    pub flags: ServerFlags,
    /// Server is not safe for work.
    #[serde(default)]
    pub nsfw: bool,
}

impl Server {
    /// Get a server from the API.
    pub async fn fetch(cx: &Context, id: &Id) -> Result<Self> {
        cx.http_client.get(format!("servers/{}", id)).await
    }

    /// Edit the server.
    pub async fn edit(&self, cx: &Context, builder: EditServer) -> Result {
        cx.http_client
            .patch(format!("servers/{}", self.id), builder)
            .await
    }

    /// Create a channel in the server.
    pub async fn create_channel(&self, cx: &Context, builder: CreateChannel) -> Result<Channel> {
        cx.http_client
            .post(format!("servers/{}/channels", self.id), builder)
            .await
    }

    /// Leave the server
    pub async fn leave(&self, cx: &Context) -> Result {
        cx.http_client.delete(format!("servers/{}", self.id)).await
    }
}
