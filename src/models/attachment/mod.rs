pub use {metadata::*, tag::*};

mod metadata;
mod tag;

use serde::Deserialize;

use crate::models::Id;

/// An attachment like icons, avatars, banners or message attachments.
#[derive(Debug, Deserialize, Clone, PartialEq)]
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
