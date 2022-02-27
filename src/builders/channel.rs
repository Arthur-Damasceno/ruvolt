use serde::Serialize;

/// Builder for create a server channel.
#[derive(Debug, Clone, Serialize)]
pub struct CreateChannel {
    r#type: ChannelType,
    name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,
    nsfw: bool,
}

#[derive(Debug, Clone, Serialize)]
enum ChannelType {
    Text,
    Voice,
}

impl CreateChannel {
    /// Creates a new builder with `Text` channel type.
    pub fn text(name: impl Into<String>) -> Self {
        Self {
            r#type: ChannelType::Text,
            name: name.into(),
            description: None,
            nsfw: false,
        }
    }

    /// Creates a new builder with `Voice` channel type.
    pub fn voice(name: impl Into<String>) -> Self {
        Self {
            r#type: ChannelType::Voice,
            name: name.into(),
            description: None,
            nsfw: false,
        }
    }

    /// Set the description.
    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    /// Set whether channel is not safe for work.
    pub fn nsfw(mut self, nsfw: bool) -> Self {
        self.nsfw = nsfw;
        self
    }
}
