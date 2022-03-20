use serde::Deserialize;

use crate::models::Id;

#[cfg(feature = "cache")]
use crate::{cache::UpdateCache, Context};

/// A server has been deleted.
#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct ServerDeleteEvent {
    /// Server id.
    #[serde(rename = "id")]
    pub server_id: Id,
}

#[cfg(feature = "cache")]
#[async_trait::async_trait]
impl UpdateCache for ServerDeleteEvent {
    async fn update(&self, cx: &Context) {
        cx.cache.servers.write().await.remove(&self.server_id);
    }
}
