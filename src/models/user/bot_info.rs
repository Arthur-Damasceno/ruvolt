use serde::Deserialize;

#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct BotInfo {
    #[serde(rename = "owner")]
    pub owner_id: String,
}
