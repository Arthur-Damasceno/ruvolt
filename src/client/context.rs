use {
    std::sync::Arc,
    tokio::time::{sleep, Duration},
};

use crate::{
    http::{Authentication, HttpClient, DELTA_API},
    models::events::ClientEvent,
    state::State,
    ActionMessenger, Result,
};

#[cfg(feature = "cache")]
use crate::cache::Cache;

/// A struct for general utilities and wrapper for the http client.
#[derive(Debug, Clone)]
pub struct Context {
    /// A http client.
    pub http: HttpClient,
    /// A cache.
    #[cfg(feature = "cache")]
    pub cache: Arc<Cache>,
    /// A state.
    pub state: State,
    messenger: ActionMessenger,
}

impl Context {
    pub(crate) async fn new(
        authentication: Authentication,
        messenger: ActionMessenger,
    ) -> Result<Self> {
        let http = HttpClient::new(DELTA_API, authentication).await?;

        Ok(Self {
            http,
            #[cfg(feature = "cache")]
            cache: Arc::new(Cache::new(todo!())),
            state: State::default(),
            messenger,
        })
    }

    /// Tell other users that you have begin typing in a channel.
    pub async fn begin_typing(&self, channel_id: &str) -> Result {
        self.messenger
            .send(ClientEvent::BeginTyping {
                channel_id: channel_id.into(),
            })
            .await
    }

    /// Tell other users that you have stopped typing in a channel.
    pub async fn end_typing(&self, channel_id: &str) -> Result {
        self.messenger
            .send(ClientEvent::EndTyping {
                channel_id: channel_id.into(),
            })
            .await
    }

    /// Get the WebSocket latency.
    ///
    /// If the client sent a heartbeat and did not receive it back, the function will sleep
    /// for `150` milliseconds and try again.
    pub async fn latency(&self) -> Duration {
        loop {
            match self.messenger.latency().await {
                Some(latency) => return latency,
                None => {
                    sleep(Duration::from_millis(150)).await;
                    continue;
                }
            };
        }
    }

    /// Close the WebSocket connection.
    pub async fn close(&self) -> Result {
        self.messenger.close().await
    }
}
