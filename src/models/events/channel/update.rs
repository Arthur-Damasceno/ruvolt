use crate::models::Attachment;

#[cfg(feature = "cache")]
use crate::{cache::UpdateCache, models::Channel, Context};

/// Specifies a field to remove on channel update.
#[derive(Debug, Clone, Copy, PartialEq, Deserialize, Serialize)]
#[non_exhaustive]
pub enum ChannelField {
    /// Channel icon.
    Icon,
    /// Channel description,
    Description,
}

/// A channel details were updated.
#[derive(Debug, Clone, PartialEq, Deserialize)]
#[non_exhaustive]
pub struct ChannelUpdateEvent {
    /// Channel id.
    #[serde(rename = "id")]
    pub channel_id: String,
    /// A partial channel.
    pub data: PartialChannel,
    /// A specified field to remove on channel update.
    pub clear: Option<ChannelField>,
}

/// A partial channel.
#[derive(Debug, Clone, PartialEq, Deserialize)]
#[non_exhaustive]
pub struct PartialChannel {
    /// Channel name.
    pub name: Option<String>,
    /// Channel description.
    pub description: Option<String>,
    /// Channel icon.
    pub icon: Option<Attachment>,
    /// Whether channel is not safe for work.
    pub nsfw: Option<bool>,
}

#[cfg(feature = "cache")]
#[async_trait]
impl UpdateCache for ChannelUpdateEvent {
    async fn update(&self, cx: &Context) {
        if let Some(channel) = cx.cache.channels.write().await.get_mut(&self.channel_id) {
            match channel {
                Channel::Group(channel) => {
                    if let Some(field) = self.clear {
                        match field {
                            ChannelField::Description => channel.description = None,
                            ChannelField::Icon => channel.icon = None,
                        }
                    }

                    if let Some(ref name) = self.data.name {
                        channel.name = name.clone();
                    }

                    if let Some(ref description) = self.data.description {
                        channel.description = Some(description.clone());
                    }

                    if let Some(ref icon) = self.data.icon {
                        channel.icon = Some(icon.clone());
                    }

                    if let Some(nsfw) = self.data.nsfw {
                        channel.nsfw = nsfw;
                    }
                }
                Channel::Text(channel) => {
                    if let Some(field) = self.clear {
                        match field {
                            ChannelField::Description => channel.description = None,
                            ChannelField::Icon => channel.icon = None,
                        }
                    }

                    if let Some(ref name) = self.data.name {
                        channel.name = name.clone();
                    }

                    if let Some(ref description) = self.data.description {
                        channel.description = Some(description.clone());
                    }

                    if let Some(ref icon) = self.data.icon {
                        channel.icon = Some(icon.clone());
                    }

                    if let Some(nsfw) = self.data.nsfw {
                        channel.nsfw = nsfw;
                    }
                }
                Channel::Voice(channel) => {
                    if let Some(field) = self.clear {
                        match field {
                            ChannelField::Description => channel.description = None,
                            ChannelField::Icon => channel.icon = None,
                        }
                    }

                    if let Some(ref name) = self.data.name {
                        channel.name = name.clone();
                    }

                    if let Some(ref description) = self.data.description {
                        channel.description = Some(description.clone());
                    }

                    if let Some(ref icon) = self.data.icon {
                        channel.icon = Some(icon.clone());
                    }

                    if let Some(nsfw) = self.data.nsfw {
                        channel.nsfw = nsfw;
                    }
                }
                _ => return,
            }
        }
    }
}
