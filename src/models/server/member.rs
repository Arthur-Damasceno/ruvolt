use serde::{Deserialize, Serialize};

use crate::{
    builders::EditMember,
    models::{Attachment, Id},
    Context, Result,
};

/// A server member id.
#[derive(Debug, Clone, PartialEq, Deserialize, Hash, Eq)]
pub struct MemberId {
    /// Server id.
    #[serde(rename = "server")]
    pub server_id: Id,
    /// User id.
    #[serde(rename = "user")]
    pub user_id: Id,
}

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
    pub roles: Vec<Id>,
}

impl Member {
    /// Get a member from the API.
    pub async fn fetch(cx: &Context, server_id: &Id, user_id: &Id) -> Result<Self> {
        cx.http_client
            .get(format!("servers/{}/members/{}", server_id, user_id))
            .await
    }

    /// Edit the member.
    pub async fn edit(&self, cx: &Context, builder: EditMember) -> Result {
        cx.http_client
            .patch(
                format!("servers/{}/members/{}", self.id.server_id, self.id.user_id),
                builder,
            )
            .await
    }

    /// Kick the member from the server.
    pub async fn kick(&self, cx: &Context) -> Result {
        cx.http_client
            .delete(format!(
                "servers/{}/members/{}",
                self.id.server_id, self.id.user_id
            ))
            .await
    }

    /// Ban the member from the server.
    pub async fn ban(&self, cx: &Context, reason: Option<impl Into<String>>) -> Result {
        cx.http_client
            .put(
                format!("servers/{}/bans/{}", self.id.server_id, self.id.user_id),
                CreateBan::new(reason),
            )
            .await
    }
}

#[derive(Debug, Serialize)]
struct CreateBan {
    #[serde(skip_serializing_if = "Option::is_none")]
    reason: Option<String>,
}

impl CreateBan {
    fn new(reason: Option<impl Into<String>>) -> Self {
        match reason {
            Some(reason) => CreateBan {
                reason: Some(reason.into()),
            },
            None => CreateBan { reason: None },
        }
    }
}
