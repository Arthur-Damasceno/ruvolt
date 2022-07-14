use crate::{
    error::Result,
    http::{builders::EditUser, HttpClient, DELTA_API},
    models::{Channel, User, UserProfile},
};

impl HttpClient {
    /// Fetch a user.
    ///
    /// **Note**: To fetch the current user, the `id` must be `@me`.
    pub async fn user(&self, id: &str) -> Result<User> {
        self.request(self.inner.get(format!("{DELTA_API}/users/{id}")))
            .await?
            .json()
            .await
            .map_err(From::from)
    }

    /// Fetch a user profile.
    pub async fn user_profile(&self, id: &str) -> Result<UserProfile> {
        self.request(self.inner.get(format!("{DELTA_API}/users/{id}/profile")))
            .await?
            .json()
            .await
            .map_err(From::from)
    }

    /// Edit the current user.
    pub async fn edit_user(&self, data: &EditUser) -> Result<User> {
        self.request(
            self.inner
                .patch(format!("{DELTA_API}/users/@me"))
                .json(data),
        )
        .await?
        .json()
        .await
        .map_err(From::from)
    }

    /// Change your username.
    pub async fn change_username(&self, username: &str, password: &str) -> Result<User> {
        #[derive(Serialize)]
        struct Data<'a> {
            username: &'a str,
            password: &'a str,
        }

        self.request(
            self.inner
                .patch(format!("{DELTA_API}/users/@me/username"))
                .json(&Data { username, password }),
        )
        .await?
        .json()
        .await
        .map_err(From::from)
    }

    /// Fetch your DM and group channels.
    pub async fn direct_message_channels(&self) -> Result<Vec<Channel>> {
        self.request(self.inner.get(format!("{DELTA_API}/users/dms")))
            .await?
            .json()
            .await
            .map_err(From::from)
    }

    /// Open a DM with a user.
    ///
    /// If the `id` is yours, a saved message channel is returned.
    pub async fn open_direct_message(&self, id: &str) -> Result<Channel> {
        self.request(self.inner.get(format!("{DELTA_API}/users/{id}/dm")))
            .await?
            .json()
            .await
            .map_err(From::from)
    }

    /// Fetch a list of mutual friends and servers with a user.
    ///
    /// **Note**: The return type consists of a tuple with the ids of users and servers, respectively.
    pub async fn mutual(&self, id: &str) -> Result<(Vec<String>, Vec<String>)> {
        #[derive(Deserialize)]
        struct Data {
            users: Vec<String>,
            servers: Vec<String>,
        }

        self.request(self.inner.get(format!("{DELTA_API}/users/{id}/mutual")))
            .await?
            .json()
            .await
            .map_err(From::from)
            .map(|Data { users, servers }| (users, servers))
    }

    /// Send a friend request to a user.
    pub async fn add_friend(&self, username: &str) -> Result {
        #[derive(Serialize)]
        struct Data<'a> {
            username: &'a str,
        }

        self.request(
            self.inner
                .post(format!("{DELTA_API}/users/friend"))
                .json(&Data { username }),
        )
        .await
        .map(|_| ())
    }

    /// Accept a user's friend request.
    pub async fn accept_friend(&self, id: &str) -> Result {
        self.request(self.inner.put(format!("{DELTA_API}/users/{id}/friend")))
            .await
            .map(|_| ())
    }

    /// Denies a user's friend request or removes an existing friend.
    pub async fn remove_friend(&self, id: &str) -> Result {
        self.request(self.inner.delete(format!("{DELTA_API}/users/{id}/friend")))
            .await
            .map(|_| ())
    }

    /// Block a user.
    pub async fn block(&self, id: &str) -> Result {
        self.request(self.inner.put(format!("{DELTA_API}/users/{id}/block")))
            .await
            .map(|_| ())
    }

    /// Unblock a user.
    pub async fn unblock(&self, id: &str) -> Result {
        self.request(self.inner.delete(format!("{DELTA_API}/users/{id}/block")))
            .await
            .map(|_| ())
    }
}
