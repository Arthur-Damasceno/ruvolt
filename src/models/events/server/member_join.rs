use serde::Deserialize;

use crate::{
    models::{Id, Member, Server, User},
    Context, Result,
};

/// A user has joined the server.
#[derive(Debug, Deserialize, Clone, PartialEq)]
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
