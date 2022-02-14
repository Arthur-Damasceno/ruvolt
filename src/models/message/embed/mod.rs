pub use {special::*, text::*, website::*};

mod special;
mod text;
mod website;

use serde::Deserialize;

/// A message embed.
#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(tag = "type")]
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
