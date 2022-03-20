use async_trait::async_trait;

use crate::{
    models::{events::*, *},
    Context,
};

/// Define handlers for supported events.
#[async_trait]
pub trait EventHandler: Send + Sync + 'static {
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
    async fn handle(&self, cx: Context, event: ServerEvent) {
        match event {
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
            _ => return,
        }
    }
}

impl<T: EventHandler> EventHandlerExt for T {}
