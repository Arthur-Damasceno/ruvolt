//! Module for [Error] and [Result] types.

use {
    reqwest::Error as HttpError, serde::Deserialize, std::result::Result as StdResult,
    thiserror::Error as ThisError, tokio_tungstenite::tungstenite::Error as WsError,
};

/// [Result](StdResult) type for [Error].
pub type Result<T = (), E = Error> = StdResult<T, E>;

/// Errors that can happen when using [ruvolt](crate).
#[derive(ThisError, Debug)]
pub enum Error {
    /// Http requests error.
    #[error("Http error: {0}")]
    Http(#[from] HttpError),
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
#[derive(ThisError, Debug, Deserialize, Clone, Copy, PartialEq)]
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
