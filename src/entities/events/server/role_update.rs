use {serde::Deserialize, serde_json::Value as Json};

use crate::entities::ServerToClientEvent;

/// Specifies a field to remove on server role update.
#[derive(Debug, Deserialize, Clone, Copy, PartialEq)]
pub enum RemoveServerRoleField {
    /// Role color.
    #[serde(rename = "Colour")]
    Color,
}

/// A server role details were updated.
#[derive(Debug)]
pub struct ServerRoleUpdateEvent {
    id: String,
    role_id: String,
    data: Json,
    clear: Option<RemoveServerRoleField>,
}

impl ServerRoleUpdateEvent {
    /// Returns the server id.
    pub fn id(&self) -> &str {
        &self.id
    }

    /// Returns the server role id.
    pub fn role_id(&self) -> &str {
        &self.role_id
    }

    /// Returns a partial server role object.
    pub fn data(&self) -> &Json {
        &self.data
    }

    /// Returns a specified field to remove on server role update.
    pub fn clear(&self) -> Option<RemoveServerRoleField> {
        self.clear
    }
}

impl From<ServerToClientEvent> for ServerRoleUpdateEvent {
    fn from(event: ServerToClientEvent) -> Self {
        if let ServerToClientEvent::ServerRoleUpdate {
            id,
            role_id,
            data,
            clear,
        } = event
        {
            Self {
                id,
                role_id,
                data,
                clear,
            }
        } else {
            panic!("An incorrect event was provided: {:?}", event);
        }
    }
}
