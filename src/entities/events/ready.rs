use serde::Deserialize;

use crate::entities::{Channel, Server, User};

/// Bot is ready.
#[allow(missing_docs)]
#[derive(Debug, Deserialize)]
pub struct ReadyEvent {
    pub users: Vec<User>,
    pub servers: Vec<Server>,
    pub channels: Vec<Channel>,
}
