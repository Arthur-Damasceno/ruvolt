use std::fmt::{Display, Formatter, Result};

/// Message content type.
#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(untagged)]
#[non_exhaustive]
pub enum Content {
    /// A user message.
    Text(String),
    /// A system message.
    SystemMessage(SystemMessage),
}

impl Display for Content {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        use SystemMessage::*;

        write!(
            f,
            "{}",
            match self {
                Self::Text(text) => text,
                Self::SystemMessage(Text { content }) => content,
                Self::SystemMessage(UserAdded { .. }) => "User added to the channel.",
                Self::SystemMessage(UserRemove { .. }) => "User removed from the channel.",
                Self::SystemMessage(UserJoined { .. }) => "User joined the channel.",
                Self::SystemMessage(UserLeft { .. }) => "User left the channel.",
                Self::SystemMessage(UserKicked { .. }) => "User kicked from the channel.",
                Self::SystemMessage(UserBanned { .. }) => "User banned from the channel.",
                Self::SystemMessage(ChannelRenamed { .. }) => "Channel renamed.",
                Self::SystemMessage(ChannelDescriptionChanged { .. }) =>
                    "Channel description changed.",
                Self::SystemMessage(ChannelIconChanged { .. }) => "Channel icon changed.",
            }
        )
    }
}

/// A system message.
#[allow(missing_docs)]
#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
#[non_exhaustive]
pub enum SystemMessage {
    Text { content: String },
    UserAdded { id: String, by: String },
    UserRemove { id: String, by: String },
    UserJoined { id: String },
    UserLeft { id: String },
    UserKicked { id: String },
    UserBanned { id: String },
    ChannelRenamed { name: String, by: String },
    ChannelDescriptionChanged { by: String },
    ChannelIconChanged { by: String },
}
