use serde::Deserialize;

use crate::{
    models::{Channel, Id, User},
    Context, Result,
};

/// A user has left the group.
#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct ChannelGroupLeaveEvent {
    /// Channel id.
    #[serde(rename = "id")]
    pub channel_id: Id,
    /// User id.
    #[serde(rename = "user")]
    pub user_id: Id,
}

impl ChannelGroupLeaveEvent {
    /// Fetch the channel.
    pub async fn channel(&self, cx: &Context) -> Result<Channel> {
        Channel::fetch(cx, &self.channel_id).await
    }

    /// Fetch the user.
    pub async fn user(&self, cx: &Context) -> Result<User> {
        User::fetch(cx, &self.user_id).await
    }
}
