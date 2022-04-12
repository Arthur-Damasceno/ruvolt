use crate::models::Id;

#[derive(Debug, Serialize, Clone, PartialEq)]
#[serde(tag = "type")]
pub enum ClientEvent {
    Authenticate {
        token: String,
    },
    BeginTyping {
        #[serde(rename = "channel")]
        channel_id: Id,
    },
    EndTyping {
        #[serde(rename = "channel")]
        channel_id: Id,
    },
    Ping {
        data: usize,
    },
}
