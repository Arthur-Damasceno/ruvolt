use crate::models::EmbedSpecial;

/// A website embed.
#[derive(Debug, Deserialize, Clone, PartialEq)]
#[non_exhaustive]
pub struct WebsiteEmbed {
    /// Embed url.
    pub url: Option<String>,
    /// A special 3rd party embed.
    pub special: Option<EmbedSpecial>,
    /// Embed title.
    pub title: Option<String>,
    /// Embed description.
    pub description: Option<String>,
    /// Embed image.
    pub image: Option<EmbedImage>,
    /// Embed video.
    pub video: Option<EmbedVideo>,
    /// Embed site name.
    pub site_name: Option<String>,
    /// Embed icon url.
    pub icon_url: Option<String>,
    /// Embed color.
    #[serde(rename = "colour")]
    pub color: Option<String>,
}

/// A embedded video.
#[derive(Debug, Deserialize, Clone, PartialEq)]
#[non_exhaustive]
pub struct EmbedVideo {
    /// Embed video url.
    pub url: String,
    /// Embed video width.
    pub width: usize,
    /// Embed video height.
    pub height: usize,
}

/// A embedded image.
#[derive(Debug, Deserialize, Clone, PartialEq)]
#[non_exhaustive]
pub struct EmbedImage {
    /// Embed image url.
    pub url: String,
    /// Embed image width.
    pub width: usize,
    /// Embed image height.
    pub height: usize,
    /// Embed image size.
    pub size: EmbedImageSize,
}

/// Embed image size.
#[derive(Debug, Deserialize, Clone, Copy, PartialEq)]
#[non_exhaustive]
pub enum EmbedImageSize {
    /// Large size.
    Large,
    /// Preview size.
    Preview,
}
