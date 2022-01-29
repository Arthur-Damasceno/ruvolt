use async_trait::async_trait;

use crate::{
    entities::{events::*, *},
    error::Error,
    Context,
};

/// Define handlers for supported events.
#[async_trait]
pub trait EventHandler: Send + Sync + 'static {
    /// An error has occurred.
    async fn error(&self, _error: Error) {}

    /// Bot is ready.
    async fn ready(&self, _cx: Context, _data: ReadyEvent) {}

    /// A new message received.
    async fn message(&self, _cx: Context, _data: Message) {}

    /// A message has been edited or otherwise updated.
    async fn message_update(&self, _cx: Context, _data: MessageUpdateEvent) {}

    /// A message has been deleted.
    async fn message_delete(&self, _cx: Context, _data: MessageDeleteEvent) {}

    /// A channel has been created.
    async fn channel_create(&self, _cx: Context, _data: Channel) {}

    /// A channel details were updated.
    async fn channel_update(&self, _cx: Context, _data: ChannelUpdateEvent) {}

    /// A channel has been deleted.
    async fn channel_delete(&self, _cx: Context, _data: ChannelDeleteEvent) {}

    /// A user has joined the group.
    async fn channel_group_join(&self, _cx: Context, _data: ChannelGroupJoinEvent) {}

    /// A user has left the group.
    async fn channel_group_leave(&self, _cx: Context, _data: ChannelGroupLeaveEvent) {}

    /// A user has started typing in a channel.
    async fn channel_start_typing(&self, _cx: Context, _data: ChannelStartTypingEvent) {}

    /// A user has stopped typing in a channel.
    async fn channel_stop_typing(&self, _cx: Context, _data: ChannelStopTypingEvent) {}

    /// You have acknowledged new messages in the channel up to the message id.
    async fn channel_ack(&self, _cx: Context, _data: ChannelAckEvent) {}

    /// A server details were updated.
    async fn server_update(&self, _cx: Context, _data: ServerUpdateEvent) {}

    /// A server has been deleted.
    async fn server_delete(&self, _cx: Context, _data: ServerDeleteEvent) {}

    /// A server member details were updated.
    async fn server_member_update(&self, _cx: Context, _data: ServerMemberUpdateEvent) {}

    /// A user has joined the group.
    async fn server_member_join(&self, _cx: Context, _data: ServerMemberJoinEvent) {}

    /// A user has left the group.
    async fn server_member_leave(&self, _cx: Context, _data: ServerMemberLeaveEvent) {}

    /// A server role details were updated.
    async fn server_role_update(&self, _cx: Context, _data: ServerRoleUpdateEvent) {}

    /// A server role has been deleted.
    async fn server_role_delete(&self, _cx: Context, _data: ServerRoleDeleteEvent) {}

    /// A user has been updated.
    async fn user_update(&self, _cx: Context, _data: UserUpdateEvent) {}
}

#[async_trait]
pub(crate) trait EventHandlerExt: EventHandler {
    async fn handle(&self, cx: Context, event: ServerToClientEvent) {
        match event {
            ServerToClientEvent::Pong { .. } => return,
            ServerToClientEvent::Ready(data) => self.ready(cx, data).await,
            ServerToClientEvent::Message(msg) => self.message(cx, msg).await,
            ServerToClientEvent::MessageUpdate { .. } => {
                self.message_update(cx, MessageUpdateEvent::from(event))
                    .await;
            }
            ServerToClientEvent::MessageDelete { .. } => {
                self.message_delete(cx, MessageDeleteEvent::from(event))
                    .await;
            }
            ServerToClientEvent::ChannelCreate(channel) => self.channel_create(cx, channel).await,
            ServerToClientEvent::ChannelUpdate { .. } => {
                self.channel_update(cx, ChannelUpdateEvent::from(event))
                    .await;
            }
            ServerToClientEvent::ChannelDelete { .. } => {
                self.channel_delete(cx, ChannelDeleteEvent::from(event))
                    .await;
            }
            ServerToClientEvent::ChannelGroupJoin { .. } => {
                self.channel_group_join(cx, ChannelGroupJoinEvent::from(event))
                    .await;
            }
            ServerToClientEvent::ChannelGroupLeave { .. } => {
                self.channel_group_leave(cx, ChannelGroupLeaveEvent::from(event))
                    .await;
            }
            ServerToClientEvent::ChannelStartTyping { .. } => {
                self.channel_start_typing(cx, ChannelStartTypingEvent::from(event))
                    .await;
            }
            ServerToClientEvent::ChannelStopTyping { .. } => {
                self.channel_stop_typing(cx, ChannelStopTypingEvent::from(event))
                    .await;
            }
            ServerToClientEvent::ChannelAck { .. } => {
                self.channel_ack(cx, ChannelAckEvent::from(event)).await
            }
            ServerToClientEvent::ServerUpdate { .. } => {
                self.server_update(cx, ServerUpdateEvent::from(event)).await;
            }
            ServerToClientEvent::ServerDelete { .. } => {
                self.server_delete(cx, ServerDeleteEvent::from(event)).await;
            }
            ServerToClientEvent::ServerMemberUpdate { .. } => {
                self.server_member_update(cx, ServerMemberUpdateEvent::from(event))
                    .await;
            }
            ServerToClientEvent::ServerMemberJoin { .. } => {
                self.server_member_join(cx, ServerMemberJoinEvent::from(event))
                    .await;
            }
            ServerToClientEvent::ServerMemberLeave { .. } => {
                self.server_member_leave(cx, ServerMemberLeaveEvent::from(event))
                    .await;
            }
            ServerToClientEvent::ServerRoleUpdate { .. } => {
                self.server_role_update(cx, ServerRoleUpdateEvent::from(event))
                    .await;
            }
            ServerToClientEvent::ServerRoleDelete { .. } => {
                self.server_role_delete(cx, ServerRoleDeleteEvent::from(event))
                    .await;
            }
            ServerToClientEvent::UserUpdate { .. } => {
                self.user_update(cx, UserUpdateEvent::from(event)).await;
            }
            event => {
                let error = Error::Unknown(format!("Unexpected event from server: {:?}", event));
                self.error(error).await;
            }
        }
    }
}

impl<T: EventHandler> EventHandlerExt for T {}
