use {
    futures_util::{SinkExt, StreamExt},
    tokio::net::TcpStream,
    tokio_tungstenite::{connect_async, tungstenite::Message, MaybeTlsStream, WebSocketStream},
};

use crate::{
    error::Error,
    models::events::{ClientToServerEvent, ServerToClientEvent},
    Result,
};

const REVOLT_WS_API: &str = "wss://ws.revolt.chat";

#[derive(Debug)]
pub struct WebSocketClient {
    stream: WebSocketStream<MaybeTlsStream<TcpStream>>,
}

impl WebSocketClient {
    pub async fn connect() -> Result<Self> {
        let (stream, _) = connect_async(REVOLT_WS_API).await?;

        Ok(Self { stream })
    }

    pub async fn send(&mut self, event: ClientToServerEvent) -> Result {
        let msg = Message::Text(serde_json::to_string(&event).unwrap());

        self.stream.send(msg).await?;

        Ok(())
    }

    pub async fn accept(&mut self) -> Option<Result<ServerToClientEvent>> {
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
                        Ok(event) => Some(Ok(event)),
                        Err(err) => Some(Err(err)),
                    }
                }
                Message::Close(_) => None,
                _ => todo!(),
            },
            Err(err) => Some(Err(err.into())),
        }
    }
}
