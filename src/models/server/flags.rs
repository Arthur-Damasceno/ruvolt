use {serde::Deserialize, std::iter::Iterator};

/// Server flags count.
#[derive(Debug, Deserialize, Default, Clone, Copy, PartialEq)]
pub struct ServerFlags(pub u32);

impl ServerFlags {
    /// Check if has a [flag](ServerFlag).
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use ruvolt::models::{ServerFlags, ServerFlag};
    /// let flags = ServerFlags(1);
    ///
    /// assert!(flags.has(ServerFlag::OfficialRevoltServer));
    /// ```
    pub fn has(&self, flag: ServerFlag) -> bool {
        self.all().contains(&flag)
    }

    /// Check if has a list of [flags](ServerFlag).
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use ruvolt::models::{ServerFlags, ServerFlag};
    /// let flags = ServerFlags(2);
    ///
    /// assert!(!flags.has_all(&vec![
    ///     ServerFlag::OfficialRevoltServer,
    ///     ServerFlag::VerifiedCommunityServer
    /// ]));
    /// ```
    pub fn has_all(&self, flags: &[ServerFlag]) -> bool {
        let all = self.all();

        for flag in flags {
            if !all.contains(flag) {
                return false;
            }
        }

        true
    }

    /// All flags.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use ruvolt::models::ServerFlags;
    /// let flags = ServerFlags(0);
    ///
    /// assert_eq!(flags.all(), vec![])
    /// ```
    pub fn all(&self) -> Vec<ServerFlag> {
        let mut count = self.0;
        let mut flags = Vec::new();

        for flag in ServerFlag::VerifiedCommunityServer {
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

/// Possible server flags.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ServerFlag {
    /// Official Revolt server.
    OfficialRevoltServer = 1,
    /// Verified community server.
    VerifiedCommunityServer = 2,
}

impl Iterator for ServerFlag {
    type Item = Self;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            Self::OfficialRevoltServer => {
                *self = Self::VerifiedCommunityServer;
                Some(Self::OfficialRevoltServer)
            }
            Self::VerifiedCommunityServer => {
                *self = Self::OfficialRevoltServer;
                Some(Self::VerifiedCommunityServer)
            }
        }
    }
}
