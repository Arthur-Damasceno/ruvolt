#![allow(missing_docs)]

use serde::Serialize;

use crate::models::Id;

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
        #[serde(rename = "channel")]
        channel_id: Id,
    },
    /// Tell other users that you have stopped typing in a channel.
    EndTyping {
        /// Channel Id.
        #[serde(rename = "channel")]
        channel_id: Id,
    },
    Ping {
        data: usize,
    },
}
