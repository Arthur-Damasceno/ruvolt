use crate::{
    error::Result,
    http::{builders::EditMember, HttpClient},
    models::{Member, ServerBan, User},
};

impl HttpClient {
    /// Fetch a member.
    pub async fn member(&self, server_id: &str, user_id: &str) -> Result<Member> {
        self.request(self.get(format!("/servers/{server_id}/members/{user_id}")))
            .await?
            .json()
            .await
            .map_err(From::from)
    }

    /// Fetch all server members.
    pub async fn members(
        &self,
        server_id: &str,
        exclude_offline: bool,
    ) -> Result<(Vec<Member>, Vec<User>)> {
        #[derive(Serialize)]
        struct Query {
            exclude_offline: bool,
        }

        #[derive(Deserialize)]
        struct Data {
            members: Vec<Member>,
            users: Vec<User>,
        }

        self.request(
            self.get(format!("/servers/{server_id}/members"))
                .query(&Query { exclude_offline }),
        )
        .await?
        .json()
        .await
        .map_err(From::from)
        .map(|Data { members, users }| (members, users))
    }

    /// Edit a member.
    pub async fn edit_member(
        &self,
        server_id: &str,
        user_id: &str,
        data: &EditMember,
    ) -> Result<Member> {
        self.request(
            self.patch(format!("/servers/{server_id}/members/{user_id}"))
                .json(data),
        )
        .await?
        .json()
        .await
        .map_err(From::from)
    }

    /// Remove a member from a server.
    pub async fn kick(&self, server_id: &str, user_id: &str) -> Result {
        self.request(self.delete(format!("/servers/{server_id}/members/{user_id}")))
            .await
            .map(|_| ())
    }

    /// Fetch all bans on a server.
    pub async fn bans(&self, server_id: &str) -> Result<(Vec<User>, Vec<ServerBan>)> {
        #[derive(Deserialize)]
        struct Data {
            users: Vec<User>,
            bans: Vec<ServerBan>,
        }

        self.request(self.get(format!("/servers/{server_id}/bans")))
            .await?
            .json()
            .await
            .map_err(From::from)
            .map(|Data { users, bans }| (users, bans))
    }

    /// Ban a user from a server.
    pub async fn ban(
        &self,
        server_id: &str,
        user_id: &str,
        reason: Option<&str>,
    ) -> Result<ServerBan> {
        #[derive(Serialize)]
        struct Data<'a> {
            #[serde(skip_serializing_if = "Option::is_none")]
            reason: Option<&'a str>,
        }

        self.request(
            self.put(format!("/servers/{server_id}/bans/{user_id}"))
                .json(&Data { reason }),
        )
        .await?
        .json()
        .await
        .map_err(From::from)
    }

    /// Remove a user's ban.
    pub async fn unban(&self, server_id: &str, user_id: &str) -> Result {
        self.request(self.delete(format!("/servers/{server_id}/bans/{user_id}")))
            .await
            .map(|_| ())
    }
}
