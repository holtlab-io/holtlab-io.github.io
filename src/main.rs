//! ## Overview
//! Root crate entrypoint for Holt Lab's official landing page.
//! Configures global metadata, internationalization context, and launches the Dioxus application.
//!
//! ## Search Tags
//! #entrypoint, #app, #dioxus, #wasm, #main
//!
//! ## Submodules
//! - [`components`] (`components/mod.rs`): Reusable UI components and section blocks.
//! - [`i18n`] (`i18n/mod.rs`): Bilingual Korean/English internationalization system.
//! - [`views`] (`views/mod.rs`): Top-level page views.

use dioxus::prelude::*;

pub mod components;
pub mod i18n;
pub mod views;

use i18n::Language;
use views::Home;

fn main() {
    dioxus::launch(App);
}

/// Root application component setting document metadata and rendering the primary view.
#[component]
fn App() -> Element {
    use_context_provider(|| Signal::new(Language::detect()));

    rsx! {
        Home {}
    }
}
