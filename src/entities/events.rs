#![allow(missing_docs)]

use serde::{Deserialize, Serialize};

use crate::error::AuthenticationError;

/// Client originated events.
#[derive(Debug, Serialize, Clone, PartialEq)]
#[serde(tag = "type")]
pub enum ClientToServerEvent {
    Authenticate {
        token: String,
    },
    /// Tell other users that you have begin typing in a channel.
    BeginTyping {
        /// Channel Id.
        channel: String,
    },
    /// Tell other users that you have stopped typing in a channel.
    EndTyping {
        /// Channel Id.
        channel: String,
    },
    Ping {
        data: usize,
    },
}

#[derive(Debug, Deserialize, Clone, Copy, PartialEq)]
#[serde(tag = "type")]
pub enum ServerToClientEvent {
    Authenticated,
    Error { error: AuthenticationError },
    Pong { data: usize },
}
