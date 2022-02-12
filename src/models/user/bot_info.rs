use serde::Deserialize;

use crate::models::Id;

#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct BotInfo {
    #[serde(rename = "owner")]
    pub owner_id: Id,
}
