use {
    futures_util::{select, FutureExt},
    tracing::{error, info, warn},
};

use crate::{
    error::Error,
    http::Authentication,
    models::events::{ClientEvent, ServerEvent},
    websocket::WebSocketClient,
    Action, ActionMessenger, ActionRx, Context, EventHandler, EventHandlerExt, Result,
};

#[cfg(feature = "cache")]
use crate::cache::Cache;

/// API wrapper to interact with Revolt.
#[derive(Debug)]
pub struct Client {
    ws_client: WebSocketClient,
    action_rx: ActionRx,
    context: Context,
}

impl Client {
    /// Create a new client (Bot) and connect to the server.
    pub async fn new(token: impl Into<String>) -> Result<Self> {
        Self::with_authentication(Authentication::Bot(token.into())).await
    }

    /// Create a new client (Self-bot) and connect to the server.
    pub async fn session(token: impl Into<String>) -> Result<Self> {
        Self::with_authentication(Authentication::Session(token.into())).await
    }

    async fn with_authentication(authentication: Authentication) -> Result<Self> {
        let ws_client = WebSocketClient::connect().await?;
        let (messenger, action_rx) = ActionMessenger::new();
        let context = Context::new(authentication, messenger).await?;

        Ok(Self {
            ws_client,
            action_rx,
            context,
        })
    }

    /// Start listening for server events.
    pub async fn listen<T: EventHandler>(&mut self) -> Result {
        self.authenticate().await?;

        info!(target: "Client", "Client authenticated successfully. Starting listening for events");

        loop {
            if let Err(err) = self.ws_client.check_heartbeat().await {
                warn!(target: "Client", "Err heartbeating: {}", err);
            }

            select! {
                event = self.ws_client.accept().fuse() => {
                    if let Some(event) = event {
                        self.handle_event::<T>(event);
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
                token: self.context.http.token(),
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

    fn handle_event<T: EventHandler>(&self, event: Result<ServerEvent>) {
        match event {
            Ok(event) => {
                let context = self.context.clone();

                tokio::spawn(async move {
                    #[cfg(feature = "cache")]
                    Cache::update(&context, &event).await;

                    T::handle(context, event).await;
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
