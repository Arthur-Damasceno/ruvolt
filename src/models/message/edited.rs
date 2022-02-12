use serde::Deserialize;

#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct Edited {
    #[serde(rename = "$date")]
    pub date: String,
}
