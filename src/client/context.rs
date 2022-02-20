use std::sync::Arc;

use crate::{http::HttpClient, models::User};

/// A struct for general utilities and wrapper for the http client.
#[derive(Debug, Clone)]
pub struct Context {
    /// A http client.
    pub http_client: Arc<HttpClient>,
    /// The current user.
    pub user: User,
}

impl Context {
    pub(crate) fn new(http_client: Arc<HttpClient>, user: User) -> Self {
        Self { http_client, user }
    }
}
