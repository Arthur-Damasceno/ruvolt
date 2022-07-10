#[cfg(feature = "cache")]
use crate::{cache::UpdateCache, Context};

/// A server has been deleted.
#[derive(Debug, Clone, PartialEq, Deserialize)]
#[non_exhaustive]
pub struct ServerDeleteEvent {
    /// Server id.
    #[serde(rename = "id")]
    pub server_id: String,
}

#[cfg(feature = "cache")]
#[async_trait]
impl UpdateCache for ServerDeleteEvent {
    async fn update(&self, cx: &Context) {
        cx.cache.servers.write().await.remove(&self.server_id);
    }
}
