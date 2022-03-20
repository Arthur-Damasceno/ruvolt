use serde::{Deserialize, Serialize};

/// Message edition date.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct MessageEdited {
    /// Edition date.
    #[serde(rename = "$date")]
    pub date: String,
}
