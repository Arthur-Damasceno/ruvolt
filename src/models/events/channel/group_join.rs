#[cfg(feature = "cache")]
use crate::{cache::UpdateCache, models::Channel, Context};

/// A user has joined the group.
#[derive(Debug, Clone, PartialEq, Deserialize)]
#[non_exhaustive]
pub struct ChannelGroupJoinEvent {
    /// Channel id.
    #[serde(rename = "id")]
    pub channel_id: String,
    /// User id.
    #[serde(rename = "user")]
    pub user_id: String,
}

#[cfg(feature = "cache")]
#[async_trait]
impl UpdateCache for ChannelGroupJoinEvent {
    async fn update(&self, cx: &Context) {
        if let Some(Channel::Group(ref mut group)) =
            cx.cache.channels.write().await.get_mut(&self.channel_id)
        {
            group.recipients.push(self.user_id.clone());
        }
    }
}
