use serde::Deserialize;

use crate::{
    error::AuthenticationError,
    models::{events::*, Channel, Message},
};

#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(tag = "type")]
pub enum ServerEvent {
    Authenticated,
    Error { error: AuthenticationError },
    Pong { data: usize },
    Ready(ReadyEvent),
    Message(Message),
    MessageUpdate(MessageUpdateEvent),
    MessageDelete(MessageDeleteEvent),
    ChannelCreate(Channel),
    ChannelUpdate(ChannelUpdateEvent),
    ChannelDelete(ChannelDeleteEvent),
    ChannelGroupJoin(ChannelGroupJoinEvent),
    ChannelGroupLeave(ChannelGroupLeaveEvent),
    ChannelStartTyping(ChannelStartTypingEvent),
    ChannelStopTyping(ChannelStopTypingEvent),
    ChannelAck(ChannelAckEvent),
    ServerUpdate(ServerUpdateEvent),
    ServerDelete(ServerDeleteEvent),
    ServerMemberUpdate(ServerMemberUpdateEvent),
    ServerMemberJoin(ServerMemberJoinEvent),
    ServerMemberLeave(ServerMemberLeaveEvent),
    ServerRoleUpdate(ServerRoleUpdateEvent),
    ServerRoleDelete(ServerRoleDeleteEvent),
    UserUpdate(UserUpdateEvent),
}
