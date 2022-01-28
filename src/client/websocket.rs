use {
    futures_util::{
        stream::{SplitSink, SplitStream},
        SinkExt, StreamExt,
    },
    tokio::net::TcpStream,
    tokio_tungstenite::{connect_async, tungstenite::Message, MaybeTlsStream, WebSocketStream},
};

use crate::{
    entities::events::{ClientToServerEvent, ServerToClientEvent},
    error::Error,
    Result,
};

type Stream = WebSocketStream<MaybeTlsStream<TcpStream>>;

pub struct WebSocketClient {
    tx: Sender,
    rx: Receiver,
}

impl WebSocketClient {
    pub async fn connect(url: &str) -> Result<Self> {
        let (stream, _) = connect_async(url).await?;
        let (tx, rx) = stream.split();

        Ok(Self {
            tx: Sender(tx),
            rx: Receiver(rx),
        })
    }

    pub async fn send(&mut self, event: ClientToServerEvent) -> Result {
        self.tx.send(event).await
    }

    pub async fn recv(&mut self) -> Result<ServerToClientEvent> {
        self.rx.recv().await
    }

    pub fn split(self) -> (Sender, Receiver) {
        (self.tx, self.rx)
    }
}

pub struct Sender(SplitSink<Stream, Message>);

impl Sender {
    pub async fn send(&mut self, event: ClientToServerEvent) -> Result {
        let msg = Message::Text(serde_json::to_string(&event).unwrap());

        self.0.send(msg).await?;

        Ok(())
    }
}

pub struct Receiver(SplitStream<Stream>);

impl Receiver {
    pub async fn recv(&mut self) -> Result<ServerToClientEvent> {
        let msg = self.0.next().await.unwrap()?;

        let event = match msg {
            Message::Text(msg) => serde_json::from_str(&msg).map_err(|err| {
                Error::Unknown(format!(
                    "Cannot deserialize unexpected event from server: {}",
                    err
                ))
            })?,
            _ => unimplemented!(),
        };

        Ok(event)
    }
}
