use crate::models::{Attachment, Category, SystemMessageChannels};

#[cfg(feature = "cache")]
use crate::{cache::UpdateCache, Context};

/// Specifies a field to remove on server update.
#[derive(Debug, Clone, Copy, PartialEq, Deserialize, Serialize)]
#[non_exhaustive]
pub enum ServerField {
    /// Server icon.
    Icon,
    /// Server banner.
    Banner,
    /// Server description.
    Description,
}

/// A server details were updated.
#[derive(Debug, Clone, PartialEq, Deserialize)]
#[non_exhaustive]
pub struct ServerUpdateEvent {
    /// Server id.
    #[serde(rename = "id")]
    pub server_id: String,
    /// A partial server object.
    pub data: PartialServer,
    /// A specified field to remove on server update.
    pub clear: Option<ServerField>,
}

/// A partial server.
#[derive(Debug, Clone, PartialEq, Deserialize)]
#[non_exhaustive]
pub struct PartialServer {
    /// Server name.
    pub name: Option<String>,
    /// Server description.
    pub description: Option<String>,
    /// Server icon.
    pub icon: Option<Attachment>,
    /// Server banner.
    pub banner: Option<Attachment>,
    /// Server categories.
    #[serde(default)]
    pub categories: Vec<Category>,
    /// Server system message channels.
    pub system_messages: Option<SystemMessageChannels>,
    /// Whether server is not safe for work.
    pub nsfw: Option<bool>,
}

#[cfg(feature = "cache")]
#[async_trait]
impl UpdateCache for ServerUpdateEvent {
    async fn update(&self, cx: &Context) {
        if let Some(server) = cx.cache.servers.write().await.get_mut(&self.server_id) {
            if let Some(field) = self.clear {
                match field {
                    ServerField::Icon => server.icon = None,
                    ServerField::Banner => server.banner = None,
                    ServerField::Description => server.description = None,
                }
            }

            if let Some(ref name) = self.data.name {
                server.name = name.clone();
            }

            if let Some(ref description) = self.data.description {
                server.description = Some(description.clone());
            }

            if let Some(ref icon) = self.data.icon {
                server.icon = Some(icon.clone());
            }

            if let Some(ref banner) = self.data.banner {
                server.banner = Some(banner.clone());
            }

            if !self.data.categories.is_empty() {
                server.categories = self.data.categories.clone();
            }

            if let Some(ref system_messages) = self.data.system_messages {
                server.system_messages = system_messages.clone();
            }

            if let Some(nsfw) = self.data.nsfw {
                server.nsfw = nsfw;
            }
        }
    }
}
