//! Module for [enum@Error] and [Result] types.

use {
    reqwest::{Error as HttpError, Response},
    serde::Deserialize,
    std::result::Result as StdResult,
    thiserror::Error,
    tokio_tungstenite::tungstenite::Error as WsError,
};

/// [Result](StdResult) type for [enum@Error].
pub type Result<T = (), E = Error> = StdResult<T, E>;

/// Errors that can happen when using [ruvolt](crate).
#[derive(Error, Debug)]
pub enum Error {
    /// Http requests error.
    #[error("Http error: {0}")]
    Http(#[from] HttpError),
    /// Received a response with a status code other than 200-299.
    #[error("Unsuccessful request: {0:?}")]
    UnsuccessfulRequest(Response),
    /// WebSocket error.
    #[error("WebSocket error: {0}")]
    Ws(#[from] WsError),
    /// Could not authenticate due to an error.
    #[error("Authentication error: {0}")]
    Authentication(#[from] AuthenticationError),
    /// Unknown or unexpected error.
    #[error("Unknown error: {0}")]
    Unknown(String),
}

/// Authentication error.
#[derive(Error, Debug, Deserialize, Clone, Copy, PartialEq)]
pub enum AuthenticationError {
    /// Uncategorized error.
    #[error("LabelMe")]
    LabelMe,
    /// The Revolt server ran into an issue.
    #[error("InternalError")]
    InternalError,
    /// The token provided is incorrect.
    #[error("InvalidSession")]
    InvalidSession,
    /// The bot is already authenticated.
    #[error("AlreadyAuthenticated")]
    AlreadyAuthenticated,
}
