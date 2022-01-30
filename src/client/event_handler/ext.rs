use async_trait::async_trait;

use {
    super::EventHandler,
    crate::{entities::events::*, error::Error, Context},
};

#[async_trait]
pub trait EventHandlerExt: EventHandler {
    async fn handle(&self, cx: Context, event: ServerToClientEvent) {
        match event {
            ServerToClientEvent::Pong { .. } => return,
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
