/// Message edition date.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[non_exhaustive]
pub struct MessageEdited {
    /// Edition date.
    #[serde(rename = "$date")]
    pub date: String,
}
