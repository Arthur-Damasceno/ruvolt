//! Module for all entities like [ClientToServerEvent], messages, server members, etc.

#[doc(inline)]
pub use {events::*, message::*};

mod events;
mod message;
