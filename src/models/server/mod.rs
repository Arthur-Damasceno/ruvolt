pub use {category::*, flags::*, member::*, role::*, system_message_channels::*};

mod category;
mod flags;
mod member;
mod role;
mod system_message_channels;

use {serde::Deserialize, std::collections::HashMap};

use crate::{
    models::{Attachment, Id, User},
    Context, Result,
};

/// Tuple consisting of server and channel permissions in that order.
pub type PermissionTuple = (u32, u32);

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
    pub system_messages: Option<SystemMessageChannels>,
    /// Server roles.
    #[serde(default)]
    pub roles: HashMap<Id, Role>,
    /// Default permissions for all members
    pub default_permissions: PermissionTuple,
    /// Server icon.
    pub icon: Option<Attachment>,
    /// Server banner.
    pub banner: Option<Attachment>,
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

    /// Leave from the server.
    pub async fn leave(&self, cx: &Context) -> Result {
        let path = format!("servers/{}", self.id);
        cx.http_client.delete(&path).await?;

        Ok(())
    }
}
