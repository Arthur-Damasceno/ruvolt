use {std::sync::Arc, tokio::sync::Mutex};

use crate::{
    http::HttpClient,
    models::{events::ClientToServerEvent, User},
    websocket::Sender,
    Result,
};

/// A struct for general utilities and wrapper for the http client for fetch entities from the API.
#[derive(Debug)]
pub struct Context {
    pub(crate) tx: Arc<Mutex<Sender>>,
    /// A http client.
    pub http_client: Arc<HttpClient>,
    /// The current user.
    pub user: User,
}

impl Context {
    pub(crate) fn new(http_client: Arc<HttpClient>, tx: Arc<Mutex<Sender>>, user: User) -> Self {
        Self {
            http_client,
            tx,
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
            event => self.tx.lock().await.send(event).await,
        }
    }
}

pub(crate) struct ContextFactory {
    http_client: Arc<HttpClient>,
    tx: Arc<Mutex<Sender>>,
    user: User,
}

impl ContextFactory {
    pub fn new(http_client: HttpClient, tx: Arc<Mutex<Sender>>, user: User) -> Self {
        Self {
            http_client: Arc::new(http_client),
            tx,
            user,
        }
    }

    pub fn make(&self) -> Context {
        let http_client = self.http_client.clone();
        let tx = self.tx.clone();
        let user = self.user.clone();

        Context::new(http_client, tx, user)
    }
}
