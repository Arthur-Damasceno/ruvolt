use crate::models::{
    events::{MemberField, ServerField},
    Category, SystemMessageChannels,
};

/// Builder for edit a server.
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
    nsfw: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    remove: Option<ServerField>,
}

impl EditServer {
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

    /// Set the banner.
    pub fn banner(mut self, id: impl Into<String>) -> Self {
        self.banner = Some(id.into());
        self
    }

    /// Add a category.
    pub fn category(mut self, category: &Category) -> Self {
        self.categories.push(category.clone());
        self
    }

    /// Set the system message channels.
    pub fn system_messages(
        mut self,
        build: impl Fn(SystemMessageChannels) -> SystemMessageChannels,
    ) -> Self {
        self.system_messages = Some(build(SystemMessageChannels::default()));
        self
    }

    /// Set whether server is not safe for work.
    pub fn nsfw(mut self, nsfw: bool) -> Self {
        self.nsfw = Some(nsfw);
        self
    }

    /// Set a server field to remove.
    pub fn remove(mut self, field: ServerField) -> Self {
        self.remove = Some(field);
        self
    }
}

/// Builder for edit a member.
#[derive(Debug, Clone, Default, Serialize)]
pub struct EditMember {
    #[serde(skip_serializing_if = "Option::is_none")]
    nickname: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    avatar: Option<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    roles: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    remove: Option<MemberField>,
}

impl EditMember {
    /// Creates a new builder.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the nickname.
    pub fn nickname(mut self, nickname: impl Into<String>) -> Self {
        self.nickname = Some(nickname.into());
        self
    }

    /// Set the avatar.
    pub fn avatar(mut self, id: impl Into<String>) -> Self {
        self.avatar = Some(id.into());
        self
    }

    /// Include a role.
    pub fn role(mut self, id: impl Into<String>) -> Self {
        self.roles.push(id.into());
        self
    }

    /// Set a member field to remove.
    pub fn remove(mut self, field: MemberField) -> Self {
        self.remove = Some(field);
        self
    }
}
