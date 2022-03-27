use async_trait::async_trait;

use crate::{
    models::{events::*, *},
    Context,
};

/// Define handlers for supported events.
#[async_trait]
pub trait EventHandler: Send + Sync + 'static {
    /// Bot is ready.
    async fn ready(_cx: Context, _data: ReadyEvent) {}

    /// A new message received.
    async fn message(_cx: Context, _data: Message) {}

    /// A message has been edited or otherwise updated.
    async fn message_update(_cx: Context, _data: MessageUpdateEvent) {}

    /// A message has been deleted.
    async fn message_delete(_cx: Context, _data: MessageDeleteEvent) {}

    /// A channel has been created.
    async fn channel_create(_cx: Context, _data: Channel) {}

    /// A channel details were updated.
    async fn channel_update(_cx: Context, _data: ChannelUpdateEvent) {}

    /// A channel has been deleted.
    async fn channel_delete(_cx: Context, _data: ChannelDeleteEvent) {}

    /// A user has joined the group.
    async fn channel_group_join(_cx: Context, _data: ChannelGroupJoinEvent) {}

    /// A user has left the group.
    async fn channel_group_leave(_cx: Context, _data: ChannelGroupLeaveEvent) {}

    /// A user has started typing in a channel.
    async fn channel_start_typing(_cx: Context, _data: ChannelStartTypingEvent) {}

    /// A user has stopped typing in a channel.
    async fn channel_stop_typing(_cx: Context, _data: ChannelStopTypingEvent) {}

    /// You have acknowledged new messages in the channel up to the message id.
    async fn channel_ack(_cx: Context, _data: ChannelAckEvent) {}

    /// A server details were updated.
    async fn server_update(_cx: Context, _data: ServerUpdateEvent) {}

    /// A server has been deleted.
    async fn server_delete(_cx: Context, _data: ServerDeleteEvent) {}

    /// A server member details were updated.
    async fn server_member_update(_cx: Context, _data: ServerMemberUpdateEvent) {}

    /// A user has joined the group.
    async fn server_member_join(_cx: Context, _data: ServerMemberJoinEvent) {}

    /// A user has left the group.
    async fn server_member_leave(_cx: Context, _data: ServerMemberLeaveEvent) {}

    /// A server role details were updated.
    async fn server_role_update(_cx: Context, _data: ServerRoleUpdateEvent) {}

    /// A server role has been deleted.
    async fn server_role_delete(_cx: Context, _data: ServerRoleDeleteEvent) {}

    /// A user has been updated.
    async fn user_update(_cx: Context, _data: UserUpdateEvent) {}
}

#[async_trait]
pub(crate) trait EventHandlerExt: EventHandler {
    async fn handle(cx: Context, event: ServerEvent) {
        match event {
            ServerEvent::Ready(data) => Self::ready(cx, data).await,
            ServerEvent::Message(data) => Self::message(cx, data).await,
            ServerEvent::MessageUpdate(data) => Self::message_update(cx, data).await,
            ServerEvent::MessageDelete(data) => Self::message_delete(cx, data).await,
            ServerEvent::ChannelCreate(data) => Self::channel_create(cx, data).await,
            ServerEvent::ChannelUpdate(data) => Self::channel_update(cx, data).await,
            ServerEvent::ChannelDelete(data) => Self::channel_delete(cx, data).await,
            ServerEvent::ChannelGroupJoin(data) => Self::channel_group_join(cx, data).await,
            ServerEvent::ChannelGroupLeave(data) => Self::channel_group_leave(cx, data).await,
            ServerEvent::ChannelStartTyping(data) => Self::channel_start_typing(cx, data).await,
            ServerEvent::ChannelStopTyping(data) => Self::channel_stop_typing(cx, data).await,
            ServerEvent::ChannelAck(data) => Self::channel_ack(cx, data).await,
            ServerEvent::ServerUpdate(data) => Self::server_update(cx, data).await,
            ServerEvent::ServerDelete(data) => Self::server_delete(cx, data).await,
            ServerEvent::ServerMemberUpdate(data) => Self::server_member_update(cx, data).await,
            ServerEvent::ServerMemberJoin(data) => Self::server_member_join(cx, data).await,
            ServerEvent::ServerMemberLeave(data) => Self::server_member_leave(cx, data).await,
            ServerEvent::ServerRoleUpdate(data) => Self::server_role_update(cx, data).await,
            ServerEvent::ServerRoleDelete(data) => Self::server_role_delete(cx, data).await,
            ServerEvent::UserUpdate(data) => Self::user_update(cx, data).await,
            _ => return,
        }
    }
}

impl<T: EventHandler> EventHandlerExt for T {}
