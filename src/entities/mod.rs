//! Revolt API entities.

#[doc(inline)]
pub use {attachment::*, channel::*, message::*, server::*, user::*};

mod attachment;
mod channel;
pub mod events;
mod message;
mod server;
mod user;

/// Entity id type.
pub type Id = String;
