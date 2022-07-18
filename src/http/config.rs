/// Revolt server configuration.
#[derive(Debug, Clone, Deserialize)]
pub struct RevoltConfig {
    /// Revolt API version.
    pub revolt: String,
    /// Features enabled on the Revolt node.
    pub features: RevoltFeatures,
    /// WebSocket url.
    pub ws: String,
    /// Url pointing to the client serving the node.
    pub app: String,
    /// Web Push VAPID public key.
    pub vapid: String,
}

/// Server features configuration.
#[derive(Debug, Clone, Deserialize)]
pub struct RevoltFeatures {
    /// hCaptcha configuration.
    pub captcha: CaptchaFeature,
    /// Whether email verification is enabled.
    pub email: bool,
    /// Whether the server is invite only.
    pub invite_only: bool,
    /// File server service configuration.
    pub autumn: Feature,
    /// Proxy service configuration.
    pub january: Feature,
    /// Voice server configuration.
    pub voso: VoiceFeature,
}

/// Voice server configuration.
#[derive(Debug, Clone, Deserialize)]
pub struct VoiceFeature {
    /// Whether voice is enabled.
    pub enabled: bool,
    /// Url pointing to the voice API.
    pub url: String,
    /// Url pointing to the voice WebSocket server.
    pub ws: String,
}

/// hCaptcha configuration.
#[derive(Debug, Clone, Deserialize)]
pub struct CaptchaFeature {
    /// Whether captcha is enabled.
    pub enabled: bool,
    /// Client key used for solving captcha.
    pub key: String,
}

/// Generic service configuration.
#[derive(Debug, Clone, Deserialize)]
pub struct Feature {
    /// Whether the service is enabled.
    pub enabled: bool,
    /// Url pointing to the service.
    pub url: String,
}
