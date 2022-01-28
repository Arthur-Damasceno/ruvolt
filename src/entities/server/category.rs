use serde::Deserialize;

/// A server category.
#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct Category {
    /// Category id.
    #[serde(rename = "_id")]
    pub id: String,
    /// Category title.
    pub title: String,
    /// Category channels ids.
    pub channels: Vec<String>,
}
