use std::sync::Arc;

use crate::{
    http::HttpClient,
    models::{events::ClientEvent, Id, User},
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

    /// Returns the current user.
    pub async fn user(&self) -> Result<User> {
        let user = self.http_client.get("users/@me").await?;

        Ok(user)
    }

    /// Returns the given token.
    pub fn token(&self) -> String {
        self.token.as_ref().clone()
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
}
