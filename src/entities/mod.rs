//! Module for all entities like [ClientToServerEvent], messages, server members, etc.

#[doc(inline)]
pub use events::ClientToServerEvent;
pub(crate) use events::ServerToClientEvent;

mod events;
