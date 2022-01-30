#![allow(missing_debug_implementations)]

use {
    std::sync::Arc,
    tokio::{sync::Mutex, task},
};

use {
    super::cx_builder_from_ready,
    crate::{
        entities::events::{ClientToServerEvent, ServerToClientEvent},
        error::Error,
        websocket::WebSocketClient,
        EventHandler, EventHandlerExt, Result,
    },
};

/// API wrapper to interact with Revolt.
#[derive(Debug)]
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
    pub async fn listen(mut self, token: &str) -> Result {
        self.authenticate(token.into()).await?;

        let (tx, mut rx) = self.ws_client.split();
        let tx = Arc::new(Mutex::new(tx));
        let cx_builder =
            cx_builder_from_ready(&mut rx, tx.clone(), self.event_handler.clone(), token).await?;

        WebSocketClient::heartbeat(tx.clone(), self.event_handler.clone());

        loop {
            let event = rx.recv().await;
            let event_handler = self.event_handler.clone();
            let cx = cx_builder.build();

            task::spawn(async move {
                match event {
                    Ok(event) => event_handler.handle(cx, event).await,
                    Err(err) => event_handler.error(err).await,
                }
            });
        }
    }

    async fn authenticate(&mut self, token: String) -> Result {
        self.ws_client
            .send(ClientToServerEvent::Authenticate { token })
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
