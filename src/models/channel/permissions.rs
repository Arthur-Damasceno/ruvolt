use {serde::Deserialize, std::iter::Iterator};

/// Channel permissions count.
#[derive(Debug, Deserialize, Default, Clone, Copy, PartialEq)]
#[serde(transparent)]
pub struct ChannelPermissionsRaw(pub u32);

impl ChannelPermissionsRaw {
    /// Check if has [permission](ChannelPermission).
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use ruvolt::models::{ChannelPermissionsRaw, ChannelPermission};
    /// let permissions = ChannelPermissionsRaw(511);
    ///
    /// assert!(permissions.has(ChannelPermission::ManageChannel));
    /// ```
    pub fn has(&self, permission: ChannelPermission) -> bool {
        self.all().contains(&permission)
    }

    /// Check if has a list of [permissions](ChannelPermission).
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use ruvolt::models::{ChannelPermissionsRaw, ChannelPermission};
    /// let permissions = ChannelPermissionsRaw(259);
    ///
    /// assert!(permissions.has_all(&[
    ///     ChannelPermission::SendMessage,
    ///     ChannelPermission::Masquerade
    /// ]));
    /// ```
    pub fn has_all(&self, permissions: &[ChannelPermission]) -> bool {
        let all = self.all();

        for permission in permissions {
            if !all.contains(permission) {
                return false;
            }
        }

        true
    }

    /// All permissions.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use ruvolt::models::{ChannelPermissionsRaw, ChannelPermission};
    /// let permissions = ChannelPermissionsRaw(3);
    ///
    /// assert_eq!(permissions.all(), vec![ChannelPermission::SendMessage, ChannelPermission::View]);
    /// ```
    pub fn all(&self) -> Vec<ChannelPermission> {
        let mut count = self.0;
        let mut permissions = Vec::new();

        for permission in ChannelPermission::Masquerade {
            if count == 0 {
                return permissions;
            }

            if count >= permission as u32 {
                count -= permission as u32;
                permissions.push(permission);
            }
        }

        permissions
    }
}

/// Possible channel permissions.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ChannelPermission {
    /// View channel.
    View = 1,
    /// Send messages in the channel.
    SendMessage = 2,
    /// Manage channel messages.
    ManageMessages = 4,
    /// Manage channel.
    ManageChannel = 8,
    /// Voice call.
    VoiceCall = 16,
    /// Invite users to channel.
    InviteOthers = 32,
    /// Send embeds.
    EmbedLinks = 64,
    /// Upload files.
    UploadFiles = 128,
    /// Send masquerade messages.
    Masquerade = 256,
}

impl Iterator for ChannelPermission {
    type Item = Self;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            Self::View => {
                *self = Self::Masquerade;
                Some(Self::View)
            }
            Self::SendMessage => {
                *self = Self::View;
                Some(Self::SendMessage)
            }
            Self::ManageMessages => {
                *self = Self::SendMessage;
                Some(Self::ManageMessages)
            }
            Self::ManageChannel => {
                *self = Self::ManageMessages;
                Some(Self::ManageChannel)
            }
            Self::VoiceCall => {
                *self = Self::ManageChannel;
                Some(Self::VoiceCall)
            }
            Self::InviteOthers => {
                *self = Self::VoiceCall;
                Some(Self::InviteOthers)
            }
            Self::EmbedLinks => {
                *self = Self::InviteOthers;
                Some(Self::EmbedLinks)
            }
            Self::UploadFiles => {
                *self = Self::EmbedLinks;
                Some(Self::UploadFiles)
            }
            Self::Masquerade => {
                *self = Self::UploadFiles;
                Some(Self::Masquerade)
            }
        }
    }
}
