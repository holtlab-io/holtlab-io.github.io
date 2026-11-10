//! ## Overview
//! Root crate entrypoint for Holt Lab's official landing page.
//! Configures global metadata, fonts, styles, and launches the Dioxus WebAssembly application.
//!
//! ## Search Tags
//! #entrypoint, #app, #dioxus, #wasm, #main
//!
//! ## Submodules
//! - [`components`] (`components/mod.rs`): Reusable UI components and section blocks.
//! - [`views`] (`views/mod.rs`): Top-level page views.

use dioxus::prelude::*;

pub mod components;
pub mod views;

use views::Home;

const MAIN_CSS: Asset = asset!("assets/tailwind.css");

fn main() {
    dioxus::launch(App);
}

/// Root application component setting document metadata and rendering the primary view.
#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        document::Link { rel: "stylesheet", href: "https://fonts.googleapis.com/css2?family=JetBrains+Mono:wght@400;500;600;700&display=swap" }
        document::Link { rel: "stylesheet", href: "https://cdn.jsdelivr.net/gh/orioncactus/pretendard/dist/web/static/pretendard.css" }
        document::Link { rel: "icon", href: "assets/branding/avatar.jpg" }
        document::Meta { name: "viewport", content: "width=device-width, initial-scale=1.0" }
        document::Meta { name: "description", content: "Holt Lab — Boutique Hardware, Mechanical & Embedded Engineering Studio. 0.00 mm³ Tolerance Turnkey Development." }
        document::Meta { property: "og:title", content: "HOLT LAB // BOUTIQUE HARDWARE STUDIO" }
        document::Meta { property: "og:description", content: "From Silicon to Enclosure to Screen. 0.00 mm³ 오차 없는 단일 리드 아키텍트 직통 턴키 개발." }
        document::Meta { property: "og:image", content: "assets/branding/banner.png" }

        Home {}
    }
}
