use {
    super::super::ServerToClientEvent,
    crate::{
        entities::{Id, Server},
        Context, Result,
    },
};

/// A user has joined the group.
#[derive(Debug)]
pub struct ServerMemberJoinEvent {
    /// Server id.
    pub id: Id,
    /// Server member id.
    pub user_id: Id,
}

impl ServerMemberJoinEvent {
    /// Get the server from the API.
    pub async fn fetch_server(&self, cx: &Context) -> Result<Server> {
        Server::fetch(cx, &self.id).await
    }
}

impl From<ServerToClientEvent> for ServerMemberJoinEvent {
    fn from(event: ServerToClientEvent) -> Self {
        if let ServerToClientEvent::ServerMemberJoin { id, user_id } = event {
            Self { id, user_id }
        } else {
            panic!("An incorrect event was provided: {:?}", event);
        }
    }
}
