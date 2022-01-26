use crate::entities::ServerToClientEvent;

/// A server has been deleted.
#[derive(Debug)]
pub struct ServerDeleteEvent {
    id: String,
}

impl ServerDeleteEvent {
    /// Returns the server id.
    pub fn id(&self) -> &str {
        &self.id
    }
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
