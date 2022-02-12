#![allow(missing_docs)]

use serde::Serialize;

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
