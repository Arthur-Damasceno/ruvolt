use std::sync::Arc;

use crate::{http::HttpClient, models::User, Result};

/// A struct for general utilities and wrapper for the http client.
#[derive(Debug, Clone)]
pub struct Context {
    /// A http client.
    pub http_client: Arc<HttpClient>,
}

impl Context {
    pub(crate) fn new(http_client: Arc<HttpClient>) -> Self {
        Self { http_client }
    }

    /// Returns the current user.
    pub async fn user(&self) -> Result<User> {
        let user = self.http_client.get("users/@me").await?;

        Ok(user)
    }
}
