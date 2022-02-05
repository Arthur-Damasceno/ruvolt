use serde::Deserialize;

use crate::entities::Id;

#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct BotInfo {
    #[serde(rename = "owner")]
    pub owner_id: Id,
}
