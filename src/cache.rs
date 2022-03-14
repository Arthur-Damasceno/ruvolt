//! A cache containing data received from the API.
//!
//! ## Why use cache?
//! Using caching reduces latency to access data and allows you to avoid requests to the API.

use {std::collections::HashMap, tokio::sync::RwLock};

use crate::models::{Channel, Id, Member, MemberId, Server, User};

/// A cache containing data received from the API.
#[derive(Debug, Default)]
pub struct Cache {
    pub(crate) users: RwLock<HashMap<Id, User>>,
    pub(crate) channels: RwLock<HashMap<Id, Channel>>,
    pub(crate) servers: RwLock<HashMap<Id, Server>>,
    pub(crate) members: RwLock<HashMap<MemberId, Member>>,
}

impl Cache {
    /// Get a user from cache.
    pub async fn user(&self, id: &Id) -> Option<User> {
        self.users.read().await.get(id).cloned()
    }

    /// Get all users in cache.
    pub async fn users(&self) -> HashMap<Id, User> {
        self.users.read().await.clone()
    }

    /// Filter users in cache.
    pub async fn filter_users(&self, filter: impl Fn(&User) -> bool) -> Vec<User> {
        self.users
            .read()
            .await
            .iter()
            .filter_map(|(_, user)| {
                if filter(user) {
                    return Some(user);
                }

                None
            })
            .cloned()
            .collect()
    }

    /// Get a channel from cache.
    pub async fn channel(&self, id: &Id) -> Option<Channel> {
        self.channels.read().await.get(id).cloned()
    }

    /// Get all channels in cache.
    pub async fn channels(&self) -> HashMap<Id, Channel> {
        self.channels.read().await.clone()
    }

    /// Filter channels in cache.
    pub async fn filter_channels(&self, filter: impl Fn(&Channel) -> bool) -> Vec<Channel> {
        self.channels
            .read()
            .await
            .iter()
            .filter_map(|(_, channel)| {
                if filter(channel) {
                    return Some(channel);
                }

                None
            })
            .cloned()
            .collect()
    }

    /// Get a server from cache.
    pub async fn server(&self, id: &Id) -> Option<Server> {
        self.servers.read().await.get(id).cloned()
    }

    /// Get all servers in cache.
    pub async fn servers(&self) -> HashMap<Id, Server> {
        self.servers.read().await.clone()
    }

    /// Filter servers in cache.
    pub async fn filter_servers(&self, filter: impl Fn(&Server) -> bool) -> Vec<Server> {
        self.servers
            .read()
            .await
            .iter()
            .filter_map(|(_, server)| {
                if filter(server) {
                    return Some(server);
                }

                None
            })
            .cloned()
            .collect()
    }

    /// Returns the number of servers in cache.
    pub async fn servers_count(&self) -> usize {
        self.servers.read().await.len()
    }

    /// Get a member from cache.
    pub async fn member(&self, id: &MemberId) -> Option<Member> {
        self.members.read().await.get(id).cloned()
    }

    /// Get all members in cache.
    pub async fn members(&self) -> HashMap<MemberId, Member> {
        self.members.read().await.clone()
    }

    /// Filter members in cache.
    pub async fn filter_members(&self, filter: impl Fn(&Member) -> bool) -> Vec<Member> {
        self.members
            .read()
            .await
            .iter()
            .filter_map(|(_, member)| {
                if filter(member) {
                    return Some(member);
                }

                None
            })
            .cloned()
            .collect()
    }
}
