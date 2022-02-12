use serde::Deserialize;

use crate::models::{Channel, Server, User};

/// Bot is ready.
#[allow(missing_docs)]
#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct ReadyEvent {
    pub users: Vec<User>,
    pub servers: Vec<Server>,
    pub channels: Vec<Channel>,
}
