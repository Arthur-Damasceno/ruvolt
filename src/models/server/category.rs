/// A server category.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[non_exhaustive]
pub struct Category {
    /// Category id.
    pub id: String,
    /// Category title.
    pub title: String,
    /// Category channels ids.
    pub channels: Vec<String>,
}

impl Category {
    /// Creates a [Category].
    pub fn new(id: impl Into<String>, title: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            title: title.into(),
            channels: Vec::new(),
        }
    }
}
