#![allow(missing_docs)]

/// A special 3rd party embed.
#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(tag = "type")]
#[non_exhaustive]
pub enum EmbedSpecial {
    /// Youtube type.
    Youtube {
        id: String,
        timestamp: Option<String>,
    },
    /// Twitch type.
    Twitch {
        id: String,
        content_type: TwitchContentType,
    },
    /// Spotify type.
    Spotify { id: String, content_type: String },
    /// Bandcamp type.
    Bandcamp {
        id: String,
        content_type: BandcampContentType,
    },
    /// Soundcloud type.
    Soundcloud,
    /// None type.
    None,
}

/// Twitch content type.
#[derive(Debug, Clone, Copy, PartialEq, Deserialize)]
#[non_exhaustive]
pub enum TwitchContentType {
    Channel,
    Video,
    Clip,
}

/// Bandcamp content type.
#[derive(Debug, Clone, Copy, PartialEq, Deserialize)]
#[non_exhaustive]
pub enum BandcampContentType {
    Album,
    Track,
}
