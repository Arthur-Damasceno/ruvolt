#[cfg(feature = "cache")]
use crate::{cache::UpdateCache, Context};

/// A channel has been deleted.
#[derive(Debug, Clone, PartialEq, Deserialize)]
#[non_exhaustive]
pub struct ChannelDeleteEvent {
    /// Channel id.
    #[serde(rename = "id")]
    pub channel_id: String,
}

#[cfg(feature = "cache")]
#[async_trait]
impl UpdateCache for ChannelDeleteEvent {
    async fn update(&self, cx: &Context) {
        cx.cache.channels.write().await.remove(&self.channel_id);
    }
}
