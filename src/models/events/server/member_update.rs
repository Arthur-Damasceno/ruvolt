use crate::models::{Attachment, MemberId};

#[cfg(feature = "cache")]
use crate::{cache::UpdateCache, Context};

/// Specifies a field to remove on server member update.
#[derive(Debug, Clone, Copy, PartialEq, Deserialize, Serialize)]
#[non_exhaustive]
pub enum MemberField {
    /// Server member nickname.
    Nickname,
    /// Server member avatar.
    Avatar,
}

/// A server member details were updated.
#[derive(Debug, Clone, PartialEq, Deserialize)]
#[non_exhaustive]
pub struct ServerMemberUpdateEvent {
    /// Member id.
    #[serde(rename = "id")]
    pub member_id: MemberId,
    /// A partial server member.
    pub data: PartialMember,
    /// A specified field to remove on server member update.
    pub clear: Option<MemberField>,
}

/// A partial server member.
#[derive(Debug, Clone, PartialEq, Deserialize)]
#[non_exhaustive]
pub struct PartialMember {
    /// Member nickname.
    pub nickname: Option<String>,
    /// Member avatar.
    pub avatar: Option<Attachment>,
    /// Member roles.
    #[serde(default)]
    pub roles: Vec<String>,
}

#[cfg(feature = "cache")]
#[async_trait]
impl UpdateCache for ServerMemberUpdateEvent {
    async fn update(&self, cx: &Context) {
        if let Some(member) = cx.cache.members.write().await.get_mut(&self.member_id) {
            if let Some(field) = self.clear {
                match field {
                    MemberField::Nickname => member.nickname = None,
                    MemberField::Avatar => member.avatar = None,
                }
            }

            if let Some(ref nickname) = self.data.nickname {
                member.nickname = Some(nickname.clone());
            }

            if let Some(ref avatar) = self.data.avatar {
                member.avatar = Some(avatar.clone());
            }

            if !self.data.roles.is_empty() {
                member.roles = self.data.roles.clone();
            }
        }
    }
}
