use crate::models::events::ChannelField;

/// A builder to create a group channel.
#[derive(Debug, Clone, Default, Serialize)]
pub struct CreateGroup {
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,
    users: Vec<String>,
    nsfw: bool,
}

impl CreateGroup {
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

    /// Set the users ids to add to the group.
    pub fn user(mut self, user: impl Into<String>) -> Self {
        self.users.push(user.into());
        self
    }

    /// Set whether the channel is not safe for work.
    pub fn nsfw(mut self, nsfw: bool) -> Self {
        self.nsfw = nsfw;
        self
    }
}

/// A builder to edit a channel.
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
    #[serde(skip_serializing_if = "Vec::is_empty")]
    remove: Vec<ChannelField>,
}

impl EditChannel {
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

    /// Set whether the channel is not safe for work.
    pub fn nsfw(mut self, nsfw: bool) -> Self {
        self.nsfw = Some(nsfw);
        self
    }

    /// Set the fields to remove from the channel.
    pub fn remove(mut self, field: ChannelField) -> Self {
        self.remove.push(field);
        self
    }
}
