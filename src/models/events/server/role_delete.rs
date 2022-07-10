/// A server role has been deleted.
#[derive(Debug, Clone, PartialEq, Deserialize)]
#[non_exhaustive]
pub struct ServerRoleDeleteEvent {
    /// Server id.
    #[serde(rename = "id")]
    pub server_id: String,
    /// Server role id.
    pub role_id: String,
}
