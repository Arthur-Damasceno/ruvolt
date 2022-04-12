use crate::models::Id;

/// A server category.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[non_exhaustive]
pub struct Category {
    /// Category id.
    pub id: Id,
    /// Category title.
    pub title: String,
    /// Category channels ids.
    pub channels: Vec<Id>,
}

impl Category {
    /// Creates a new [Category].
    pub fn new(id: Id, title: impl Into<String>) -> Self {
        Self {
            id,
            title: title.into(),
            channels: Vec::new(),
        }
    }
}
