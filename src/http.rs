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

    /// Make a `GET` request to the API and convert the response body to json.
    pub async fn get<T: DeserializeOwned>(&self, path: impl AsRef<str>) -> Result<T> {
        Ok(self
            .request(self.inner.get(Self::api(path)))
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
            .request(self.inner.post(Self::api(path)).json(&body))
            .await?
            .json()
            .await?)
    }

    /// Make a `PUT` request to the API with a json body.
    pub async fn put(&self, path: impl AsRef<str>, body: impl Serialize) -> Result {
        self.request(self.inner.put(Self::api(path)).json(&body))
            .await?;

        Ok(())
    }

    /// Make a `PATCH` request to the API with a json body.
    pub async fn patch(&self, path: impl AsRef<str>, body: impl Serialize) -> Result {
        self.request(self.inner.patch(Self::api(path)).json(&body))
            .await?;

        Ok(())
    }

    /// Make a `DELETE` request to the API.
    pub async fn delete(&self, path: impl AsRef<str>) -> Result {
        self.request(self.inner.delete(Self::api(path))).await?;

        Ok(())
    }

    fn api(path: impl AsRef<str>) -> String {
        format!("{}/{}", DELTA_API, path.as_ref())
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
                status: response.status(),
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
