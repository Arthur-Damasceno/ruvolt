use {serde::Deserialize, std::iter::Iterator};

/// User flags count.
#[derive(Debug, Deserialize, Default, Clone, Copy, PartialEq)]
#[serde(transparent)]
pub struct UserFlagsRaw(pub u32);

impl UserFlagsRaw {
    /// Check if has a [flag](UserFlag).
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use ruvolt::models::{UserFlagsRaw, UserFlag};
    /// let flags = UserFlagsRaw(2);
    ///
    /// assert!(flags.has(UserFlag::Deleted));
    /// ```
    pub fn has(&self, flag: UserFlag) -> bool {
        self.all().contains(&flag)
    }

    /// Check if has a list of [flags](UserFlag).
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use ruvolt::models::{UserFlagsRaw, UserFlag};
    /// let flags = UserFlagsRaw(6);
    ///
    /// assert!(flags.has_all(&[UserFlag::Deleted, UserFlag::Banned]));
    /// ```
    pub fn has_all(&self, flags: &[UserFlag]) -> bool {
        let all = self.all();

        for flag in flags {
            if !all.contains(flag) {
                return false;
            }
        }

        true
    }

    /// All [flags](UserFlag).
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use ruvolt::models::{UserFlagsRaw, UserFlag};
    /// let flags = UserFlagsRaw(1);
    ///
    /// assert_eq!(flags.all(), vec![UserFlag::Suspended])
    /// ```
    pub fn all(&self) -> Vec<UserFlag> {
        let mut count = self.0;
        let mut flags = Vec::new();

        for flag in UserFlag::Banned {
            if count == 0 {
                return flags;
            }

            if count >= flag as u32 {
                count -= flag as u32;
                flags.push(flag);
            }
        }

        flags
    }
}

/// Possible user flags.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum UserFlag {
    /// User account is suspended.
    Suspended = 1,
    /// User account was deleted.
    Deleted = 2,
    /// User account is banned.
    Banned = 4,
}

impl Iterator for UserFlag {
    type Item = Self;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            Self::Suspended => {
                *self = Self::Banned;
                Some(Self::Suspended)
            }
            Self::Deleted => {
                *self = Self::Suspended;
                Some(Self::Deleted)
            }
            Self::Banned => {
                *self = Self::Deleted;
                Some(Self::Banned)
            }
        }
    }
}
