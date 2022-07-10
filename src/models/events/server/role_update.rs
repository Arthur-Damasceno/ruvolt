/// Specifies a field to remove on server role update.
#[derive(Debug, Clone, Copy, PartialEq, Deserialize)]
#[non_exhaustive]
pub enum RoleField {
    /// Role color.
    #[serde(rename = "Colour")]
    Color,
}

/// A server role details were updated.
#[derive(Debug, Clone, PartialEq, Deserialize)]
#[non_exhaustive]
pub struct ServerRoleUpdateEvent {
    /// Server id.
    #[serde(rename = "id")]
    pub server_id: String,
    /// Server role id.
    pub role_id: String,
    /// A partial server role object.
    pub data: serde_json::Value,
    /// A specified field to remove on server role update.
    pub clear: Option<RoleField>,
}
