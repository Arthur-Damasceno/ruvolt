pub use {badges::*, flags::*, profile::*, status::*};

mod badges;
mod bot_info;
mod flags;
mod profile;
mod status;

use serde::Deserialize;

use {
    crate::{
        models::{Attachment, Id},
        Context, Result,
    },
    bot_info::BotInfo,
};

/// A user.
#[derive(Debug, Deserialize, Clone, PartialEq)]
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
    bot: Option<BotInfo>,
}

impl User {
    /// Get a user from the API.
    pub async fn fetch(cx: &Context, id: &Id) -> Result<Self> {
        let path = format!("users/{}", id);
        let user = cx.http_client.get(&path).await?;

        Ok(user)
    }

    /// Returns the owner id of the bot.
    pub fn owner_id(&self) -> Option<&Id> {
        match self.bot {
            Some(BotInfo { ref owner_id }) => Some(owner_id),
            None => None,
        }
    }

    /// Returns if the user is a bot.
    pub fn is_bot(&self) -> bool {
        self.bot.is_some()
    }

    /// Get the user profile from the API.
    pub async fn profile(&self, cx: &Context) -> Result<UserProfile> {
        let path = format!("users/{}/profile", self.id);
        let profile = cx.http_client.get(&path).await?;

        Ok(profile)
    }
}
