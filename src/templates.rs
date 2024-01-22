use std::sync::Arc;

use askama::Template;

use crate::{
    ar_configs::ArConfigs,
    ARFlow,
    AppConfig,
};

/// The index page template, displays a 3D model and a list of buttons that link to other websites.
/// If an [ARFlow] is specified, the page will automatically present an AR view of the model to the
/// user.
#[derive(Template)]
#[template(path = "index.html")]
pub struct Index {
    /// Which AR flow to use when the page is loaded, if any.
    pub ar_flow: ARFlow,
    /// The application configuration, as defined in `ar-router.toml`.
    pub config: Arc<AppConfig>,
    /// The AR configuration for each platform, constructs the URL that will be used to load the
    /// 3D model.
    pub ar_configs: Arc<ArConfigs>,
}

/// A small banner that renders at the bottom of an iOS AR View.
/// The documentation for this feature can be found [here.](https://developer.apple.com/documentation/arkit/arkit_in_ios/adding_an_apple_pay_button_or_a_custom_action_in_ar_quick_look#3402837)
#[derive(Template)]
#[template(path = "ios_banner.html")]
pub struct Banner {
    /// The application configuration, as defined in `ar-router.toml`.
    pub config: Arc<AppConfig>,
}
