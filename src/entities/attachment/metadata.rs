use serde::Deserialize;

/// Attachment metadata.
#[allow(missing_docs)]
#[derive(Debug, Deserialize, Clone, Copy, PartialEq)]
#[serde(tag = "type")]
pub enum AttachmentMetadata {
    File,
    Text,
    Image { width: usize, height: usize },
    Video { width: usize, height: usize },
    Audio,
}
