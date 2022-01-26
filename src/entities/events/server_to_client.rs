use serde::Deserialize;

use crate::error::AuthenticationError;

#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(tag = "type")]
pub enum ServerToClientEvent {
    Authenticated,
    Error { error: AuthenticationError },
    Pong { data: usize },
    ChannelStartTyping { id: String, user: String },
    ChannelStopTyping { id: String, user: String },
}
