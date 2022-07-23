use chrono::{DateTime, Utc};

use crate::models::Attachment;

/// An embed.
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(tag = "type")]
pub enum Embed {
    /// Website metadata.
    Website {
        /// Direct url to web page.
        url: Option<String>,
        /// Remote content.
        special: Option<EmbedSpecial>,
        /// Title of website.
        title: Option<String>,
        /// Description of website.
        description: Option<String>,
        /// Embedded image.
        image: Option<EmbedImage>,
        /// Embedded video.
        video: Option<EmbedVideo>,
        /// Site name.
        site_name: Option<String>,
        /// Url to site icon.
        icon_url: Option<String>,
        /// CSS colour.
        colour: Option<String>,
    },
    /// Text type.
    Text {
        /// Url to icon.
        icon_url: Option<String>,
        /// Url for title.
        url: Option<String>,
        /// Title of text embed.
        title: Option<String>,
        /// Description of text embed.
        description: Option<String>,
        /// Attachment included in the embed.
        media: Option<Attachment>,
        /// CSS colour.
        colour: Option<String>,
    },
    /// Image type.
    Image(EmbedImage),
    /// Video type.
    Video(EmbedVideo),
    /// None type.
    None,
}

/// A embed video.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct EmbedVideo {
    /// Url to the original video.
    pub url: String,
    /// Width of the video.
    pub width: usize,
    /// Height of the video.
    pub height: usize,
}

/// A embed image.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct EmbedImage {
    /// Url to the original image.
    pub url: String,
    /// Width of the image.
    pub width: usize,
    /// Height of the image.
    pub height: usize,
    /// Positioning and size.
    pub size: EmbedImageSize,
}

/// Embed image positioning and size.
#[derive(Debug, Clone, Copy, PartialEq, Deserialize, Serialize)]
pub enum EmbedImageSize {
    /// Show large preview at the bottom of the embed.
    Large,
    /// Show small preview to the side of the embed.
    Preview,
}

/// Information about special remote content.
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(tag = "type")]
pub enum EmbedSpecial {
    /// No remote content.
    None,
    /// Content hint that this contains a GIF.
    #[serde(rename = "GIF")]
    Gif,
    /// YouTube video.
    YouTube {
        /// Id of the video.
        id: String,
        /// Video timestamp.
        timestamp: Option<DateTime<Utc>>,
    },
    /// Lightspeed.tv stream.
    Lightspeed {
        /// Id of the content.
        id: String,
        /// Type of the content.
        content_type: LightspeedContentType,
    },
    /// Twitch stream or clip.
    Twitch {
        /// Id of the content.
        id: String,
        /// Type of the content.
        content_type: TwitchContentType,
    },
    /// Spotify track.
    Spotify {
        /// Id of the content.
        id: String,
        /// Type of the content.
        content_type: String,
    },
    /// Soundcloud track.
    Soundcloud,
    /// Bandcamp track.
    Bandcamp {
        /// Id of the content.
        id: String,
        /// Type of the content.
        content_type: BandcampContentType,
    },
}

/// Lightspeed content type.
#[derive(Debug, Clone, Copy, PartialEq, Deserialize, Serialize)]
pub enum LightspeedContentType {
    /// Lightspeed channel.
    Channel,
}

/// Twitch content type.
#[derive(Debug, Clone, Copy, PartialEq, Deserialize, Serialize)]
pub enum TwitchContentType {
    /// Twitch channel.
    Channel,
    /// Twitch video.
    Video,
    /// Twitch clip.
    Clip,
}

/// Bandcamp content type.
#[derive(Debug, Clone, Copy, PartialEq, Deserialize, Serialize)]
pub enum BandcampContentType {
    /// Bandcamp album.
    Album,
    /// Bandcamp track.
    Track,
}
