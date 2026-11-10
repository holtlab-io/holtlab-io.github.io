//! ## Overview
//! Root crate entrypoint for Holt Lab's official multi-page web platform.
//! Configures global metadata, internationalization context, SSG route discovery, and launches the Dioxus router application.
//!
//! ## Search Tags
//! #entrypoint, #app, #dioxus, #wasm, #main, #router, #ssg
//!
//! ## Submodules
//! - [`components`] (`components/mod.rs`): Reusable UI components and section blocks.
//! - [`i18n`] (`i18n/mod.rs`): Bilingual Korean/English internationalization system.
//! - [`routes`] (`routes.rs`): Multi-page application route definitions and route enum.
//! - [`views`] (`views/mod.rs`): Top-level page views rendered by the router.

use dioxus::prelude::*;

pub mod components;
pub mod i18n;
pub mod routes;
pub mod views;

use i18n::Language;
use routes::Route;

/// Discovers static pre-render routes for SSG export.
#[server(endpoint = "static_routes")]
pub async fn static_routes() -> Result<Vec<String>, ServerFnError> {
    let routes = vec![
        "/".to_string(),
        "/demonstrator".to_string(),
        "/studio".to_string(),
        "/contact".to_string(),
    ];
    Ok(routes)
}

/// Synchronizes with Dioxus CLI client bundler to ensure clean index.html template.
#[cfg(feature = "server")]
async fn sync_client_template(static_dir: &std::path::Path) {
    let index_file = static_dir.join("index.html");
    for _ in 0..100 {
        if index_file.exists() {
            if let Ok(content) = std::fs::read_to_string(&index_file) {
                if content.contains("type=\"module\"") {
                    if content.contains("data-node-hydration") {
                        if let Some((before_main, rest)) = content.split_once("id=\"main\">") {
                            if let Some(script_idx) = rest.find("<script type=\"module\"") {
                                let script_part = &rest[script_idx..];
                                let clean = format!("{}id=\"main\"></div>\n{}", before_main, script_part);
                                let _ = std::fs::write(&index_file, clean);
                            }
                        }
                    }
                    tokio::time::sleep(std::time::Duration::from_millis(50)).await;
                    break;
                }
            }
        }
        tokio::time::sleep(std::time::Duration::from_millis(50)).await;
    }
}

/// Server entrypoint running Axum with Dioxus Fullstack SSR handler for SSG builds.
#[cfg(feature = "server")]
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let static_dir = std::env::var("DIOXUS_PUBLIC_PATH")
        .map(std::path::PathBuf::from)
        .ok()
        .or_else(|| {
            std::env::current_exe()
                .ok()
                .and_then(|p| p.parent().map(|parent| parent.join("public")))
        })
        .unwrap_or_else(|| std::path::PathBuf::from("./public"));

    sync_client_template(&static_dir).await;

    let incremental_cfg = dioxus::server::IncrementalRendererConfig::new()
        .static_dir(static_dir);

    let serve_cfg = dioxus::server::ServeConfig::builder()
        .incremental(incremental_cfg);

    let addr = dioxus_cli_config::fullstack_address_or_localhost();

    use dioxus::server::DioxusRouterExt;
    let router = axum::Router::new()
        .serve_dioxus_application(serve_cfg.clone(), App);

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, router).await?;
    Ok(())
}

/// Client entrypoint for browser WASM hydration.
#[cfg(not(feature = "server"))]
fn main() {
    dioxus::launch(App);
}

/// Root application component setting document metadata and rendering the router shell.
#[component]
fn App() -> Element {
    let mut lang = use_context_provider(|| Signal::new(Language::En));

    use_effect(move || {
        let detected = Language::detect();
        if *lang.peek() != detected {
            lang.set(detected);
        }
    });

    rsx! {
        components::HeadMetadata {}
        Router::<Route> {}
    }
}
