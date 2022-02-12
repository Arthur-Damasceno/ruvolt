use serde::Deserialize;

use crate::{
    models::{Channel, Id},
    Context, Result,
};

/// A message has been deleted.
#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct MessageDeleteEvent {
    /// Message id.
    #[serde(rename = "id")]
    pub message_id: Id,
    /// Message channel id.
    #[serde(rename = "channel")]
    pub channel_id: Id,
}

impl MessageDeleteEvent {
    /// Get the channel from the API.
    pub async fn fetch_channel(&self, cx: &Context) -> Result<Channel> {
        Channel::fetch(cx, &self.channel_id).await
    }
}
