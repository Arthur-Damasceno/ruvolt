pub use {special::*, text::*, website::*};

mod special;
mod text;
mod website;

/// A message embed.
#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(tag = "type")]
#[non_exhaustive]
pub enum Embed {
    /// Text type.
    Text(TextEmbed),
    /// Website type.
    Website(WebsiteEmbed),
    /// Image type.
    Image(EmbedImage),
    /// None type.
    None,
}
