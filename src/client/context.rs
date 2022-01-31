use {std::sync::Arc, tokio::sync::Mutex};

use {
    super::websocket::Sender,
    crate::{
        entities::{events::ClientToServerEvent, User},
        http::HttpClient,
        Result,
    },
};

/// A struct for general utilities and wrapper for the http client for fetch entities from the API.
#[derive(Debug)]
pub struct Context {
    pub(crate) ws_tx: Arc<Mutex<Sender>>,
    /// A http client.
    pub http_client: Arc<HttpClient>,
    /// The current user.
    pub user: User,
}

impl Context {
    pub(crate) fn new(http_client: Arc<HttpClient>, ws_tx: Arc<Mutex<Sender>>, user: User) -> Self {
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
    http_client: Arc<HttpClient>,
    ws_tx: Arc<Mutex<Sender>>,
    user: User,
}

impl ContextBuilder {
    pub fn new(http_client: HttpClient, ws_tx: Arc<Mutex<Sender>>, user: User) -> Self {
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
