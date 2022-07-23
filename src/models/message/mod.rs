//! Message-related models.

mod embed;

pub use embed::*;

use chrono::{DateTime, Utc};

use crate::{
    http::builders::{CreateMessage, EditMessage},
    models::Attachment,
    Context, Result,
};

/// A message.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Message {
    /// Unique id.
    #[serde(rename = "_id")]
    pub id: String,
    /// Id of the channel this message was sent in.
    #[serde(rename = "channel")]
    pub channel_id: String,
    /// Id of the user that sent this message.
    #[serde(rename = "author")]
    pub author_id: String,
    /// Message content.
    pub content: Option<String>,
    /// System message.
    pub system: Option<SystemMessage>,
    /// When this message was last edited.
    pub edited: Option<DateTime<Utc>>,
    /// Embeds included in the message.
    #[serde(default)]
    pub embeds: Vec<Embed>,
    /// Attachments included in the message.
    #[serde(default)]
    pub attachments: Vec<Attachment>,
    /// Ids of the users mentioned in the message.
    #[serde(default)]
    pub mentions: Vec<String>,
    /// Ids of the messages that is replying to.
    #[serde(default)]
    pub replies: Vec<String>,
    /// Masquerade displayed for the message.
    pub masquerade: Option<Masquerade>,
}

impl Message {
    /// Acknowledge the message as read.
    pub async fn acknowledge(&self, ctx: &Context) -> Result {
        ctx.http
            .acknowledge_message(&self.channel_id, &self.id)
            .await
    }

    /// Edit the message.
    pub async fn edit(&self, ctx: &Context, data: impl Fn(EditMessage) -> EditMessage) -> Result {
        ctx.http
            .edit_message(&self.channel_id, &self.id, &data(Default::default()))
            .await
            .map(|_| ())
    }

    /// Delete the message.
    pub async fn delete(&self, ctx: &Context) -> Result {
        ctx.http.delete_message(&self.channel_id, &self.id).await
    }

    /// Reply the message.
    pub async fn reply(
        &self,
        ctx: &Context,
        data: impl Fn(CreateMessage) -> CreateMessage,
        mention: bool,
    ) -> Result<Self> {
        ctx.http
            .send_message(
                &self.channel_id,
                &data(Default::default()).reply(&self.id, mention),
            )
            .await
    }
}

/// A system message.
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum SystemMessage {
    /// A general text message.
    Text {
        /// Message content.
        content: String,
    },
    /// A user has been added to a group.
    UserAdded {
        /// Id of the user that was added.
        id: String,
        /// Id of the user who added the other.
        by: String,
    },
    /// A user has been removed from a group.
    UserRemove {
        /// Id of the user that was removed.
        id: String,
        /// Id of the user who removed the other.
        by: String,
    },
    /// A user joined a group.
    UserJoined {
        /// Id of the user who joined the group.
        id: String,
    },
    /// A user has left a group.
    UserLeft {
        /// Id of the user who left the group.
        id: String,
    },
    /// A user has been kicked from a server.
    UserKicked {
        /// Id of the user who has been kicked.
        id: String,
    },
    /// A user has been banned from a server.
    UserBanned {
        /// Id of the user who has been banned.
        id: String,
    },
    /// A channel has been renamed.
    ChannelRenamed {
        /// The name for which the channel was renamed.
        name: String,
        /// Id of the user who renamed the channel.
        by: String,
    },
    /// The description of a channel has changed.
    ChannelDescriptionChanged {
        /// Id of the user who changed the description.
        by: String,
    },
    /// The icon of a channel has changed.
    ChannelIconChanged {
        /// Id of the user who changed the icon.
        by: String,
    },
    /// The ownership of a channel has changed.
    ChannelOwnershipChanged {
        /// Id of the former owner of the channel.
        from: String,
        /// Id of the new channel owner.
        to: String,
    },
}

/// Masquerade displayed for a message.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Masquerade {
    /// Replace the display name shown on the message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Replace the avatar shown on the message (Url to image file).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avatar: Option<String>,
    /// Replace the display role colour shown on the message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub colour: Option<String>,
}
