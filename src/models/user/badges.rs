bitflags! {
    /// User badges.
    #[derive(Deserialize, Default)]
    #[serde(transparent)]
    pub struct Badges: u32 {
        /// Active or significant contributor to Revolt.
        const DEVELOPER = 1;
        /// Help translate Revolt.
        const TRANSLATOR = 2;
        /// Donate to Revolt.
        const SUPPORTER = 4;
        /// Helped discovered a security issue and responsibly disclosed it.
        const RESPONSIBLE_DISCLOSURE = 8;
        /// Founded Revolt.
        const FOUNDER = 16;
        /// Part of the platform moderation team.
        const PLATFORM_MODERATION = 32;
        /// Revolt active supporter.
        const ACTIVE_SUPPORTER = 64;
        /// It's a paw.
        const PAW = 128;
        /// Joined as one of the first 1k users.
        const EARLY_ADOPTER = 256;
        /// Whatever the funny joke is at any given time.
        const RESERVED_RELEVANT_JOKE_BADGE1 = 512;
    }
}
