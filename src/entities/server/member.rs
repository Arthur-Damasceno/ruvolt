use serde::Deserialize;

use crate::{entities::Attachment, Context, Result};

/// A server member.
#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct Member {
    /// Member id.
    #[serde(rename = "_id")]
    pub id: MemberId,
    /// Member nickname.
    pub nickname: Option<String>,
    /// Member avatar.
    pub avatar: Option<Attachment>,
    /// Member roles ids.
    #[serde(default)]
    pub roles: Vec<String>,
}

impl Member {
    /// Get a server member from the API.
    pub async fn fetch(cx: &Context, server_id: &str, member_id: &str) -> Result<Self> {
        let path = format!("servers/{}/members/{}", server_id, member_id);
        let member = cx.http_client.get(&path).await?;

        Ok(member)
    }
}

/// A server member id.
#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct MemberId {
    /// Server id.
    #[serde(rename = "server")]
    pub server_id: String,
    /// User id.
    #[serde(rename = "user")]
    pub user_id: String,
}
