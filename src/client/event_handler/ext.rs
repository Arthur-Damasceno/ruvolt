use async_trait::async_trait;

use {
    super::EventHandler,
    crate::{error::Error, models::events::ServerEvent, Context},
};

#[async_trait]
pub trait EventHandlerExt: EventHandler {
    async fn handle(&self, cx: Context, event: ServerEvent) {
        match event {
            ServerEvent::Pong { .. } => return,
            ServerEvent::Ready(data) => self.ready(cx, data).await,
            ServerEvent::Message(msg) => self.message(cx, msg).await,
            ServerEvent::MessageUpdate(data) => self.message_update(cx, data).await,
            ServerEvent::MessageDelete(data) => self.message_delete(cx, data).await,
            ServerEvent::ChannelCreate(channel) => self.channel_create(cx, channel).await,
            ServerEvent::ChannelUpdate(data) => self.channel_update(cx, data).await,
            ServerEvent::ChannelDelete(data) => self.channel_delete(cx, data).await,
            ServerEvent::ChannelGroupJoin(data) => self.channel_group_join(cx, data).await,
            ServerEvent::ChannelGroupLeave(data) => self.channel_group_leave(cx, data).await,
            ServerEvent::ChannelStartTyping(data) => self.channel_start_typing(cx, data).await,
            ServerEvent::ChannelStopTyping(data) => self.channel_stop_typing(cx, data).await,
            ServerEvent::ChannelAck(data) => self.channel_ack(cx, data).await,
            ServerEvent::ServerUpdate(data) => self.server_update(cx, data).await,
            ServerEvent::ServerDelete(data) => self.server_delete(cx, data).await,
            ServerEvent::ServerMemberUpdate(data) => self.server_member_update(cx, data).await,
            ServerEvent::ServerMemberJoin(data) => self.server_member_join(cx, data).await,
            ServerEvent::ServerMemberLeave(data) => self.server_member_leave(cx, data).await,
            ServerEvent::ServerRoleUpdate(data) => self.server_role_update(cx, data).await,
            ServerEvent::ServerRoleDelete(data) => self.server_role_delete(cx, data).await,
            ServerEvent::UserUpdate(data) => self.user_update(cx, data).await,
            event => {
                self.error(Error::Unknown(format!(
                    "Unexpected event from server: {:?}",
                    event
                )))
                .await
            }
        }
    }
}

impl<T: EventHandler> EventHandlerExt for T {}
