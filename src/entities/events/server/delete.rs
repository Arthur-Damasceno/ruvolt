use super::super::ServerToClientEvent;

/// A server has been deleted.
#[derive(Debug)]
pub struct ServerDeleteEvent {
    /// Server id.
    pub id: String,
}

impl From<ServerToClientEvent> for ServerDeleteEvent {
    fn from(event: ServerToClientEvent) -> Self {
        if let ServerToClientEvent::ServerDelete { id } = event {
            Self { id }
        } else {
            panic!("An incorrect event was provided: {:?}", event);
        }
    }
}
