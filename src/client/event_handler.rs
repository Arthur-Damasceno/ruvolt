use async_trait::async_trait;

use crate::{entities::*, error::Error};

/// Define handlers for supported events.
#[async_trait]
pub trait EventHandler: Send + Sync + 'static {
    /// An error has occurred.
    async fn error(&self, _error: Error) {}

    /// A new message received.
    async fn message(&self, _data: Message) {}

    /// A message has been edited or otherwise updated.
    async fn message_update(&self, _data: MessageUpdateEvent) {}

    /// A message has been deleted.
    async fn message_delete(&self, _data: MessageDeleteEvent) {}

    /// A channel details were updated.
    async fn channel_update(&self, _data: ChannelUpdateEvent) {}

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

    /// A server details were updated.
    async fn server_update(&self, _data: ServerUpdateEvent) {}

    /// A server has been deleted.
    async fn server_delete(&self, _data: ServerDeleteEvent) {}

    /// A server member details were updated.
    async fn server_member_update(&self, _data: ServerMemberUpdateEvent) {}

    /// A user has joined the group.
    async fn server_member_join(&self, _data: ServerMemberJoinEvent) {}

    /// A user has left the group.
    async fn server_member_leave(&self, _data: ServerMemberLeaveEvent) {}

    /// A server role details were updated.
    async fn server_role_update(&self, _data: ServerRoleUpdateEvent) {}

    /// A server role has been deleted.
    async fn server_role_delete(&self, _data: ServerRoleDeleteEvent) {}

    /// A user has been updated.
    async fn user_update(&self, _data: UserUpdateEvent) {}
}

#[async_trait]
pub(crate) trait EventHandlerExt: EventHandler {
    async fn handle(&self, event: ServerToClientEvent) {
        match event {
            ServerToClientEvent::Pong { .. } => return,
            ServerToClientEvent::Message(msg) => self.message(msg).await,
            ServerToClientEvent::MessageUpdate { .. } => {
                self.message_update(MessageUpdateEvent::from(event)).await;
            }
            ServerToClientEvent::MessageDelete { .. } => {
                self.message_delete(MessageDeleteEvent::from(event)).await;
            }
            ServerToClientEvent::ChannelUpdate { .. } => {
                self.channel_update(ChannelUpdateEvent::from(event)).await;
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
            ServerToClientEvent::ServerUpdate { .. } => {
                self.server_update(ServerUpdateEvent::from(event)).await;
            }
            ServerToClientEvent::ServerDelete { .. } => {
                self.server_delete(ServerDeleteEvent::from(event)).await;
            }
            ServerToClientEvent::ServerMemberUpdate { .. } => {
                self.server_member_update(ServerMemberUpdateEvent::from(event))
                    .await;
            }
            ServerToClientEvent::ServerMemberJoin { .. } => {
                self.server_member_join(ServerMemberJoinEvent::from(event))
                    .await;
            }
            ServerToClientEvent::ServerMemberLeave { .. } => {
                self.server_member_leave(ServerMemberLeaveEvent::from(event))
                    .await;
            }
            ServerToClientEvent::ServerRoleUpdate { .. } => {
                self.server_role_update(ServerRoleUpdateEvent::from(event))
                    .await;
            }
            ServerToClientEvent::ServerRoleDelete { .. } => {
                self.server_role_delete(ServerRoleDeleteEvent::from(event))
                    .await;
            }
            ServerToClientEvent::UserUpdate { .. } => {
                self.user_update(UserUpdateEvent::from(event)).await;
            }
            event => {
                let error = Error::Unknown(format!("Unexpected event from server: {:?}", event));
                self.error(error).await;
            }
        }
    }
}

impl<T: EventHandler> EventHandlerExt for T {}
