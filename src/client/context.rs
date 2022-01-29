use {
    reqwest::{
        header::{HeaderMap, HeaderValue},
        Client,
    },
    std::sync::Arc,
    tokio::sync::Mutex,
};

use super::websocket::Sender;

pub struct Context {
    pub(crate) http_client: Arc<Client>,
    pub(crate) ws_tx: Arc<Mutex<Sender>>,
}

impl Context {
    pub(crate) fn new(http_client: Arc<Client>, ws_tx: Arc<Mutex<Sender>>) -> Self {
        Self { http_client, ws_tx }
    }
}

pub struct ContextBuilder {
    http_client: Arc<Client>,
    ws_tx: Arc<Mutex<Sender>>,
}

impl ContextBuilder {
    pub fn new(token: &str, ws_tx: Arc<Mutex<Sender>>) -> Self {
        let mut headers = HeaderMap::new();
        headers.insert("x-bot-token", HeaderValue::from_str(token).unwrap());

        let http_client = Client::builder().default_headers(headers).build().unwrap();

        Self {
            http_client: Arc::new(http_client),
            ws_tx,
        }
    }

    pub fn build(&self) -> Context {
        let http_client = Arc::clone(&self.http_client);
        let ws_tx = Arc::clone(&self.ws_tx);

        Context::new(http_client, ws_tx)
    }
}
