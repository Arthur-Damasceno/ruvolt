use {
    futures_util::{SinkExt, StreamExt},
    std::time::{Duration, Instant},
    tokio::net::TcpStream,
    tokio_tungstenite::{connect_async, tungstenite::Message, MaybeTlsStream, WebSocketStream},
};

use crate::{
    error::Error,
    models::events::{ClientEvent, ServerEvent},
    Result,
};

const REVOLT_WS_API: &str = "wss://ws.revolt.chat";

#[derive(Debug)]
pub struct WebSocketClient {
    stream: WebSocketStream<MaybeTlsStream<TcpStream>>,
    heartbeat_dur: Duration,
    last_heartbeat: (Instant, Instant),
}

impl WebSocketClient {
    pub async fn connect() -> Result<Self> {
        let (stream, _) = connect_async(REVOLT_WS_API).await?;
        let now = Instant::now();

        Ok(Self {
            stream,
            heartbeat_dur: Duration::from_secs(20),
            last_heartbeat: (now, now),
        })
    }

    pub async fn send(&mut self, event: ClientEvent) -> Result {
        let msg = Message::Text(serde_json::to_string(&event).unwrap());

        self.stream.send(msg).await?;

        Ok(())
    }

    pub async fn accept(&mut self) -> Option<Result<ServerEvent>> {
        match self.stream.next().await? {
            Ok(msg) => match msg {
                Message::Text(text) => {
                    let event = serde_json::from_str(&text).map_err(|_| {
                        Error::Unknown(format!(
                            "Cannot deserialize a websocket message: {:?}",
                            text
                        ))
                    });

                    match event {
                        Ok(event) => {
                            self.check_pong(&event);

                            Some(Ok(event))
                        }
                        Err(err) => Some(Err(err)),
                    }
                }
                Message::Close(_) => None,
                _ => todo!(),
            },
            Err(err) => Some(Err(err.into())),
        }
    }

    pub fn latency(&self) -> Option<Duration> {
        if self.last_heartbeat.0 < self.last_heartbeat.1 {
            Some(self.last_heartbeat.1 - self.last_heartbeat.0);
        }

        None
    }

    pub async fn close(&mut self) -> Result {
        self.stream.close(None).await?;

        Ok(())
    }

    pub async fn check_heartbeat(&mut self) -> Result {
        let dur = Instant::now() - self.last_heartbeat.0;

        if dur >= self.heartbeat_dur {
            self.heartbeat().await?;
        }

        Ok(())
    }

    async fn heartbeat(&mut self) -> Result {
        self.send(ClientEvent::Ping { data: 0 }).await?;

        self.last_heartbeat.0 = Instant::now();

        Ok(())
    }

    fn check_pong(&mut self, event: &ServerEvent) {
        if let ServerEvent::Pong { .. } = event {
            self.last_heartbeat.1 = Instant::now();
        }
    }
}
