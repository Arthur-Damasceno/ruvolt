use {
    futures_util::{select, FutureExt},
    std::sync::Arc,
    tracing::{error, info, warn},
};

use crate::{
    error::Error,
    models::events::{ClientEvent, ServerEvent},
    websocket::WebSocketClient,
    Action, ActionMessenger, ActionRx, Context, EventHandler, EventHandlerExt, Result,
};

#[cfg(feature = "cache")]
use crate::cache::Cache;

/// API wrapper to interact with Revolt.
#[derive(Debug)]
pub struct Client<T: EventHandler> {
    event_handler: Arc<T>,
    ws_client: WebSocketClient,
    action_rx: ActionRx,
    context: Context,
}

impl<T: EventHandler> Client<T> {
    /// Create a new client and connect to the server.
    pub async fn new(event_handler: T, token: impl Into<String>) -> Result<Self> {
        let ws_client = WebSocketClient::connect().await?;
        let (messenger, action_rx) = ActionMessenger::new();
        let context = Context::new(token, messenger);

        Ok(Self {
            event_handler: Arc::new(event_handler),
            ws_client,
            action_rx,
            context,
        })
    }

    /// Start listening for server events.
    pub async fn listen(&mut self) -> Result {
        self.authenticate().await?;

        info!(target: "Client", "Client authenticated successfully. Starting listening for events");

        loop {
            if let Err(err) = self.ws_client.check_heartbeat().await {
                warn!(target: "Client", "Err heartbeating: {}", err);
            }

            select! {
                event = self.ws_client.accept().fuse() => {
                    if let Some(event) = event {
                        self.handle_event(event);
                    } else {
                        info!(target: "Client", "Connection closed");
                        return Ok(());
                    }
                },
                action = self.action_rx.recv().fuse() => self.handle_action(action.unwrap()).await,
            }
        }
    }

    async fn authenticate(&mut self) -> Result {
        self.ws_client
            .send(ClientEvent::Authenticate {
                token: self.context.token(),
            })
            .await?;

        let event = self.ws_client.accept().await.ok_or(Error::Unknown(
            "The server closed the connection unexpectedly".into(),
        ))??;

        match event {
            ServerEvent::Authenticated => Ok(()),
            ServerEvent::Error { error } => Err(error.into()),
            event => Err(Error::Unknown(format!(
                "Unexpected event after authentication: {:?}",
                event
            ))),
        }
    }

    fn handle_event(&self, event: Result<ServerEvent>) {
        match event {
            Ok(event) => {
                let event_handler = self.event_handler.clone();
                let context = self.context.clone();

                tokio::spawn(async move {
                    #[cfg(feature = "cache")]
                    Cache::update(&context, &event).await;

                    event_handler.handle(context, event).await
                });
            }
            Err(err) => error!(target: "Client", "Err handling event: {}", err),
        }
    }

    async fn handle_action(&mut self, action: Action) {
        match action {
            Action::SendEvent { event, tx } => tx.send(self.ws_client.send(event).await).unwrap(),
            Action::GetLatency { tx } => tx.send(self.ws_client.latency()).unwrap(),
            Action::Close { tx } => tx.send(self.ws_client.close().await).unwrap(),
        }
    }
}
