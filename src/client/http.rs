//! A module for the http client that makes requests to the Revolt REST API.

use {
    reqwest::{
        header::{HeaderMap, HeaderValue},
        Client,
    },
    serde::de::DeserializeOwned,
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

    /// Make a GET request to the API and convert the body to json.
    pub async fn get<T: DeserializeOwned>(&self, path: &str) -> Result<T> {
        let url = format!("{}/{}", REVOLT_API, path);
        let response = self.0.get(url).send().await?;
        let body = response.json().await?;

        Ok(body)
    }
}
