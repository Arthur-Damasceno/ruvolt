//! Mappings of objects received from the API, with helper methods.

#[doc(inline)]
pub use {attachment::*, channel::*, message::*, server::*};

mod attachment;
mod channel;
pub mod events;
mod message;
mod server;
pub mod user;

/// Re-exports of all [crate::models] sub-modules.
pub mod prelude {
    pub use super::events::*;
    pub use super::user::*;
}
