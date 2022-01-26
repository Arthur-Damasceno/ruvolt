use async_trait::async_trait;

use crate::{entities::*, error::Error};

/// Define handlers for supported events.
#[async_trait]
pub trait EventHandler: Send + Sync + 'static {
    /// An error has occurred.
    async fn error(&self, _error: Error) {}

    /// A message has been deleted.
    async fn message_delete(&self, _data: MessageDeleteEvent) {}

    /// A channel has been deleted.
    async fn channel_delete(&self, _data: ChannelDeleteEvent) {}

    /// A user has joined the group.
    async fn channel_group_join(&self, _data: ChannelGroupJoinEvent) {}

    /// A user has left the group.
    async fn channel_group_leave(&self, _data: ChannelGroupLeaveEvent) {}

    /// A user has started typing in a channel.
    async fn channel_start_typing(&self, _data: ChannelStartTypingEvent) {}

    /// A user has stopped typing in a channel.
    async fn channel_stop_typing(&self, _data: ChannelStopTypingEvent) {}

    /// You have acknowledged new messages in the channel up to the message id.
    async fn channel_ack(&self, _data: ChannelAckEvent) {}
}

#[async_trait]
pub(crate) trait EventHandlerExt: EventHandler {
    async fn handle(&self, event: ServerToClientEvent) {
        match event {
            ServerToClientEvent::Pong { .. } => return,
            ServerToClientEvent::MessageDelete { .. } => {
                self.message_delete(MessageDeleteEvent::from(event)).await;
            }
            ServerToClientEvent::ChannelDelete { .. } => {
                self.channel_delete(ChannelDeleteEvent::from(event)).await;
            }
            ServerToClientEvent::ChannelGroupJoin { .. } => {
                self.channel_group_join(ChannelGroupJoinEvent::from(event))
                    .await;
            }
            ServerToClientEvent::ChannelGroupLeave { .. } => {
                self.channel_group_leave(ChannelGroupLeaveEvent::from(event))
                    .await;
            }
            ServerToClientEvent::ChannelStartTyping { .. } => {
                self.channel_start_typing(ChannelStartTypingEvent::from(event))
                    .await;
            }
            ServerToClientEvent::ChannelStopTyping { .. } => {
                self.channel_stop_typing(ChannelStopTypingEvent::from(event))
                    .await;
            }
            ServerToClientEvent::ChannelAck { .. } => {
                self.channel_ack(ChannelAckEvent::from(event)).await
            }
            event => {
                let error = Error::Unknown(format!("Unexpected event from server: {:?}", event));
                self.error(error).await;
            }
        }
    }
}

impl<T: EventHandler> EventHandlerExt for T {}
