#![allow(missing_debug_implementations)]

use {
    futures_util::{select, FutureExt},
    std::{sync::Arc, time::Duration},
    tokio::{sync::Mutex, task, time},
};

use crate::{
    entities::events::{ClientToServerEvent, ServerToClientEvent},
    error::Error,
    websocket::WebSocketClient,
    EventHandler, EventHandlerExt, Result,
};

/// API wrapper to interact with Revolt.
pub struct Client<T: EventHandler> {
    event_handler: Arc<T>,
    ws_client: WebSocketClient,
}

impl<T: EventHandler> Client<T> {
    /// Create a new client and connect to the server.
    pub async fn new(event_handler: T) -> Result<Self> {
        let ws_client = WebSocketClient::connect("wss://ws.revolt.chat").await?;

        Ok(Self {
            event_handler: Arc::new(event_handler),
            ws_client,
        })
    }

    /// Start listening for server events.
    pub async fn listen(mut self, token: String) -> Result {
        self.authenticate(token).await?;

        let (tx, mut rx) = self.ws_client.split();
        let tx = Arc::new(Mutex::new(tx));
        let mut interval = time::interval(Duration::from_secs(20));

        loop {
            select! {
                _ = interval.tick().fuse() => {
                    let tx = Arc::clone(&tx);
                    let event_handler = Arc::clone(&self.event_handler);

                    task::spawn(async move {
                        let mut tx = tx.lock().await;

                        match tx.send(ClientToServerEvent::Ping { data: 0 }).await {
                            Ok(_) => return,
                            Err(err) => event_handler.error(err).await
                        }
                    });
                }
                event = rx.recv().fuse() => {
                    let event_handler = Arc::clone(&self.event_handler);

                    task::spawn(async move {
                        match event {
                            Ok(event) => event_handler.handle(event).await,
                            Err(err) => event_handler.error(err).await
                        }
                    });
                }
            }
        }
    }

    async fn authenticate(&mut self, token: String) -> Result {
        self.ws_client
            .send(ClientToServerEvent::Authenticate {
                token: token.clone(),
            })
            .await?;

        let event = self.ws_client.recv().await?;

        match event {
            ServerToClientEvent::Authenticated => Ok(()),
            ServerToClientEvent::Error { error } => Err(Error::Authentication(error)),
            event => Err(Error::Unknown(format!(
                "Unexpected event after authentication: {:?}",
                event,
            ))),
        }
    }
}
