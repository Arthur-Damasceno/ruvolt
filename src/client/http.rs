//! A module for the http client that makes requests to the Revolt REST API.

use reqwest::{
    header::{HeaderMap, HeaderValue},
    Client,
};

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
}
