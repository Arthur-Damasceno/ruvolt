//! Module for [Error] and [Result] types.

/// [Result](std::result::Result) type alias for [Error].
pub type Result<T = (), E = Error> = std::result::Result<T, E>;

/// Errors that can happen when using [ruvolt](crate).
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// Http requests error.
    #[error("Http error: {0}")]
    Http(#[from] HttpError),
    /// [Tungstenite](tokio_tungstenite::tungstenite) error.
    #[error("Tungstenite error: {0}")]
    Tungstenite(#[from] tokio_tungstenite::tungstenite::Error),
    /// Could not authenticate due to an error.
    #[error("Authentication error: {0}")]
    Authentication(#[from] AuthenticationError),
    /// Unknown or unexpected error.
    #[error("Unknown error: {0}")]
    Unknown(String),
}

impl From<reqwest::Error> for Error {
    fn from(err: reqwest::Error) -> Self {
        Self::Http(err.into())
    }
}

/// Http requests error.
#[derive(Debug, thiserror::Error)]
pub enum HttpError {
    /// [Reqwest](reqwest) error.
    #[error("Reqwest error: {0}")]
    Reqwest(#[from] reqwest::Error),
    /// A resource was not found.
    #[error("A resource was not found")]
    NotFound,
    /// Client has reached rate limits.
    #[error("Rate limits reached, retry after {0}ms")]
    RetryAfter(
        /// Milliseconds until calls are replenished.
        u64,
    ),
    /// The Revolt server ran into an issue.
    #[error("The Revolt server ran into an issue")]
    Internal,
    /// An unexpected response occurred.
    #[error("An unexpected response occurred: {status} - {url} with body: {body:?}")]
    Other {
        /// Response status code.
        status: crate::http::StatusCode,
        /// Requested url.
        url: String,
        /// Response body.
        body: Option<String>,
    },
}

/// Authentication error.
#[derive(Debug, Clone, Copy, PartialEq, Deserialize, thiserror::Error)]
pub enum AuthenticationError {
    /// Uncategorized error.
    #[error("Uncategorized error")]
    LabelMe,
    /// The Revolt server ran into an issue.
    #[error("The Revolt server ran into an issuernalError")]
    InternalError,
    /// The token provided is incorrect.
    #[error("The token provided is incorrect")]
    InvalidSession,
    /// The bot is already authenticated.
    #[error("The bot is already authenticated")]
    AlreadyAuthenticated,
}
