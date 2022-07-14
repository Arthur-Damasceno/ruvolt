use crate::{
    error::Result,
    http::{
        builders::{CreateGroup, EditChannel},
        HttpClient, DELTA_API,
    },
    models::{Channel, User},
};

impl HttpClient {
    /// Fetch a channel.
    pub async fn channel(&self, id: &str) -> Result<Channel> {
        self.request(self.inner.get(format!("{DELTA_API}/channels/{id}")))
            .await?
            .json()
            .await
            .map_err(From::from)
    }

    /// Edit a channel.
    pub async fn edit_channel(&self, id: &str, data: &EditChannel) -> Result<Channel> {
        self.request(
            self.inner
                .patch(format!("{DELTA_API}/channels/{id}"))
                .json(data),
        )
        .await?
        .json()
        .await
        .map_err(From::from)
    }

    /// Deletes a server channel, leaves a group or closes a group.
    pub async fn delete_channel(&self, id: &str) -> Result {
        self.request(self.inner.delete(format!("{DELTA_API}/channels/{id}")))
            .await
            .map(|_| ())
    }

    /// Fetch all users who are part of a group.
    pub async fn group_members(&self, id: &str) -> Result<Vec<User>> {
        self.request(self.inner.get(format!("{DELTA_API}/channels/{id}/members")))
            .await?
            .json()
            .await
            .map_err(From::from)
    }

    /// Create a group channel.
    pub async fn create_group(&self, data: &CreateGroup) -> Result<Channel> {
        self.request(
            self.inner
                .post(format!("{DELTA_API}/channels/create"))
                .json(data),
        )
        .await?
        .json()
        .await
        .map_err(From::from)
    }

    /// Add a user to the group.
    pub async fn add_group_member(&self, channel_id: &str, user_id: &str) -> Result {
        self.request(self.inner.put(format!(
            "{DELTA_API}/channels/{channel_id}/recipients/{user_id}"
        )))
        .await
        .map(|_| ())
    }

    /// Remove a user from the group.
    pub async fn remove_group_member(&self, channel_id: &str, user_id: &str) -> Result {
        self.request(self.inner.delete(format!(
            "{DELTA_API}/channels/{channel_id}/recipients/{user_id}"
        )))
        .await
        .map(|_| ())
    }

    /// Get a token to join the call.
    pub async fn join_call(&self, id: &str) -> Result<String> {
        #[derive(Deserialize)]
        struct Data {
            token: String,
        }

        self.request(
            self.inner
                .post(format!("{DELTA_API}/channels/{id}/join_call")),
        )
        .await?
        .json()
        .await
        .map_err(From::from)
        .map(|Data { token }| token)
    }
}
