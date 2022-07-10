use crate::models::events::ChannelField;

/// Builder for create a server channel.
#[derive(Debug, Clone, Serialize)]
pub struct CreateChannel {
    #[serde(rename = "type")]
    kind: ChannelType,
    name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,
    nsfw: bool,
}

#[derive(Debug, Clone, Serialize)]
enum ChannelType {
    Text,
    Voice,
}

impl CreateChannel {
    /// Creates a new builder with `Text` channel type.
    pub fn text(name: impl Into<String>) -> Self {
        Self {
            kind: ChannelType::Text,
            name: name.into(),
            description: None,
            nsfw: false,
        }
    }

    /// Creates a new builder with `Voice` channel type.
    pub fn voice(name: impl Into<String>) -> Self {
        Self {
            kind: ChannelType::Voice,
            name: name.into(),
            description: None,
            nsfw: false,
        }
    }

    /// Set the description.
    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    /// Set whether channel is not safe for work.
    pub fn nsfw(mut self, nsfw: bool) -> Self {
        self.nsfw = nsfw;
        self
    }
}

/// Builder for edit a channel.
#[derive(Debug, Clone, Default, Serialize)]
pub struct EditChannel {
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    icon: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    nsfw: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    remove: Option<ChannelField>,
}

impl EditChannel {
    /// Creates a new builder.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the name.
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }

    /// Set the description.
    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    /// Set the icon.
    pub fn icon(mut self, id: impl Into<String>) -> Self {
        self.icon = Some(id.into());
        self
    }

    /// Set whether channel is not safe for work.
    pub fn nsfw(mut self, nsfw: bool) -> Self {
        self.nsfw = Some(nsfw);
        self
    }

    /// Set a channel field to remove.
    pub fn remove(mut self, field: ChannelField) -> Self {
        self.remove = Some(field);
        self
    }
}
