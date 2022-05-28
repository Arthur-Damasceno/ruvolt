//! A module for the http client that makes requests to the Revolt REST API.

use {
    reqwest::{Client as ReqwestClient, RequestBuilder, Response},
    serde::{de::DeserializeOwned, ser::Serialize},
    std::sync::Arc,
};

use crate::error::{Error, HttpError, Result};

pub use reqwest::StatusCode;

/// The url of the Revolt REST API.
pub const DELTA_API: &str = "https://api.revolt.chat";

#[derive(Debug)]
pub(crate) enum Authentication {
    Bot(String),
    Session(String),
}

/// A struct to execute requests to the [Revolt REST API](https://developers.revolt.chat/api/).
#[derive(Debug, Clone)]
pub struct HttpClient(ReqwestClient, Arc<Authentication>);

impl HttpClient {
    pub(crate) fn new(authentication: Authentication) -> Self {
        let client = ReqwestClient::new();

        Self(client, Arc::new(authentication))
    }

    /// Returns the given token.
    pub fn token(&self) -> String {
        match self.1.as_ref() {
            Authentication::Bot(token) | Authentication::Session(token) => token.clone(),
        }
    }

    /// Make a `GET` request to the API and convert the response body to json.
    pub async fn get<T: DeserializeOwned>(&self, path: impl AsRef<str>) -> Result<T> {
        Ok(self
            .request(self.0.get(Self::make_url(path)))
            .await?
            .json()
            .await?)
    }

    /// Make a `POST` request to the API with a json body and convert the response body to json.
    pub async fn post<T: DeserializeOwned>(
        &self,
        path: impl AsRef<str>,
        body: impl Serialize,
    ) -> Result<T> {
        Ok(self
            .request(self.0.post(Self::make_url(path)).json(&body))
            .await?
            .json()
            .await?)
    }

    /// Make a `PUT` request to the API with a json body.
    pub async fn put(&self, path: impl AsRef<str>, body: impl Serialize) -> Result {
        self.request(self.0.put(Self::make_url(path)).json(&body))
            .await?;

        Ok(())
    }

    /// Make a `PATCH` request to the API with a json body.
    pub async fn patch(&self, path: impl AsRef<str>, body: impl Serialize) -> Result {
        self.request(self.0.patch(Self::make_url(path)).json(&body))
            .await?;

        Ok(())
    }

    /// Make a `DELETE` request to the API.
    pub async fn delete(&self, path: impl AsRef<str>) -> Result {
        self.request(self.0.delete(Self::make_url(path))).await?;

        Ok(())
    }

    fn make_url(path: impl AsRef<str>) -> String {
        format!("{}/{}", DELTA_API, path.as_ref())
    }

    async fn request(&self, builder: RequestBuilder) -> Result<Response> {
        let response = match self.1.as_ref() {
            Authentication::Bot(token) => builder.header("x-bot-token", token),
            Authentication::Session(token) => builder.header("x-session-token", token),
        }
        .send()
        .await?;

        match response.status().as_u16() {
            200..=299 => Ok(response),
            404 => Err(Error::Http(HttpError::NotFound)),
            429 => todo!(),
            400..=499 => todo!(),
            500 => Err(Error::Http(HttpError::Internal)),
            _ => Err(Error::Http(HttpError::Other {
                status: response.status(),
                url: response.url().to_string(),
                body: response.text().await.ok(),
            })),
        }
    }
}

impl AsRef<ReqwestClient> for HttpClient {
    fn as_ref(&self) -> &ReqwestClient {
        &self.0
    }
}
