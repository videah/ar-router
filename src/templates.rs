use std::sync::Arc;

use askama::Template;

use crate::{
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
}
