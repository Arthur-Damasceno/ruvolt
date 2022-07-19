use crate::{
    error::Result,
    http::{
        builders::{CreateGroup, EditChannel},
        HttpClient,
    },
    models::{user::User, Channel},
};

impl HttpClient {
    /// Fetch a channel.
    pub async fn channel(&self, id: &str) -> Result<Channel> {
        self.request(self.get(format!("/channels/{id}")))
            .await?
            .json()
            .await
            .map_err(From::from)
    }

    /// Edit a channel.
    pub async fn edit_channel(&self, id: &str, data: &EditChannel) -> Result<Channel> {
        self.request(self.patch(format!("/channels/{id}")).json(data))
            .await?
            .json()
            .await
            .map_err(From::from)
    }

    /// Deletes a server channel, leaves a group or closes a group.
    pub async fn delete_channel(&self, id: &str) -> Result {
        self.request(self.delete(format!("/channels/{id}")))
            .await
            .map(|_| ())
    }

    /// Fetch all users who are part of a group.
    pub async fn group_members(&self, id: &str) -> Result<Vec<User>> {
        self.request(self.get(format!("/channels/{id}/members")))
            .await?
            .json()
            .await
            .map_err(From::from)
    }

    /// Create a group channel.
    pub async fn create_group(&self, data: &CreateGroup) -> Result<Channel> {
        self.request(self.post("/channels/create").json(data))
            .await?
            .json()
            .await
            .map_err(From::from)
    }

    /// Add a user to the group.
    pub async fn add_group_member(&self, channel_id: &str, user_id: &str) -> Result {
        self.request(self.put(format!("/channels/{channel_id}/recipients/{user_id}")))
            .await
            .map(|_| ())
    }

    /// Remove a user from the group.
    pub async fn remove_group_member(&self, channel_id: &str, user_id: &str) -> Result {
        self.request(self.delete(format!("/channels/{channel_id}/recipients/{user_id}")))
            .await
            .map(|_| ())
    }

    /// Get a token to join the call.
    pub async fn join_call(&self, id: &str) -> Result<String> {
        #[derive(Deserialize)]
        struct Data {
            token: String,
        }

        self.request(self.post(format!("/channels/{id}/join_call")))
            .await?
            .json()
            .await
            .map_err(From::from)
            .map(|Data { token }| token)
    }
}
