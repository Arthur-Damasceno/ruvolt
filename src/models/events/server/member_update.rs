use {serde::Deserialize, serde_json::Value as Json};

use crate::{
    models::{MemberId, Server},
    Context, Result,
};

/// Specifies a field to remove on server member update.
#[derive(Debug, Deserialize, Clone, Copy, PartialEq)]
pub enum RemoveServerMemberField {
    /// Server member nickname.
    Nickname,
    /// Server member avatar.
    Avatar,
}

/// A server member details were updated.
#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct ServerMemberUpdateEvent {
    /// Member id.
    #[serde(rename = "id")]
    pub member_id: MemberId,
    /// A partial server member object.
    pub data: Json,
    /// A specified field to remove on server member update.
    pub clear: Option<RemoveServerMemberField>,
}

impl ServerMemberUpdateEvent {
    /// Get the server from the API.
    pub async fn fetch_server(&self, cx: &Context) -> Result<Server> {
        Server::fetch(cx, &self.member_id.server_id).await
    }
}
