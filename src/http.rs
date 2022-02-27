//! A module for the http client that makes requests to the Revolt REST API.

use {
    reqwest::{
        header::{HeaderMap, HeaderValue},
        Client,
    },
    serde::{de::DeserializeOwned, ser::Serialize},
};

use crate::error::{Error, Result};

/// The url of the Revolt REST API.
pub const REVOLT_REST_API: &str = "https://api.revolt.chat";

/// A struct to execute requests to the [Revolt REST API](https://developers.revolt.chat/api/).
#[derive(Debug, Clone)]
pub struct HttpClient(Client);

impl HttpClient {
    pub(crate) fn new(token: &str) -> Self {
        let mut headers = HeaderMap::new();
        headers.insert("x-bot-token", HeaderValue::from_str(token).unwrap());

        let client = Client::builder().default_headers(headers).build().unwrap();

        Self(client)
    }

    fn make_url(path: impl AsRef<str>) -> String {
        format!("{}/{}", REVOLT_REST_API, path.as_ref())
    }

    /// Make a `GET` request to the API and convert the response body to json.
    pub async fn get<T: DeserializeOwned>(&self, path: impl AsRef<str>) -> Result<T> {
        let response = self.0.get(Self::make_url(path)).send().await?;

        if !response.status().is_success() {
            return Err(Error::UnsuccessfulRequest(response));
        }

        let body = response.json().await?;

        Ok(body)
    }

    /// Make a `POST` request to the API with a json body and convert the response body to json.
    pub async fn post<T: DeserializeOwned, U: Serialize>(
        &self,
        path: impl AsRef<str>,
        body: U,
    ) -> Result<T> {
        let response = self.0.post(Self::make_url(path)).json(&body).send().await?;

        if !response.status().is_success() {
            return Err(Error::UnsuccessfulRequest(response));
        }

        let body = response.json().await?;

        Ok(body)
    }

    /// Make a `PATCH` request to the API with a json body.
    pub async fn patch<T: Serialize>(&self, path: impl AsRef<str>, body: T) -> Result {
        let response = self
            .0
            .patch(Self::make_url(path))
            .json(&body)
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(Error::UnsuccessfulRequest(response));
        }

        Ok(())
    }

    /// Make a `DELETE` request to the API.
    pub async fn delete(&self, path: impl AsRef<str>) -> Result {
        let response = self.0.delete(Self::make_url(path)).send().await?;

        if !response.status().is_success() {
            return Err(Error::UnsuccessfulRequest(response));
        }

        Ok(())
    }
}
