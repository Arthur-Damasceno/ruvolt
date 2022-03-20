use serde::Deserialize;

use crate::{
    models::{Channel, Id, User},
    Context, Result,
};

#[cfg(feature = "cache")]
use crate::cache::UpdateCache;

/// A user has joined the group.
#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct ChannelGroupJoinEvent {
    /// Channel id.
    #[serde(rename = "id")]
    pub channel_id: Id,
    /// User id.
    #[serde(rename = "user")]
    pub user_id: Id,
}

impl ChannelGroupJoinEvent {
    /// Fetch the channel.
    pub async fn channel(&self, cx: &Context) -> Result<Channel> {
        Channel::fetch(cx, &self.channel_id).await
    }

    /// Fetch the user.
    pub async fn user(&self, cx: &Context) -> Result<User> {
        User::fetch(cx, &self.user_id).await
    }
}

#[cfg(feature = "cache")]
#[async_trait::async_trait]
impl UpdateCache for ChannelGroupJoinEvent {
    async fn update(&self, cx: &Context) {
        if let Some(Channel::Group(ref mut group)) =
            cx.cache.channels.write().await.get_mut(&self.channel_id)
        {
            group.recipients.push(self.user_id.clone());
        }
    }
}
