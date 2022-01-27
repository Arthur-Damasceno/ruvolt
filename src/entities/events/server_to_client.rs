use {serde::Deserialize, serde_json::Value as Json};

use crate::error::AuthenticationError;

#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(tag = "type")]
pub enum ServerToClientEvent {
    Authenticated,
    Error {
        error: AuthenticationError,
    },
    Pong {
        data: usize,
    },
    MessageUpdate {
        id: String,
        data: Json,
    },
    MessageDelete {
        id: String,
        channel: String,
    },
    ChannelDelete {
        id: String,
    },
    ChannelGroupJoin {
        id: String,
        user: String,
    },
    ChannelGroupLeave {
        id: String,
        user: String,
    },
    ChannelStartTyping {
        id: String,
        user: String,
    },
    ChannelStopTyping {
        id: String,
        user: String,
    },
    ChannelAck {
        id: String,
        user: String,
        message_id: String,
    },
    ServerDelete {
        id: String,
    },
    ServerMemberJoin {
        id: String,
        user: String,
    },
    ServerMemberLeave {
        id: String,
        user: String,
    },
    ServerRoleDelete {
        id: String,
        role_id: String,
    },
}
