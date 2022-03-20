use serde::Deserialize;

use crate::{
    models::{Id, Server, User},
    Context, Result,
};

#[cfg(feature = "cache")]
use crate::cache::UpdateCache;

/// A user has left the server.
#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct ServerMemberLeaveEvent {
    /// Server id.
    #[serde(rename = "id")]
    pub server_id: Id,
    /// User id.
    #[serde(rename = "user")]
    pub user_id: Id,
}

impl ServerMemberLeaveEvent {
    /// Fetch the server.
    pub async fn server(&self, cx: &Context) -> Result<Server> {
        Server::fetch(cx, &self.server_id).await
    }

    /// Fetch the user.
    pub async fn user(&self, cx: &Context) -> Result<User> {
        User::fetch(cx, &self.user_id).await
    }
}

#[cfg(feature = "cache")]
#[async_trait::async_trait]
impl UpdateCache for ServerMemberLeaveEvent {
    async fn update(&self, cx: &Context) {
        cx.cache
            .members
            .write()
            .await
            .remove(&(&self.server_id, &self.user_id).into());
    }
}
