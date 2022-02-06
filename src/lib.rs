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
#[doc(hidden)]
pub use error::Result;

mod client;
pub mod error;
pub mod models;
