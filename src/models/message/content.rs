use {
    serde::Deserialize,
    std::fmt::{self, Display, Formatter},
};

use crate::models::Id;

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
        write!(
            f,
            "{}",
            match self {
                Self::Text(text) => text,
                Self::SystemMessage(msg) => match msg {
                    SystemMessage::Text { content } => content,
                    SystemMessage::UserAdded { .. } => "User added to the channel.",
                    SystemMessage::UserRemove { .. } => "User removed from the channel.",
                    SystemMessage::UserJoined { .. } => "User joined the channel.",
                    SystemMessage::UserLeft { .. } => "User left the channel.",
                    SystemMessage::UserKicked { .. } => "User kicked from the channel.",
                    SystemMessage::UserBanned { .. } => "User banned from the channel.",
                    SystemMessage::ChannelRenamed { .. } => "Channel renamed.",
                    SystemMessage::ChannelDescriptionChanged { .. } =>
                        "Channel description changed.",
                    SystemMessage::ChannelIconChanged { .. } => "Channel icon changed.",
                },
            }
        )
    }
}

/// A system message.
#[allow(missing_docs)]
#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum SystemMessage {
    Text { content: String },
    UserAdded { id: Id, by: Id },
    UserRemove { id: Id, by: Id },
    UserJoined { id: Id },
    UserLeft { id: Id },
    UserKicked { id: Id },
    UserBanned { id: Id },
    ChannelRenamed { name: String, by: Id },
    ChannelDescriptionChanged { by: Id },
    ChannelIconChanged { by: Id },
}
