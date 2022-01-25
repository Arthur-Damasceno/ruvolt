use async_trait::async_trait;

use crate::error::Error;

/// Define handlers for supported events.
#[async_trait]
pub trait EventHandler: Send + Sync + 'static {
    /// An error has occurred.
    async fn error(&self, error: Error);
}
