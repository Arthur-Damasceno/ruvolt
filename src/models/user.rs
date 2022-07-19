//! User-related models.

use crate::{
    models::{Attachment, Channel},
    Context, Result,
};

/// A user.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct User {
    /// Unique id.
    #[serde(rename = "_id")]
    pub id: String,
    /// Username.
    pub username: String,
    /// Avatar attachment.
    pub avatar: Option<Attachment>,
    /// User's current status.
    pub status: Option<UserStatus>,
    /// Badges.
    #[serde(default)]
    pub badges: Badges,
    /// Flags.
    #[serde(default)]
    pub flags: UserFlags,
    /// Whether this user is privileged.
    #[serde(default)]
    pub privileged: bool,
    /// Current user's relationship with this user.
    #[serde(default)]
    pub relationship: RelationshipStatus,
    /// Whether this user is currently online.
    #[serde(default)]
    pub online: bool,
    /// The bot information.
    pub bot: Option<BotInformation>,
}

impl User {
    /// Open a DM with the user.
    ///
    /// If the user is yours, a saved message channel is returned.
    pub async fn open_direct_message(&self, ctx: &Context) -> Result<Channel> {
        ctx.http.open_direct_message(&self.id).await
    }

    /// Fetch the user profile.
    pub async fn profile(&self, ctx: &Context) -> Result<UserProfile> {
        ctx.http.user_profile(&self.id).await
    }

    /// Fetch a list of mutual friends and servers with the user.
    ///
    /// **Note**: The return type consists of a tuple with the ids of users and servers, respectively.
    pub async fn mutual(&self, ctx: &Context) -> Result<(Vec<String>, Vec<String>)> {
        ctx.http.mutual(&self.id).await
    }

    /// Send a friend request to the user.
    pub async fn add_friend(&self, ctx: &Context) -> Result {
        ctx.http.add_friend(&self.username).await
    }

    /// Accept the user's friend request.
    pub async fn accept_friend(&self, ctx: &Context) -> Result {
        ctx.http.accept_friend(&self.id).await
    }

    /// Denies the user's friend's request or removes the friend.
    pub async fn remove_friend(&self, ctx: &Context) -> Result {
        ctx.http.remove_friend(&self.id).await
    }

    /// Block the user.
    pub async fn block(&self, ctx: &Context) -> Result {
        ctx.http.block(&self.id).await
    }

    /// Unblock the user.
    pub async fn unblock(&self, ctx: &Context) -> Result {
        ctx.http.unblock(&self.id).await
    }
}

impl std::fmt::Display for User {
    /// Display user mention.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<@{}>", self.id)
    }
}

/// User's relationship with another user.
#[derive(Debug, Clone, Copy, PartialEq, Deserialize, Serialize)]
pub enum RelationshipStatus {
    /// You have no relationship with the user.
    None,
    /// It is your current user.
    User,
    /// You are friends.
    Friend,
    /// You sent a friend request to the user.
    Outgoing,
    /// You have received a friend request from the user.
    Incoming,
    /// You have blocked the user.
    Blocked,
    /// You are blocked by the user.
    BlockedOther,
}

impl Default for RelationshipStatus {
    fn default() -> Self {
        Self::User
    }
}

/// Bot information in case the user is one.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct BotInformation {
    /// Bot owner id.
    #[serde(rename = "owner")]
    pub owner_id: String,
}

/// User's active status.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct UserStatus {
    /// Custom status text.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    /// Current presence.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub presence: Option<Presence>,
}

/// User presence status.
#[derive(Debug, Clone, Copy, PartialEq, Deserialize, Serialize)]
pub enum Presence {
    /// `Online` presence.
    Online,
    /// `Idle` presence.
    Idle,
    /// `Busy` presence.
    Busy,
    /// `Invisible` presence.
    ///
    /// **Note**: Used to define your own presence.
    Invisible,
}

/// User's profile.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct UserProfile {
    /// Text content on user's profile.
    pub content: Option<String>,
    /// Background visible on user's profile.
    pub background: Option<Attachment>,
}

bitflags! {
    /// User badges.
    #[derive(Default, Deserialize, Serialize)]
    #[serde(transparent)]
    pub struct Badges: u16 {
        /// Revolt Developer.
        const DEVELOPER = 1;
        /// Helped translate Revolt.
        const TRANSLATOR = 2;
        /// Monetarily supported Revolt.
        const SUPPORTER = 4;
        /// Responsibly disclosed a security issue.
        const RESPONSIBLE_DISCLOSURE = 8;
        /// Revolt Founder.
        const FOUNDER = 16;
        /// Platform moderator.
        const PLATFORM_MODERATION = 32;
        /// Active monetary supporter.
        const ACTIVE_SUPPORTER = 64;
        /// It's a paw.
        const PAW = 128;
        /// Joined as one of the first 1000 users in 2021.
        const EARLY_ADOPTER = 256;
        /// Amogus.
        const RESERVED_RELEVANT_JOKE_BADGE1 = 512;
        /// Low resolution troll face.
        const RESERVED_RELEVANT_JOKE_BADGE2 = 1024;
    }

    /// User flags.
    #[derive(Default, Deserialize, Serialize)]
    #[serde(transparent)]
    pub struct UserFlags: u8 {
        /// User has been suspended from the platform.
        const SUSPENDED = 1;
        /// User has deleted their account.
        const DELETED = 2;
        /// User was banned off the platform.
        const BANNED = 4;
    }
}
