use serde::Deserialize;

use super::PermissionTuple;

/// Server role.
#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct Role {
    /// Role name.
    pub name: String,
    /// Role permissions.
    pub permissions: PermissionTuple,
    /// Role color.
    #[serde(rename = "colour")]
    pub color: Option<String>,
    /// Whether to display this role separately on the members list.
    #[serde(default)]
    pub hoist: bool,
    /// Role ranking.
    pub rank: Option<u32>,
}
