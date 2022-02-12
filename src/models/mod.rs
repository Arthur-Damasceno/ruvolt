//! Revolt API models.

#[doc(inline)]
pub use {channel::*, message::*, server::*, user::*};

mod channel;
pub mod events;
mod message;
mod server;
mod user;

/// Models id type.
pub type Id = String;
