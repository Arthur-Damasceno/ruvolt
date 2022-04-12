use crate::models::Id;

/// A bot information.
#[derive(Debug, Deserialize, Clone, PartialEq)]
#[non_exhaustive]
pub struct BotInformation {
    /// Bot owner id.
    #[serde(rename = "owner")]
    pub owner_id: Id,
}
