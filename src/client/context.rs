use {
    reqwest::{
        header::{HeaderMap, HeaderValue},
        Client,
    },
    std::sync::Arc,
    tokio::sync::Mutex,
};

use {
    super::websocket::Sender,
    crate::{
        entities::{events::ClientToServerEvent, User},
        Result,
    },
};

/// A struct for general utilities and wrapper for the http client for fetch entities from the API.
#[derive(Debug)]
pub struct Context {
    pub(crate) http_client: Arc<Client>,
    pub(crate) ws_tx: Arc<Mutex<Sender>>,
    /// The current user.
    pub user: User,
}

impl Context {
    pub(crate) fn new(http_client: Arc<Client>, ws_tx: Arc<Mutex<Sender>>, user: User) -> Self {
        Self {
            http_client,
            ws_tx,
            user,
        }
    }

    /// Send an event to the server.
    ///
    /// # Panics
    ///
    /// Panics if the event is [Ping](ClientToServerEvent::Ping) or [Authenticate](ClientToServerEvent::Authenticate).
    pub async fn send(&self, event: ClientToServerEvent) -> Result {
        match event {
            ClientToServerEvent::Ping { .. } | ClientToServerEvent::Authenticate { .. } => {
                panic!("{:?} event is handled by the client", event);
            }
            event => self.ws_tx.lock().await.send(event).await,
        }
    }
}

pub struct ContextBuilder {
    http_client: Arc<Client>,
    ws_tx: Arc<Mutex<Sender>>,
    user: User,
}

impl ContextBuilder {
    pub fn new(token: &str, ws_tx: Arc<Mutex<Sender>>, user: User) -> Self {
        let mut headers = HeaderMap::new();
        headers.insert("x-bot-token", HeaderValue::from_str(token).unwrap());

        let http_client = Client::builder().default_headers(headers).build().unwrap();

        Self {
            http_client: Arc::new(http_client),
            ws_tx,
            user,
        }
    }

    pub fn build(&self) -> Context {
        let http_client = self.http_client.clone();
        let ws_tx = self.ws_tx.clone();
        let user = self.user.clone();

        Context::new(http_client, ws_tx, user)
    }
}
