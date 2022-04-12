use crate::{
    models::{Channel, Id, User},
    Context, Result,
};

#[cfg(feature = "cache")]
use crate::cache::UpdateCache;

/// A user has left the group.
#[derive(Debug, Deserialize, Clone, PartialEq)]
#[non_exhaustive]
pub struct ChannelGroupLeaveEvent {
    /// Channel id.
    #[serde(rename = "id")]
    pub channel_id: Id,
    /// User id.
    #[serde(rename = "user")]
    pub user_id: Id,
}

impl ChannelGroupLeaveEvent {
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
#[async_trait]
impl UpdateCache for ChannelGroupLeaveEvent {
    async fn update(&self, cx: &Context) {
        if let Some(Channel::Group(ref mut channel)) =
            cx.cache.channels.write().await.get_mut(&self.channel_id)
        {
            if let Some(index) = channel
                .recipients
                .iter()
                .position(|user_id| *user_id == self.user_id)
            {
                channel.recipients.remove(index);
            }
        }
    }
}
