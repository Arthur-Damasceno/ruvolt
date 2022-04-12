bitflags! {
    /// Server flags.
    #[derive(Deserialize, Default)]
    #[serde(transparent)]
    pub struct ServerFlags: u32 {
        /// Official Revolt server.
        const OFFICIAL_REVOLT_SERVER = 1;
        /// Verified community server.
        const VERIFIED_COMMUNITY_SERVER = 2;
    }
}
