use {serde::Deserialize, std::iter::Iterator};

/// Server permissions count.
#[derive(Debug, Deserialize, Default, Clone, Copy, PartialEq)]
#[serde(transparent)]
pub struct ServerPermissionsRaw(pub u32);

impl ServerPermissionsRaw {
    /// Check if has [permission](ServerPermission).
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use ruvolt::models::{ServerPermissionsRaw, ServerPermission};
    /// let permissions = ServerPermissionsRaw(55);
    ///
    /// assert!(permissions.has(ServerPermission::BanMembers));
    /// ```
    pub fn has(&self, permission: ServerPermission) -> bool {
        self.all().contains(&permission)
    }

    /// Check if has a list of [permissions](ServerPermission).
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use ruvolt::models::{ServerPermissionsRaw, ServerPermission};
    /// let permissions = ServerPermissionsRaw(61501);
    ///
    /// assert!(permissions.has_all(&[ServerPermission::KickMembers, ServerPermission::BanMembers]));
    /// ```
    pub fn has_all(&self, permissions: &[ServerPermission]) -> bool {
        let all = self.all();

        for permission in permissions {
            if !all.contains(permission) {
                return false;
            }
        }

        true
    }

    /// All [permissions](ServerPermission).
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use ruvolt::models::{ServerPermissionsRaw, ServerPermission};
    /// let permissions = ServerPermissionsRaw(5);
    ///
    /// assert_eq!(permissions.all(), vec![ServerPermission::ManageChannels, ServerPermission::View]);
    /// ```
    pub fn all(&self) -> Vec<ServerPermission> {
        let mut count = self.0;
        let mut permissions = Vec::new();

        for permission in ServerPermission::RemoveAvatars {
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

/// Possible server permissions.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ServerPermission {
    /// View the server.
    View = 1,
    /// Manage server roles.
    ManageRoles = 2,
    /// Manage server channels.
    ManageChannels = 4,
    /// Manage the server.
    ManageServer = 8,
    /// Kick members from server.
    KickMembers = 16,
    /// Ban members from server.
    BanMembers = 32,
    /// Change members nicknames.
    ChangeNickname = 4096,
    /// Manage members nicknames.
    ManageNicknames = 8192,
    /// Change members avatars.
    ChangeAvatar = 16382,
    /// Remove members avatars.
    RemoveAvatars = 32768,
}

impl Iterator for ServerPermission {
    type Item = Self;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            Self::View => {
                *self = Self::RemoveAvatars;
                Some(Self::View)
            }
            Self::ManageRoles => {
                *self = Self::View;
                Some(Self::ManageRoles)
            }
            Self::ManageChannels => {
                *self = Self::ManageRoles;
                Some(Self::ManageChannels)
            }
            Self::ManageServer => {
                *self = Self::ManageChannels;
                Some(Self::ManageServer)
            }
            Self::KickMembers => {
                *self = Self::ManageServer;
                Some(Self::KickMembers)
            }
            Self::BanMembers => {
                *self = Self::KickMembers;
                Some(Self::BanMembers)
            }
            Self::ChangeNickname => {
                *self = Self::BanMembers;
                Some(Self::ChangeNickname)
            }
            Self::ManageNicknames => {
                *self = Self::ChangeNickname;
                Some(Self::ManageNicknames)
            }
            Self::ChangeAvatar => {
                *self = Self::ManageNicknames;
                Some(Self::ChangeAvatar)
            }
            Self::RemoveAvatars => {
                *self = Self::ChangeAvatar;
                Some(Self::RemoveAvatars)
            }
        }
    }
}
