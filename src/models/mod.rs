//! Revolt API models.

#[doc(inline)]
pub use {attachment::*, channel::*, message::*, server::*, user::*};

mod attachment;
mod channel;
pub mod events;
mod message;
mod server;
mod user;

/// Model id type.
pub type Id = String;
