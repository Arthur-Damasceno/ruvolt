#![allow(missing_docs)]

use serde::Deserialize;

use super::EmbedSpecial;

/// A website embed.
#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct WebsiteEmbed {
    pub url: Option<String>,
    pub special: Option<EmbedSpecial>,
    pub title: Option<String>,
    pub description: Option<String>,
    pub image: Option<EmbedImage>,
    pub video: Option<EmbedVideo>,
    pub site_name: Option<String>,
    pub icon_url: Option<String>,
    #[serde(rename = "colour")]
    pub color: Option<String>,
}

/// An embedded video.
#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct EmbedVideo {
    pub url: String,
    pub width: usize,
    pub height: usize,
}

/// An embedded image.
#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct EmbedImage {
    pub url: String,
    pub width: usize,
    pub heigth: usize,
    pub size: EmbedImageSize,
}

/// Embedded image size.
#[derive(Debug, Deserialize, Clone, Copy, PartialEq)]
pub enum EmbedImageSize {
    /// `Large` size.
    Large,
    /// `Preview` size.
    Preview,
}
