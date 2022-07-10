/// A bot information.
#[derive(Debug, Clone, PartialEq, Deserialize)]
#[non_exhaustive]
pub struct BotInformation {
    /// Bot owner id.
    #[serde(rename = "owner")]
    pub owner_id: String,
}
