use std::fmt::Display;

/// The AR configuration for each platform.
pub struct ArConfigs {
    /// The AR configuration for Apple devices.
    pub apple: AppleArConfig,
    /// The AR configuration for Android devices.
    pub android: AndroidArConfig,
}

/// The AR configuration for Apple devices.
pub struct AppleArConfig {
    /// The path to a `.usdz` 3D model file.
    pub model_path: String,
    /// The URL to some banner HTML that will be rendered at the bottom of the AR view.
    pub banner_url: String,
    /// The URL to be shared when using the share button in the AR view.
    pub share_url: String,
}

impl Display for AppleArConfig {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = format!(
            "{}#custom={}&canonicalWebPageURL={}",
            self.model_path, self.banner_url, self.share_url,
        );
        write!(f, "{}", str)
    }
}

/// The AR configuration for Android devices.
pub struct AndroidArConfig {
    /// The path to a `.glb` or `.gltf` 3D model file.
    pub model_path: String,
    /// The title of the model, displayed in the AR view.
    pub title: String,
    /// The URL to be redirected to if the user's device does not support AR in one way or another.
    pub fallback_url: String,
}

impl Display for AndroidArConfig {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = format!(
            "intent://arvr.google.com/scene-viewer/1.0?file={}&title={}&mode=ar_only#Intent;scheme=https;package=com.google.ar.core;action=android.intent.action.VIEW;S.browser_fallback_url={};end;",
            self.model_path, self.title, self.fallback_url,
        );
        write!(f, "{}", str)
    }
}
