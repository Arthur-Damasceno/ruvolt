#[cfg(feature = "cache")]
use crate::{cache::UpdateCache, models::Channel, Context};

/// A user has left the group.
#[derive(Debug, Clone, PartialEq, Deserialize)]
#[non_exhaustive]
pub struct ChannelGroupLeaveEvent {
    /// Channel id.
    #[serde(rename = "id")]
    pub channel_id: String,
    /// User id.
    #[serde(rename = "user")]
    pub user_id: String,
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
