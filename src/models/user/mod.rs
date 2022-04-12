pub use {badges::*, bot_information::*, flags::*, profile::*, status::*};

mod badges;
mod bot_information;
mod flags;
mod profile;
mod status;

use crate::{
    models::{Attachment, DirectMessageChannel, Id},
    Context, Result,
};

/// A user.
#[derive(Debug, Deserialize, Clone, PartialEq)]
#[non_exhaustive]
pub struct User {
    /// User id.
    #[serde(rename = "_id")]
    pub id: Id,
    /// User username.
    pub username: String,
    /// User avatar.
    pub avatar: Option<Attachment>,
    /// User status.
    pub status: Option<UserStatus>,
    /// User badges.
    #[serde(default)]
    pub badges: Badges,
    /// User flags.
    #[serde(default)]
    pub flags: UserFlags,
    /// User is online.
    #[serde(default)]
    pub online: bool,
    /// The bot information.
    pub bot: Option<BotInformation>,
}

impl User {
    /// Get a user from the cache or API.
    pub async fn fetch(cx: &Context, id: &Id) -> Result<Self> {
        #[cfg(feature = "cache")]
        if let Some(user) = cx.cache.user(id).await {
            return Ok(user);
        }

        cx.http_client.get(format!("users/{}", id)).await
    }

    /// Returns if the user is a bot.
    pub fn is_bot(&self) -> bool {
        self.bot.is_some()
    }

    /// Get the user profile from the API.
    pub async fn profile(&self, cx: &Context) -> Result<UserProfile> {
        cx.http_client
            .get(format!("users/{}/profile", self.id))
            .await
    }

    /// Open a DM with the user.
    pub async fn dm(&self, cx: &Context) -> Result<DirectMessageChannel> {
        DirectMessageChannel::open(cx, &self.id).await
    }
}
