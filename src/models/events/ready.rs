use crate::models::{user::User, Channel, Member, Server};

#[cfg(feature = "cache")]
use crate::{cache::UpdateCache, Context};

/// Bot is ready.
#[derive(Debug, Clone, Deserialize)]
#[non_exhaustive]
pub struct ReadyEvent {
    /// Users.
    pub users: Vec<User>,
    /// Servers.
    pub servers: Vec<Server>,
    /// Channels.
    pub channels: Vec<Channel>,
    /// Members.
    pub members: Vec<Member>,
}

#[cfg(feature = "cache")]
#[async_trait]
impl UpdateCache for ReadyEvent {
    async fn update(&self, cx: &Context) {
        let mut users = cx.cache.users.write().await;

        for user in &self.users {
            users.insert(user.id.clone(), user.clone());
        }

        let mut channels = cx.cache.channels.write().await;

        for channel in &self.channels {
            channels.insert(channel.id().into(), channel.clone());
        }

        let mut servers = cx.cache.servers.write().await;

        for server in &self.servers {
            servers.insert(server.id.clone(), server.clone());
        }

        let mut members = cx.cache.members.write().await;

        for member in &self.members {
            members.insert(member.id.clone(), member.clone());
        }
    }
}
