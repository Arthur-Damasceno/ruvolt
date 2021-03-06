#![deny(unsafe_code)]
#![warn(
    missing_debug_implementations,
    missing_copy_implementations,
    missing_docs,
    clippy::missing_panics_doc
)]

//! # [Ruvolt](crate)
//! ## An API wrapper for Revolt written in Rust.

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
#[cfg(feature = "state")]
pub mod state;

pub(crate) mod websocket;
