use {
    futures_util::{SinkExt, StreamExt},
    std::{sync::Arc, time::Duration},
    tokio::{net::TcpStream, sync::Mutex, task, time::sleep},
    tokio_tungstenite::{
        connect_async,
        tungstenite::error::{Error as WsError, ProtocolError},
        tungstenite::Message,
        MaybeTlsStream, WebSocketStream,
    },
};

use crate::{
    error::Error,
    models::events::{ClientToServerEvent, ServerToClientEvent},
    Result,
};

pub const REVOLT_WS_API: &str = "wss://ws.revolt.chat";

#[derive(Debug)]
pub struct WebSocketClient(Mutex<WebSocketStream<MaybeTlsStream<TcpStream>>>);

impl WebSocketClient {
    pub async fn connect() -> Result<Self> {
        let (stream, _) = connect_async(REVOLT_WS_API).await?;

        Ok(Self(Mutex::new(stream)))
    }

    pub async fn publish(&self, event: ClientToServerEvent) -> Result {
        let text = serde_json::to_string(&event).unwrap();

        self.send(text).await?;

        Ok(())
    }

    pub async fn accept(&self) -> Option<Result<ServerToClientEvent>> {
        let text = self.recv().await?;

        let result = match text {
            Ok(text) => {
                let event = serde_json::from_str(&text).map_err(|_| {
                    Error::Unknown(format!("Cannot deserialize unexpected event: {}", text))
                });

                match event {
                    Ok(event) => Ok(event),
                    Err(err) => Err(err),
                }
            }
            Err(err) => Err(err),
        };

        Some(result)
    }

    async fn send(&self, text: String) -> Result {
        self.0.lock().await.send(Message::Text(text)).await?;

        Ok(())
    }

    async fn recv(&self) -> Option<Result<String>> {
        let msg = self.0.lock().await.next().await?;

        match msg {
            Ok(msg) => match msg {
                Message::Text(text) => Some(Ok(text)),
                Message::Close(_) => None,
                msg => Some(Err(Error::Unknown(format!(
                    "Unexpected websocket message: {:?}",
                    msg
                )))),
            },
            Err(err) => Some(Err(Error::from(err))),
        }
    }

    pub async fn close(&self) -> Result {
        self.0.lock().await.close(None).await?;

        Ok(())
    }
}

pub fn heartbeat(ws_client: Arc<WebSocketClient>) {
    task::spawn(async move {
        let dur = Duration::from_secs(20);

        loop {
            if let Err(Error::Ws(WsError::Protocol(ProtocolError::SendAfterClosing))) =
                ws_client.publish(ClientToServerEvent::ping(0)).await
            {
                break;
            }

            sleep(dur).await;
        }
    });
}
