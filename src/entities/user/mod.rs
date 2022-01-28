pub use status::*;

mod bot_info;
mod status;

use serde::Deserialize;

use bot_info::BotInfo;

/// A user.
#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct User {
    /// User id.
    #[serde(rename = "_id")]
    pub id: String,
    /// User username.
    pub username: String,
    /// User status.
    #[serde(default)]
    pub status: Option<UserStatus>,
    /// User badges.
    #[serde(default)]
    pub badges: Option<u32>,
    /// User flags.
    #[serde(default)]
    pub flags: Option<u32>,
    #[serde(default)]
    bot: Option<BotInfo>,
    /// User is online.
    #[serde(default)]
    pub online: bool,
}

impl User {
    /// Returns the owner id of the bot.
    pub fn owner_id(&self) -> Option<&str> {
        match self.bot {
            Some(BotInfo { ref owner_id }) => Some(owner_id),
            None => None,
        }
    }

    /// Returns if the user is a bot.
    pub fn is_bot(&self) -> bool {
        self.bot.is_some()
    }
}
