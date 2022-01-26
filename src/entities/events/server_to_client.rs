use serde::Deserialize;

use crate::error::AuthenticationError;

#[derive(Debug, Deserialize, Clone, Copy, PartialEq)]
#[serde(tag = "type")]
pub enum ServerToClientEvent {
    Authenticated,
    Error { error: AuthenticationError },
    Pong { data: usize },
}
