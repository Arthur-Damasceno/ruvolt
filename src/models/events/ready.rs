use serde::Deserialize;

use crate::models::{Channel, Server, User};

/// Bot is ready.
#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct ReadyEvent {
    /// Users.
    pub users: Vec<User>,
    /// Servers.
    pub servers: Vec<Server>,
    /// Channels.
    pub channels: Vec<Channel>,
}
