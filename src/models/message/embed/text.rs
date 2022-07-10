use crate::models::Attachment;

/// A text embed.
#[derive(Debug, Clone, PartialEq, Deserialize)]
#[non_exhaustive]
pub struct TextEmbed {
    /// Embed icon url.
    pub icon_url: Option<String>,
    /// Embed url.
    pub url: Option<String>,
    /// Embed title.
    pub title: Option<String>,
    /// Embed description.
    pub description: Option<String>,
    /// Embed media.
    pub media: Option<Attachment>,
    /// Embed color.
    #[serde(rename = "colour")]
    pub color: Option<String>,
}
