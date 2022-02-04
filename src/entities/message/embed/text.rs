use serde::Deserialize;

use crate::entities::Attachment;

/// A text embed.
#[allow(missing_docs)]
#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct TextEmbed {
    pub icon_url: Option<String>,
    pub url: Option<String>,
    pub title: Option<String>,
    pub description: Option<String>,
    pub media: Option<Attachment>,
    #[serde(rename = "colour")]
    pub color: Option<String>,
}
