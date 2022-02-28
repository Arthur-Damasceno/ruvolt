use {
    std::sync::Arc,
    tokio::time::{sleep, Duration},
};

use crate::{
    builders::EditUser,
    http::HttpClient,
    models::{events::ClientEvent, Channel, Id, User},
    ActionMessenger, Result,
};

/// A struct for general utilities and wrapper for the http client.
#[derive(Debug, Clone)]
pub struct Context {
    /// A http client.
    pub http_client: HttpClient,
    token: Arc<String>,
    messenger: ActionMessenger,
}

impl Context {
    pub(crate) fn new(token: impl Into<String>, messenger: ActionMessenger) -> Self {
        let token = token.into();
        let http_client = HttpClient::new(&token);

        Self {
            http_client,
            token: Arc::new(token),
            messenger,
        }
    }

    /// Returns the given token.
    pub fn token(&self) -> String {
        self.token.as_ref().clone()
    }

    /// Returns the current user.
    pub async fn user(&self) -> Result<User> {
        let user = self.http_client.get("users/@me").await?;

        Ok(user)
    }

    /// Edit the current user.
    pub async fn edit(&self, builder: impl Into<EditUser>) -> Result {
        self.http_client.patch("users/@me", builder.into()).await
    }

    /// Tell other users that you have begin typing in a channel.
    pub async fn begin_typing(&self, channel_id: &Id) -> Result {
        self.messenger
            .send(ClientEvent::BeginTyping {
                channel_id: channel_id.clone(),
            })
            .await
    }

    /// Tell other users that you have stopped typing in a channel.
    pub async fn end_typing(&self, channel_id: &Id) -> Result {
        self.messenger
            .send(ClientEvent::EndTyping {
                channel_id: channel_id.clone(),
            })
            .await
    }

    /// Get the WebSocket latency.
    ///
    /// If the client sent a heartbeat and did not receive it back, the function will sleep
    /// for `150` milliseconds and try again.
    pub async fn latency(&self) -> Duration {
        loop {
            match self.messenger.latency().await {
                Some(latency) => return latency,
                None => {
                    sleep(Duration::from_millis(150)).await;
                    continue;
                }
            };
        }
    }

    /// Close the WebSocket connection.
    pub async fn close(&self) -> Result {
        self.messenger.close().await
    }

    /// Fetch your direct messages, including any DM and group conversations.
    pub async fn dm_channels(&self) -> Result<Vec<Channel>> {
        self.http_client.get("users/dms").await
    }
}
