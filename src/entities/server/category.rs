use serde::Deserialize;

use crate::entities::Id;

/// A server category.
#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct Category {
    /// Category id.
    pub id: Id,
    /// Category title.
    pub title: String,
    /// Category channels ids.
    pub channels: Vec<Id>,
}
