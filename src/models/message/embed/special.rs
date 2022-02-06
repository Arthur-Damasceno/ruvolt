#![allow(missing_docs)]

use serde::Deserialize;

/// A special 3rd party embed.
#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(tag = "type")]
pub enum EmbedSpecial {
    Youtube {
        id: String,
        timestamp: Option<String>,
    },
    Twitch {
        id: String,
        content_type: TwitchContentType,
    },
    Spotify {
        id: String,
        content_type: String,
    },
    Bandcamp {
        id: String,
        content_type: BandcampContentType,
    },
    Soundcloud,
    None,
}

/// Twitch content type.
#[derive(Debug, Deserialize, Clone, Copy, PartialEq)]
pub enum TwitchContentType {
    Channel,
    Video,
    Clip,
}

/// Bandcamp content type.
#[derive(Debug, Deserialize, Clone, Copy, PartialEq)]
pub enum BandcampContentType {
    Album,
    Track,
}
