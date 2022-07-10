#[cfg(feature = "cache")]
use crate::{cache::UpdateCache, Context};

/// A user has joined the server.
#[derive(Debug, Clone, PartialEq, Deserialize)]
#[non_exhaustive]
pub struct ServerMemberJoinEvent {
    /// Server id.
    #[serde(rename = "id")]
    pub server_id: String,
    /// User id.
    #[serde(rename = "user")]
    pub user_id: String,
}

#[cfg(feature = "cache")]
#[async_trait]
impl UpdateCache for ServerMemberJoinEvent {
    async fn update(&self, _cx: &Context) {
        todo!()
    }
}
