use crate::{
    models::{Id, Member, Server, User},
    Context, Result,
};

#[cfg(feature = "cache")]
use crate::cache::UpdateCache;

/// A user has joined the server.
#[derive(Debug, Deserialize, Clone, PartialEq)]
#[non_exhaustive]
pub struct ServerMemberJoinEvent {
    /// Server id.
    #[serde(rename = "id")]
    pub server_id: Id,
    /// User id.
    #[serde(rename = "user")]
    pub user_id: Id,
}

impl ServerMemberJoinEvent {
    /// Fetch the member.
    pub async fn member(&self, cx: &Context) -> Result<Member> {
        Member::fetch(cx, &self.server_id, &self.user_id).await
    }

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
#[async_trait]
impl UpdateCache for ServerMemberJoinEvent {
    async fn update(&self, cx: &Context) {
        if let Ok(member) = Member::fetch(cx, &self.server_id, &self.user_id).await {
            cx.cache
                .members
                .write()
                .await
                .insert(member.id.clone(), member);
        }
    }
}
