use serde::Deserialize;

/// A server.
#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct Server {
    /// Server id.
    #[serde(rename = "_id")]
    pub id: String,
    /// Server owner id.
    #[serde(rename = "owner")]
    pub owner_id: String,
    /// Server name.
    pub name: String,
    /// Server description.
    pub description: Option<String>,
    /// Server channels ids.
    pub channels: Vec<String>,
    /// Server flags.
    pub flags: Option<u32>,
    /// Server is not safe for work.
    #[serde(default)]
    pub nsfw: bool,
}
