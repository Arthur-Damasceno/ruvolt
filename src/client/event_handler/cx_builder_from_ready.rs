use {
    std::sync::Arc,
    tokio::{sync::Mutex, task},
};

use {
    super::{
        super::websocket::{Receiver, Sender},
        EventHandler,
    },
    crate::{entities::events::ServerToClientEvent, error::Error, ContextBuilder, Result},
};

pub async fn cx_builder_from_ready(
    rx: &mut Receiver,
    tx: Arc<Mutex<Sender>>,
    event_handler: Arc<impl EventHandler>,
    token: &str,
) -> Result<ContextBuilder> {
    let event = rx.recv().await?;

    if let ServerToClientEvent::Ready(mut data) = event {
        let user = data.users.remove(0);
        let cx_builder = ContextBuilder::new(token, tx, user);
        let cx = cx_builder.build();

        task::spawn(async move {
            event_handler.ready(cx, data).await;
        });

        Ok(cx_builder)
    } else {
        Err(Error::Unknown(format!(
            "Event other than ready after authentication: {:?}",
            event
        )))
    }
}
