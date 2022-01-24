//! Module for [Error] and [Result] types.

use std::{
    error::Error as StdError,
    fmt::{Display, Formatter, Result as FmtResult},
    result::Result as StdResult,
};

/// [Result](StdResult) type for [Error].
pub type Result<T = (), E = Error> = StdResult<T, E>;

/// Errors that can happen when using [ruvolt](crate).
#[derive(Debug, Clone, Copy)]
pub enum Error {}

impl Display for Error {
    fn fmt(&self, _: &mut Formatter<'_>) -> FmtResult {
        todo!()
    }
}

impl StdError for Error {}
