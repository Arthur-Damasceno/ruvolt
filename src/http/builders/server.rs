use crate::models::{events::ServerField, Category, SystemMessageChannels};

/// A builder to create a server.
#[derive(Debug, Clone, Default, Serialize)]
pub struct CreateServer {
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,
    nsfw: bool,
}

impl CreateServer {
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

    /// Set whether the server is not safe for work.
    pub fn nsfw(mut self, nsfw: bool) -> Self {
        self.nsfw = nsfw;
        self
    }
}

/// A builder to edit a server.
#[derive(Debug, Clone, Default, Serialize)]
pub struct EditServer {
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    icon: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    banner: Option<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    categories: Vec<Category>,
    #[serde(skip_serializing_if = "Option::is_none")]
    system_messages: Option<SystemMessageChannels>,
    #[serde(skip_serializing_if = "Option::is_none")]
    analytics: Option<bool>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    remove: Vec<ServerField>,
}

impl EditServer {
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

    /// Set the attachment id to icon.
    pub fn icon(mut self, icon: impl Into<String>) -> Self {
        self.icon = Some(icon.into());
        self
    }

    /// Set the attachment id to banner.
    pub fn banner(mut self, banner: impl Into<String>) -> Self {
        self.banner = Some(banner.into());
        self
    }

    /// Set a category to include on the server.
    pub fn category(mut self, category: Category) -> Self {
        self.categories.push(category);
        self
    }

    /// Set the system message channels.
    pub fn system_messages(mut self, system_messages: SystemMessageChannels) -> Self {
        self.system_messages = Some(system_messages);
        self
    }

    /// Set whether analytics should be collected for the server.
    pub fn analytics(mut self, analytics: bool) -> Self {
        self.analytics = Some(analytics);
        self
    }

    /// Set the fields to remove from the server.
    pub fn remove(mut self, field: ServerField) -> Self {
        self.remove.push(field);
        self
    }
}

/// A builder to create a server channel.
#[derive(Debug, Clone, Default, Serialize)]
pub struct CreateChannel {
    #[serde(rename = "type")]
    kind: ChannelKind,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,
    nsfw: bool,
}

impl CreateChannel {
    /// Set the channel type.
    pub fn kind(mut self, kind: ChannelKind) -> Self {
        self.kind = kind;
        self
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

    /// Set whether the channel is not safe for work.
    pub fn nsfw(mut self, nsfw: bool) -> Self {
        self.nsfw = nsfw;
        self
    }
}

/// Channel type used to create a server channel.
#[derive(Debug, Clone, Copy, PartialEq, Serialize)]
pub enum ChannelKind {
    /// Text channel.
    Text,
    /// Voice channel.
    Voice,
}

impl Default for ChannelKind {
    fn default() -> Self {
        Self::Text
    }
}
