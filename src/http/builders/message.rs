use crate::models::message::Masquerade;

/// A builder to create a message.
#[derive(Debug, Clone, Default, Serialize)]
pub struct CreateMessage {
    #[serde(skip_serializing_if = "Option::is_none")]
    content: Option<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    attachments: Vec<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    replies: Vec<Reply>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    embeds: Vec<CreateEmbed>,
    #[serde(skip_serializing_if = "Option::is_none")]
    masquerade: Option<Masquerade>,
}

impl CreateMessage {
    /// Set the content of the message.
    pub fn content(mut self, content: impl Into<String>) -> Self {
        self.content = Some(content.into());
        self
    }

    /// Set the id of an attachment to include in the message.
    pub fn attachment(mut self, attachment: impl Into<String>) -> Self {
        self.attachments.push(attachment.into());
        self
    }

    /// Set a message to reply.
    pub fn reply(mut self, id: impl Into<String>, mention: bool) -> Self {
        self.replies.push(Reply {
            id: id.into(),
            mention,
        });
        self
    }

    /// Set an embed to include in the message.
    pub fn embed(mut self, embed: CreateEmbed) -> Self {
        self.embeds.push(embed);
        self
    }

    /// Set the masquerade of the message.
    pub fn masquerade(mut self, masquerade: Masquerade) -> Self {
        self.masquerade = Some(masquerade);
        self
    }
}

#[derive(Debug, Clone, Serialize)]
struct Reply {
    id: String,
    mention: bool,
}

/// A builder to edit a message.
#[derive(Debug, Clone, Default, Serialize)]
pub struct EditMessage {
    #[serde(skip_serializing_if = "Option::is_none")]
    content: Option<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    embeds: Vec<CreateEmbed>,
}

impl EditMessage {
    /// Set the content of the message.
    pub fn content(mut self, content: impl Into<String>) -> Self {
        self.content = Some(content.into());
        self
    }

    /// Set an embed to include in the message.
    pub fn embed(mut self, embed: CreateEmbed) -> Self {
        self.embeds.push(embed);
        self
    }
}

/// A builder to create a embed.
#[derive(Debug, Clone, Default, Serialize)]
pub struct CreateEmbed {
    #[serde(skip_serializing_if = "Option::is_none")]
    icon_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    media: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    colour: Option<String>,
}

impl CreateEmbed {
    /// Set the url to the icon.
    pub fn icon_url(mut self, icon_url: impl Into<String>) -> Self {
        self.icon_url = Some(icon_url.into());
        self
    }

    /// Set the url to the title.
    pub fn url(mut self, url: impl Into<String>) -> Self {
        self.url = Some(url.into());
        self
    }

    /// Set the title.
    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

    /// Set the description.
    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    /// Set the attachment id to include in the embed.
    pub fn media(mut self, media: impl Into<String>) -> Self {
        self.media = Some(media.into());
        self
    }

    /// Set the colour.
    pub fn colour(mut self, colour: impl Into<String>) -> Self {
        self.colour = Some(colour.into());
        self
    }
}

/// A builder to fetch messages.
#[derive(Debug, Clone, Default, Serialize)]
pub struct FetchMessages {
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    before: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    after: Option<String>,
    sort: MessageSort,
    #[serde(skip_serializing_if = "Option::is_none")]
    nearby: Option<String>,
    include_users: bool,
}

impl FetchMessages {
    /// Set the maximum number of messages to fetch.
    pub fn limit(mut self, limit: u8) -> Self {
        self.limit = Some(limit);
        self
    }

    /// Set the message id before which messages should be fetched.
    pub fn before(mut self, before: impl Into<String>) -> Self {
        self.before = Some(before.into());
        self
    }

    /// Set the message id after which messages should be fetched.
    pub fn after(mut self, after: impl Into<String>) -> Self {
        self.after = Some(after.into());
        self
    }

    /// Set the message sort direction.
    pub fn sort(mut self, sort: MessageSort) -> Self {
        self.sort = sort;
        self
    }

    /// Set the message id to search around.
    pub fn nearby(mut self, nearby: impl Into<String>) -> Self {
        self.nearby = Some(nearby.into());
        self
    }

    /// Set whether to include users and members.
    pub fn include_users(mut self, include_users: bool) -> Self {
        self.include_users = include_users;
        self
    }
}

/// A builder to search for messages.
#[derive(Debug, Clone, Default, Serialize)]
pub struct SearchMessages {
    #[serde(skip_serializing_if = "Option::is_none")]
    query: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    before: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    after: Option<String>,
    sort: MessageSort,
    include_users: bool,
}

impl SearchMessages {
    /// Set the search query.
    pub fn query(mut self, query: impl Into<String>) -> Self {
        self.query = Some(query.into());
        self
    }

    /// Set the maximum number of messages to fetch.
    pub fn limit(mut self, limit: u8) -> Self {
        self.limit = Some(limit);
        self
    }

    /// Set the message id before which messages should be fetched.
    pub fn before(mut self, before: impl Into<String>) -> Self {
        self.before = Some(before.into());
        self
    }

    /// Set the message id after which messages should be fetched.
    pub fn after(mut self, after: impl Into<String>) -> Self {
        self.after = Some(after.into());
        self
    }

    /// Set the message sort direction.
    pub fn sort(mut self, sort: MessageSort) -> Self {
        self.sort = sort;
        self
    }

    /// Set whether to include users and members.
    pub fn include_users(mut self, include_users: bool) -> Self {
        self.include_users = include_users;
        self
    }
}

/// Sort used for retrieving messages.
#[derive(Debug, Clone, Copy, PartialEq, Serialize)]
pub enum MessageSort {
    /// Sort by relevance.
    Relevance,
    /// Sort by latest.
    Latest,
    /// Sort by oldest.
    Oldest,
}

impl Default for MessageSort {
    fn default() -> Self {
        Self::Latest
    }
}
