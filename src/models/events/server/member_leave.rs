#[cfg(feature = "cache")]
use crate::{cache::UpdateCache, models::MemberId, Context};

/// A user has left the server.
#[derive(Debug, Clone, PartialEq, Deserialize)]
#[non_exhaustive]
pub struct ServerMemberLeaveEvent {
    /// Server id.
    #[serde(rename = "id")]
    pub server_id: String,
    /// User id.
    #[serde(rename = "user")]
    pub user_id: String,
}

#[cfg(feature = "cache")]
#[async_trait]
impl UpdateCache for ServerMemberLeaveEvent {
    async fn update(&self, cx: &Context) {
        cx.cache
            .members
            .write()
            .await
            .remove(&MemberId::new(&self.server_id, &self.user_id));
    }
}
