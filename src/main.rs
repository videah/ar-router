mod templates;
mod user_agent;

use std::{
    collections::HashMap,
    sync::Arc,
};

use axum::{
    extract::State,
    routing::get,
    Router,
};
use axum_embed::ServeEmbed;
use env_logger::Env;
use log::*;
use rust_embed::RustEmbed;
use serde::{
    Deserialize,
    Serialize,
};
use tokio::net::TcpListener;
use user_agent_parser::UserAgentParser;

use crate::user_agent::UserAgent;

/// Which AR flow to use when the page is loaded, if any.
#[derive(PartialEq, Debug)]
pub enum ARFlow {
    /// The user is using an Android device and we want to present a 3D model to them.
    /// We use the official ARCore web flow, where Scene Viewer is opened automatically using an
    /// intent.
    Android,
    /// The user is using an iOS device and we want to present a 3D model to them.
    /// We use the official ARKit web flow which is largely undocumented. We create a fake anchor
    /// `<img>` tag and then click it in JavaScript.
    #[allow(non_camel_case_types)]
    iOS,
    /// The user is using an unknown device or we don't want to present a 3D model to them.
    None,
}

/// Describes the assets folder to be served.
#[derive(RustEmbed, Clone)]
#[folder = "assets/"]
struct Assets;

/// The application state.
#[derive(Clone)]
struct AppState {
    /// A parser used to turn a user agent string into a more useful format.
    user_agent_parser: Arc<UserAgentParser>,
    /// The application configuration, as defined in `ar-router.toml`.
    config: Arc<AppConfig>,
}

/// The application configuration, as defined in `ar-router.toml`.
#[derive(Clone, Default, Debug, Serialize, Deserialize)]
struct AppConfig {
    /// The base URL for this application, including the protocol and domain.
    base_url: String,
    /// List of social links to display like a carrd.co page.
    social_links: HashMap<String, SocialLink>,
}

/// A social link to be displayed as a button.
#[derive(Clone, Default, Debug, Serialize, Deserialize)]
struct SocialLink {
    /// The name of the link, e.g. "My Website".
    name: String,
    /// The URL to open when the button is clicked.
    url: String,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let env = Env::default().filter_or("RUST_LOG", "info");
    env_logger::init_from_env(env);

    let listener = TcpListener::bind("0.0.0.0:8080").await?;
    let serve_assets = ServeEmbed::<Assets>::new();

    let regex = include_str!("../regexes.yaml");
    let config_str = include_str!("../ar-router.toml");
    let config: AppConfig = toml::from_str(config_str)?;

    let app = Router::new()
        .route("/", get(index))
        .route("/ar", get(route_to_model))
        .nest_service("/assets", serve_assets)
        .with_state(AppState {
            user_agent_parser: Arc::new(UserAgentParser::from_str(regex)?),
            config: Arc::new(config),
        });

    info!("Listening on {}", listener.local_addr()?);

    axum::serve(listener, app).await?;
    Ok(())
}

/// Renders the index page with no AR flow, just a simple carrd.co-like page.
async fn index(State(state): State<AppState>) -> templates::Index {
    templates::Index {
        ar_flow: ARFlow::None,
        config: state.config.clone(),
    }
}

/// The user scanned a QR code and we want to present them a 3D model and route them to the correct
/// AR flow based on their device user agent.
async fn route_to_model(
    State(state): State<AppState>,
    UserAgent(user_agent_header): UserAgent,
) -> templates::Index {
    let user_agent = user_agent_header.to_str().unwrap_or_default();
    let os = state.user_agent_parser.parse_os(&user_agent);
    let os_name = os.name.unwrap_or_default();

    let ar_flow = match os_name.as_ref() {
        "Android" => ARFlow::Android,
        "iOS" => ARFlow::iOS,
        _ => ARFlow::None,
    };
    debug!("ARFlow: {ar_flow:?} | User agent: {user_agent}");

    templates::Index {
        ar_flow,
        config: state.config.clone(),
    }
}
