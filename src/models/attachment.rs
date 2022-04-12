use crate::models::Id;

/// An attachment like icons, avatars, banners or message attachments.
#[derive(Debug, Deserialize, Clone, PartialEq)]
#[non_exhaustive]
pub struct Attachment {
    /// Attachment id.
    #[serde(rename = "_id")]
    pub id: Id,
    /// Attachment tag.
    pub tag: AttachmentTag,
    /// Attachment file name.
    pub filename: String,
    /// Attachment metadata.
    pub metadata: AttachmentMetadata,
    /// Attachment size.
    pub size: usize,
    /// Attachment content type.
    pub content_type: String,
}

/// Attachment tag.
#[derive(Debug, Deserialize, Clone, Copy, PartialEq)]
#[serde(rename_all = "lowercase")]
#[non_exhaustive]
pub enum AttachmentTag {
    /// Attachments tag.
    Attachments,
    /// Avatars tag.
    Avatars,
    /// Backgrounds tag.
    Backgrounds,
    /// Icons tag.
    Icons,
    /// Banners tag.
    Banners,
}

/// Attachment metadata.
#[derive(Debug, Deserialize, Clone, Copy, PartialEq)]
#[serde(tag = "type")]
#[non_exhaustive]
pub enum AttachmentMetadata {
    /// File type.
    File,
    /// Text type.
    Text,
    /// Image type.
    Image {
        /// Image width.
        width: usize,
        /// Image height.
        height: usize,
    },
    /// Video type.
    Video {
        /// Video width.
        width: usize,
        /// Video height.
        height: usize,
    },
    /// Audio type.
    Audio,
}
