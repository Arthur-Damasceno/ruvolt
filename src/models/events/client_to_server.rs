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

impl ClientToServerEvent {
    pub(crate) fn auth(token: &str) -> Self {
        Self::Authenticate {
            token: token.into(),
        }
    }

    pub(crate) fn ping(data: usize) -> Self {
        Self::Ping { data }
    }

    /// Construct a [ClientToServerEvent::BeginTyping].
    pub fn begin_typing(channel_id: &Id) -> Self {
        Self::BeginTyping {
            channel_id: channel_id.into(),
        }
    }

    /// Construct a [ClientToServerEvent::EndTyping].
    pub fn end_typing(channel_id: &Id) -> Self {
        Self::EndTyping {
            channel_id: channel_id.into(),
        }
    }
}
