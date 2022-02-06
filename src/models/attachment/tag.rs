use serde::Deserialize;

/// Attachment tag.
#[allow(missing_docs)]
#[derive(Debug, Deserialize, Clone, Copy, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum AttachmentTag {
    Attachments,
    Avatars,
    Backgrounds,
    Icons,
    Banners,
}
