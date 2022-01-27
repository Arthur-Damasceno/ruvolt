use {serde::Deserialize, serde_json::Value as Json};

use {
    super::{
        RemoveChannelField, RemoveServerField, RemoveServerMemberField, RemoveServerRoleField,
        RemoveUserField,
    },
    crate::error::AuthenticationError,
};

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
        #[serde(rename = "channel")]
        channel_id: String,
    },
    ChannelUpdate {
        id: String,
        data: Json,
        clear: Option<RemoveChannelField>,
    },
    ChannelDelete {
        id: String,
    },
    ChannelGroupJoin {
        id: String,
        #[serde(rename = "user")]
        user_id: String,
    },
    ChannelGroupLeave {
        id: String,
        #[serde(rename = "user")]
        user_id: String,
    },
    ChannelStartTyping {
        id: String,
        #[serde(rename = "user")]
        user_id: String,
    },
    ChannelStopTyping {
        id: String,
        #[serde(rename = "user")]
        user_id: String,
    },
    ChannelAck {
        id: String,
        #[serde(rename = "user")]
        user_id: String,
        message_id: String,
    },
    ServerUpdate {
        id: String,
        data: Json,
        clear: Option<RemoveServerField>,
    },
    ServerDelete {
        id: String,
    },
    ServerMemberUpdate {
        id: ServerMemberUpdateId,
        data: Json,
        clear: Option<RemoveServerMemberField>,
    },
    ServerMemberJoin {
        id: String,
        #[serde(rename = "user")]
        user_id: String,
    },
    ServerMemberLeave {
        id: String,
        #[serde(rename = "user")]
        user_id: String,
    },
    ServerRoleUpdate {
        id: String,
        role_id: String,
        data: Json,
        clear: Option<RemoveServerRoleField>,
    },
    ServerRoleDelete {
        id: String,
        role_id: String,
    },
    UserUpdate {
        id: String,
        data: Json,
        clear: Option<RemoveUserField>,
    },
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct ServerMemberUpdateId {
    #[serde(rename = "server")]
    pub server_id: String,
    #[serde(rename = "user")]
    pub user_id: String,
}
