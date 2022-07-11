use crate::models::{Attachment, UserProfile, UserStatus};

#[cfg(feature = "cache")]
use crate::{cache::UpdateCache, Context};

/// Specifies a field to remove on user update.
#[derive(Debug, Clone, Copy, PartialEq, Deserialize, Serialize)]
#[non_exhaustive]
pub enum UserField {
    /// User profile content.
    ProfileContent,
    /// User profile background.
    ProfileBackground,
    /// User status presence.
    StatusPresence,
    /// User status text.
    StatusText,
    /// User avatar.
    Avatar,
}

/// A user has been updated.
#[derive(Debug, Clone, PartialEq, Deserialize)]
#[non_exhaustive]
pub struct UserUpdateEvent {
    /// User id.
    #[serde(rename = "id")]
    pub user_id: String,
    /// A partial user.
    pub data: PartialUser,
    /// A specified field to remove on user update.
    pub clear: Option<UserField>,
}

/// A partial user
#[derive(Debug, Clone, PartialEq, Deserialize)]
#[non_exhaustive]
pub struct PartialUser {
    /// User status.
    pub status: Option<UserStatus>,
    /// User profile.
    pub profile: Option<UserProfile>,
    /// User avatar.
    pub avatar: Option<Attachment>,
    /// Whether the user is online.
    pub online: Option<bool>,
}

#[cfg(feature = "cache")]
#[async_trait]
impl UpdateCache for UserUpdateEvent {
    async fn update(&self, cx: &Context) {
        if let Some(user) = cx.cache.users.write().await.get_mut(&self.user_id) {
            if let Some(field) = self.clear {
                match field {
                    UserField::StatusText => {
                        if let Some(ref mut status) = user.status {
                            status.text = None;
                        }
                    }
                    UserField::Avatar => user.avatar = None,
                    _ => {}
                }
            }

            if let Some(ref status) = self.data.status {
                user.status = Some(status.clone());
            }

            if let Some(ref avatar) = self.data.avatar {
                user.avatar = Some(avatar.clone());
            }

            if let Some(online) = self.data.online {
                user.online = online;
            }
        }
    }
}
