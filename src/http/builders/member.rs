use crate::models::events::MemberField;

/// A builder to edit a member.
#[derive(Debug, Clone, Default, Serialize)]
pub struct EditMember {
    #[serde(skip_serializing_if = "Option::is_none")]
    nickname: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    avatar: Option<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    roles: Vec<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    remove: Vec<MemberField>,
}

impl EditMember {
    /// Set the nickname.
    pub fn nickname(mut self, nickname: impl Into<String>) -> Self {
        self.nickname = Some(nickname.into());
        self
    }

    /// Set the attachment id to avatar.
    pub fn avatar(mut self, avatar: impl Into<String>) -> Self {
        self.avatar = Some(avatar.into());
        self
    }

    /// Set the role id to give to the member.
    pub fn role(mut self, role: impl Into<String>) -> Self {
        self.roles.push(role.into());
        self
    }

    /// Set the fields to remove from the member.
    pub fn remove(mut self, field: MemberField) -> Self {
        self.remove.push(field);
        self
    }
}
