use {serde::Deserialize, std::iter::Iterator};

/// User badges count.
#[derive(Debug, Deserialize, Default, Clone, Copy, PartialEq)]
#[serde(transparent)]
pub struct BadgesRaw(pub u32);

impl BadgesRaw {
    /// Check if has a [badge](Badge).
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use ruvolt::models::{BadgesRaw, Badge};
    /// let badges = BadgesRaw(19);
    ///
    /// assert!(badges.has(Badge::Founder));
    /// ```
    pub fn has(&self, badge: Badge) -> bool {
        self.all().contains(&badge)
    }

    /// Check if has a list of [badges](Badge).
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use ruvolt::models::{BadgesRaw, Badge};
    /// let badges = BadgesRaw(530);
    ///
    /// assert!(badges.has_all(&[Badge::ReservedRelevantJokeBadge1, Badge::Translator]));
    /// ```
    pub fn has_all(&self, badges: &[Badge]) -> bool {
        let all = self.all();

        for badge in badges {
            if !all.contains(badge) {
                return false;
            }
        }

        true
    }

    /// All [badges](Badge).
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use ruvolt::models::{BadgesRaw, Badge};
    /// let badges = BadgesRaw(3);
    ///
    /// assert_eq!(badges.all(), vec![Badge::Translator, Badge::Developer])
    /// ```
    pub fn all(&self) -> Vec<Badge> {
        let mut count = self.0;
        let mut badges = Vec::new();

        for badge in Badge::ReservedRelevantJokeBadge1 {
            if count == 0 {
                return badges;
            }

            if count >= badge as u32 {
                count -= badge as u32;
                badges.push(badge);
            }
        }

        badges
    }
}

/// Possible user badges.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Badge {
    /// Active or significant contributor to Revolt.
    Developer = 1,
    /// Help translate Revolt.
    Translator = 2,
    /// Donate to Revolt.
    Supporter = 4,
    /// Helped discovered a security issue and responsibly disclosed it.
    ResponsibleDisclosure = 8,
    /// Founded Revolt.
    Founder = 16,
    /// Part of the platform moderation team.
    PlatformModeration = 32,
    /// Revolt active supporter.
    ActiveSupporter = 64,
    /// It's a paw.
    Paw = 128,
    /// Joined as one of the first 1k users.
    EarlyAdopter = 256,
    /// Whatever the funny joke is at any given time.
    ReservedRelevantJokeBadge1 = 512,
}

impl Iterator for Badge {
    type Item = Self;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            Self::Developer => {
                *self = Self::ReservedRelevantJokeBadge1;
                Some(Self::Developer)
            }
            Self::Translator => {
                *self = Self::Developer;
                Some(Self::Translator)
            }
            Self::Supporter => {
                *self = Self::Translator;
                Some(Self::Supporter)
            }
            Self::ResponsibleDisclosure => {
                *self = Self::Supporter;
                Some(Self::ResponsibleDisclosure)
            }
            Self::Founder => {
                *self = Self::ResponsibleDisclosure;
                Some(Self::Founder)
            }
            Self::PlatformModeration => {
                *self = Self::EarlyAdopter;
                Some(Self::PlatformModeration)
            }
            Self::ActiveSupporter => {
                *self = Self::PlatformModeration;
                Some(Self::ActiveSupporter)
            }
            Self::Paw => {
                *self = Self::Founder;
                Some(Self::Paw)
            }
            Self::EarlyAdopter => {
                *self = Self::Paw;
                Some(Self::EarlyAdopter)
            }
            Self::ReservedRelevantJokeBadge1 => {
                *self = Self::EarlyAdopter;
                Some(Self::ReservedRelevantJokeBadge1)
            }
        }
    }
}
