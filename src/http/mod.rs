//! A module for the http client.

pub mod builders;
mod config;
mod routes;

#[doc(inline)]
pub use config::*;

use {
    reqwest::{Client as ReqwestClient, IntoUrl, RequestBuilder, Response},
    std::fmt::Display,
};

use crate::error::{Error, HttpError, Result};

/// The REST API url of the main Revolt server.
pub const DELTA_API: &'static str = "https://api.revolt.chat";

/// Authentication method used to access the Revolt platform.
#[derive(Debug, Clone)]
pub enum Authentication {
    /// Authenticate through a bot token.
    Bot(String),
    /// Authenticate through a user session.
    Session(String),
    /// Used only to make unauthenticated requests.
    None,
}

impl Authentication {
    /// Returns the [Authentication::Bot] or [Authentication::Session] token.
    ///
    /// # Panics
    /// Panics if [Authentication] is [Authentication::None].
    pub fn token(&self) -> &str {
        match self {
            Self::Bot(token) | Self::Session(token) => token,
            Self::None => panic!("The token was not provided"),
        }
    }
}

/// A client to send http requests to the Revolt API.
#[derive(Debug, Clone)]
pub struct HttpClient {
    inner: ReqwestClient,
    /// REST API url, where requests will be made.
    pub url: String,
    /// Configuration of the Revolt server.
    pub config: RevoltConfig,
    /// Authentication method used in requests.
    pub authentication: Authentication,
}

impl HttpClient {
    /// Fetch Revolt server configuration and create a http client.
    pub async fn new(url: impl IntoUrl, authentication: Authentication) -> Result<Self> {
        let inner = ReqwestClient::new();
        let (url, config) = (
            url.as_str().into(),
            inner.get(url).send().await?.json().await?,
        );

        Ok(Self {
            inner,
            url,
            config,
            authentication,
        })
    }

    fn get(&self, path: impl Display) -> RequestBuilder {
        self.inner.get(format!("{}{path}", self.url))
    }

    fn post(&self, path: impl Display) -> RequestBuilder {
        self.inner.post(format!("{}{path}", self.url))
    }

    fn put(&self, path: impl Display) -> RequestBuilder {
        self.inner.put(format!("{}{path}", self.url))
    }

    fn patch(&self, path: impl Display) -> RequestBuilder {
        self.inner.patch(format!("{}{path}", self.url))
    }

    fn delete(&self, path: impl Display) -> RequestBuilder {
        self.inner.delete(format!("{}{path}", self.url))
    }

    async fn request(&self, builder: RequestBuilder) -> Result<Response> {
        let response = match &self.authentication {
            Authentication::Bot(token) => builder.header("X-Bot-Token", token),
            Authentication::Session(token) => builder.header("X-Session-Token", token),
            Authentication::None => {
                panic!("Unable to make an authenticated request without the token")
            }
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
