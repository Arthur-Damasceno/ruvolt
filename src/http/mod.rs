//! A module for the http client.

pub mod builders;
mod channels;
mod messages;
mod users;

use {
    reqwest::{Client as ReqwestClient, RequestBuilder, Response},
    std::sync::Arc,
};

use crate::error::{Error, HttpError, Result};

/// The url of the Revolt REST API.
pub const DELTA_API: &'static str = "https://api.revolt.chat";

#[derive(Debug)]
pub(crate) enum Authentication {
    Bot(String),
    Session(String),
}

/// A client to send http requests.
#[derive(Debug, Clone)]
pub struct HttpClient {
    inner: ReqwestClient,
    authentication: Arc<Authentication>,
}

impl HttpClient {
    pub(crate) fn new(authentication: Authentication) -> Self {
        Self {
            inner: ReqwestClient::new(),
            authentication: authentication.into(),
        }
    }

    /// Returns the given token.
    pub fn token(&self) -> String {
        match self.authentication.as_ref() {
            Authentication::Bot(token) | Authentication::Session(token) => token.clone(),
        }
    }

    async fn request(&self, builder: RequestBuilder) -> Result<Response> {
        let response = match self.authentication.as_ref() {
            Authentication::Bot(token) => builder.header("X-Bot-Token", token),
            Authentication::Session(token) => builder.header("X-Session-Token", token),
        }
        .send()
        .await?;

        match response.status().as_u16() {
            200..=299 => Ok(response),
            404 => Err(Error::Http(HttpError::NotFound)),
            429 => {
                #[derive(Deserialize)]
                struct RetryAfter {
                    retry_after: u64,
                }

                let RetryAfter { retry_after } = response.json().await?;

                Err(Error::Http(HttpError::RetryAfter(retry_after)))
            }
            400..=499 => todo!(),
            500 => Err(Error::Http(HttpError::Internal)),
            _ => Err(Error::Http(HttpError::Other {
                status: response.status().as_u16(),
                url: response.url().to_string(),
                body: response.text().await.ok(),
            })),
        }
    }
}

impl AsRef<ReqwestClient> for HttpClient {
    fn as_ref(&self) -> &ReqwestClient {
        &self.inner
    }
}
