use {
    futures_util::{
        stream::{SplitSink, SplitStream},
        SinkExt, StreamExt,
    },
    std::{sync::Arc, time::Duration},
    tokio::{net::TcpStream, sync::Mutex, task, time},
    tokio_tungstenite::{connect_async, tungstenite::Message, MaybeTlsStream, WebSocketStream},
};

use crate::{
    error::Error,
    models::events::{ClientToServerEvent, ServerToClientEvent},
    EventHandler, Result,
};

type Stream = WebSocketStream<MaybeTlsStream<TcpStream>>;

#[derive(Debug)]
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

    pub fn heartbeat(tx: Arc<Mutex<Sender>>, event_handler: Arc<impl EventHandler>) {
        task::spawn(async move {
            let mut interval = time::interval(Duration::from_secs(20));

            loop {
                interval.tick().await;

                if let Err(err) = tx
                    .lock()
                    .await
                    .send(ClientToServerEvent::Ping { data: 0 })
                    .await
                {
                    event_handler.error(err).await;
                }
            }
        });
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

#[derive(Debug)]
pub struct Sender(SplitSink<Stream, Message>);

impl Sender {
    pub async fn send(&mut self, event: ClientToServerEvent) -> Result {
        let msg = Message::Text(serde_json::to_string(&event).unwrap());

        self.0.send(msg).await?;

        Ok(())
    }
}

#[derive(Debug)]
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
