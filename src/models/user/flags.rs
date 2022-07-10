bitflags! {
    /// User flags.
    #[derive(Default, Deserialize)]
    #[serde(transparent)]
    pub struct UserFlags: u32 {
        /// User account is suspended.
        const SUSPENDED = 1;
        /// User account was deleted.
        const DELETED = 2;
        /// User account is banned.
        const BANNED = 4;
    }
}
