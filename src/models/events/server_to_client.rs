use {serde::Deserialize, serde_json::Value as Json};

use crate::{
    error::AuthenticationError,
    models::{events::*, *},
};

#[derive(Debug, Deserialize)]
#[serde(tag = "type")]
pub enum ServerToClientEvent {
    Authenticated,
    Error {
        error: AuthenticationError,
    },
    Pong {
        data: usize,
    },
    Ready(ReadyEvent),
    Message(Message),
    MessageUpdate {
        id: Id,
        data: Json,
    },
    MessageDelete {
        id: Id,
        #[serde(rename = "channel")]
        channel_id: Id,
    },
    ChannelCreate(Channel),
    ChannelUpdate {
        id: Id,
        data: Json,
        clear: Option<RemoveChannelField>,
    },
    ChannelDelete {
        id: Id,
    },
    ChannelGroupJoin {
        id: Id,
        #[serde(rename = "user")]
        user_id: Id,
    },
    ChannelGroupLeave {
        id: Id,
        #[serde(rename = "user")]
        user_id: Id,
    },
    ChannelStartTyping {
        id: Id,
        #[serde(rename = "user")]
        user_id: Id,
    },
    ChannelStopTyping {
        id: Id,
        #[serde(rename = "user")]
        user_id: Id,
    },
    ChannelAck {
        id: Id,
        #[serde(rename = "user")]
        user_id: Id,
        message_id: Id,
    },
    ServerUpdate {
        id: Id,
        data: Json,
        clear: Option<RemoveServerField>,
    },
    ServerDelete {
        id: Id,
    },
    ServerMemberUpdate {
        id: ServerMemberUpdateId,
        data: Json,
        clear: Option<RemoveServerMemberField>,
    },
    ServerMemberJoin {
        id: Id,
        #[serde(rename = "user")]
        user_id: Id,
    },
    ServerMemberLeave {
        id: Id,
        #[serde(rename = "user")]
        user_id: Id,
    },
    ServerRoleUpdate {
        id: Id,
        role_id: Id,
        data: Json,
        clear: Option<RemoveServerRoleField>,
    },
    ServerRoleDelete {
        id: Id,
        role_id: Id,
    },
    UserUpdate {
        id: Id,
        data: Json,
        clear: Option<RemoveUserField>,
    },
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct ServerMemberUpdateId {
    #[serde(rename = "server")]
    pub server_id: Id,
    #[serde(rename = "user")]
    pub user_id: Id,
}
