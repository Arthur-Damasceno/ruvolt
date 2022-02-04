pub use {special::*, text::*, website::*};

mod special;
mod text;
mod website;

use serde::Deserialize;

/// A message embed.
#[allow(missing_docs)]
#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(tag = "type")]
pub enum Embed {
    Text(TextEmbed),
    Website(WebsiteEmbed),
    Image(EmbedImage),
    None,
}
