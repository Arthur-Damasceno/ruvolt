//! Module for [Error] and [Result] types.

use {
    reqwest::Error as HttpError,
    std::{
        error::Error as StdError,
        fmt::{Display, Formatter, Result as FmtResult},
        result::Result as StdResult,
    },
    tokio_tungstenite::tungstenite::Error as WsError,
};

/// [Result](StdResult) type for [Error].
pub type Result<T = (), E = Error> = StdResult<T, E>;

/// Errors that can happen when using [ruvolt](crate).
#[derive(Debug)]
pub enum Error {
    /// Http requests error.
    Http(HttpError),
    /// WebSocket error.
    Ws(WsError),
    /// Unknown or unexpected error.
    Unknown(String),
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match *self {
            Self::Http(ref err) => err.fmt(f),
            Self::Ws(ref err) => err.fmt(f),
            Self::Unknown(ref msg) => write!(f, "Unknown error: {}", msg),
        }
    }
}

impl StdError for Error {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        match *self {
            Self::Http(ref err) => Some(err),
            Self::Ws(ref err) => Some(err),
            _ => None,
        }
    }
}

impl From<HttpError> for Error {
    fn from(err: HttpError) -> Self {
        Self::Http(err)
    }
}

impl From<WsError> for Error {
    fn from(err: WsError) -> Self {
        Self::Ws(err)
    }
}
