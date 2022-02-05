use {
    serde::Deserialize,
    std::fmt::{self, Display, Formatter},
};

use crate::entities::Id;

/// Message content type.
#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(untagged)]
pub enum Content {
    /// A user message.
    Text(String),
    /// A system message.
    SystemMessage(SystemMessage),
}

impl Display for Content {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Text(text) => write!(f, "{}", text),
            Self::SystemMessage(msg) => match msg {
                SystemMessage::Text { content } => write!(f, "{}", content),
                SystemMessage::UserAdded { .. } => write!(f, "User added to the channel."),
                SystemMessage::UserRemove { .. } => write!(f, "User removed from the channel."),
                SystemMessage::UserJoined { .. } => write!(f, "User joined the channel."),
                SystemMessage::UserLeft { .. } => write!(f, "User left the channel."),
                SystemMessage::UserKicked { .. } => write!(f, "User kicked from the channel."),
                SystemMessage::UserBanned { .. } => write!(f, "User banned from the channel."),
                SystemMessage::ChannelRenamed { .. } => write!(f, "Channel renamed."),
                SystemMessage::ChannelDescriptionChanged { .. } => {
                    write!(f, "Channel description changed.")
                }
                SystemMessage::ChannelIconChanged { .. } => write!(f, "Channel icon changed."),
            },
        }
    }
}

/// A system message.
#[allow(missing_docs)]
#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(tag = "type")]
pub enum SystemMessage {
    #[serde(rename = "text")]
    Text { content: String },
    #[serde(rename = "user_added")]
    UserAdded { id: Id, by: Id },
    #[serde(rename = "user_remove")]
    UserRemove { id: Id, by: Id },
    #[serde(rename = "user_joined")]
    UserJoined { id: Id },
    #[serde(rename = "user_left")]
    UserLeft { id: Id },
    #[serde(rename = "user_kicked")]
    UserKicked { id: Id },
    #[serde(rename = "user_banned")]
    UserBanned { id: Id },
    #[serde(rename = "channel_renamed")]
    ChannelRenamed { name: String, by: Id },
    #[serde(rename = "channel_description_changed")]
    ChannelDescriptionChanged { by: Id },
    #[serde(rename = "channel_icon_changed")]
    ChannelIconChanged { by: Id },
}
