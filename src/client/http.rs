//! A module for the http client that makes requests to the Revolt REST API.

use {
    reqwest::{
        header::{HeaderMap, HeaderValue},
        Client,
    },
    serde::{de::DeserializeOwned, ser::Serialize},
};

use crate::Result;

/// The url of the Revolt REST API.
pub const REVOLT_API: &str = "https://api.revolt.chat";

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

    fn make_url(path: &str) -> String {
        format!("{}/{}", REVOLT_API, path)
    }

    /// Make a GET request to the API and convert the response body to json.
    pub async fn get<T: DeserializeOwned>(&self, path: &str) -> Result<T> {
        let url = Self::make_url(path);
        let response = self.0.get(url).send().await?;
        let body = response.json().await?;

        Ok(body)
    }

    /// Make a POST request to the API with a json body and convert the response body to json.
    pub async fn post<T: DeserializeOwned, U: Serialize>(&self, path: &str, body: U) -> Result<T> {
        let url = Self::make_url(path);
        let response = self.0.post(url).json(&body).send().await?;
        let body = response.json().await?;

        Ok(body)
    }

    /// Make a PATCH request to the API with a json body.
    pub async fn patch<T: Serialize>(&self, path: &str, body: T) -> Result {
        let url = Self::make_url(path);
        self.0.patch(url).json(&body).send().await?;

        Ok(())
    }
}
