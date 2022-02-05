#![allow(missing_docs)]

use serde::Serialize;

use crate::entities::Id;

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
        channel: Id,
    },
    /// Tell other users that you have stopped typing in a channel.
    EndTyping {
        /// Channel Id.
        channel: Id,
    },
    Ping {
        data: usize,
    },
}
