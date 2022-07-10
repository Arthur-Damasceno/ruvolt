#![deny(unsafe_code)]
#![warn(
    missing_debug_implementations,
    missing_copy_implementations,
    missing_docs,
    clippy::missing_panics_doc
)]

//! # [Ruvolt](crate)
//! ## An API wrapper for Revolt written in Rust.

#[macro_use]
extern crate serde;
#[macro_use]
extern crate bitflags;
#[macro_use]
extern crate async_trait;

#[doc(inline)]
pub use client::*;
mod client;

#[doc(hidden)]
pub use error::Result;
pub mod error;

pub mod builders;
#[cfg(feature = "cache")]
pub mod cache;
pub mod http;
pub mod models;
pub mod state;

pub(crate) mod websocket;
