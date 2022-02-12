use async_trait::async_trait;

use {
    super::EventHandler,
    crate::{error::Error, models::events::*, Context},
};

#[async_trait]
pub trait EventHandlerExt: EventHandler {
    async fn handle(&self, cx: Context, event: ServerToClientEvent) {
        match event {
            ServerToClientEvent::Pong { .. } => return,
            ServerToClientEvent::Ready(data) => self.ready(cx, data).await,
            ServerToClientEvent::Message(msg) => self.message(cx, msg).await,
            ServerToClientEvent::MessageUpdate(data) => self.message_update(cx, data).await,
            ServerToClientEvent::MessageDelete(data) => self.message_delete(cx, data).await,
            ServerToClientEvent::ChannelCreate(channel) => self.channel_create(cx, channel).await,
            ServerToClientEvent::ChannelUpdate(data) => self.channel_update(cx, data).await,
            ServerToClientEvent::ChannelDelete(data) => self.channel_delete(cx, data).await,
            ServerToClientEvent::ChannelGroupJoin(data) => self.channel_group_join(cx, data).await,
            ServerToClientEvent::ChannelGroupLeave(data) => {
                self.channel_group_leave(cx, data).await
            }
            ServerToClientEvent::ChannelStartTyping(data) => {
                self.channel_start_typing(cx, data).await
            }
            ServerToClientEvent::ChannelStopTyping(data) => {
                self.channel_stop_typing(cx, data).await
            }
            ServerToClientEvent::ChannelAck(data) => self.channel_ack(cx, data).await,
            ServerToClientEvent::ServerUpdate(data) => self.server_update(cx, data).await,
            ServerToClientEvent::ServerDelete(data) => self.server_delete(cx, data).await,
            ServerToClientEvent::ServerMemberUpdate(data) => {
                self.server_member_update(cx, data).await
            }
            ServerToClientEvent::ServerMemberJoin(data) => self.server_member_join(cx, data).await,
            ServerToClientEvent::ServerMemberLeave(data) => {
                self.server_member_leave(cx, data).await
            }
            ServerToClientEvent::ServerRoleUpdate(data) => self.server_role_update(cx, data).await,
            ServerToClientEvent::ServerRoleDelete(data) => self.server_role_delete(cx, data).await,
            ServerToClientEvent::UserUpdate(data) => self.user_update(cx, data).await,
            event => {
                let error = Error::Unknown(format!("Unexpected event from server: {:?}", event));
                self.error(error).await;
            }
        }
    }
}

impl<T: EventHandler> EventHandlerExt for T {}
