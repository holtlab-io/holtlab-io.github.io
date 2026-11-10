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
    let lang = use_context_provider(|| Signal::new(Language::detect()));

    use_effect(move || {
        let _ = lang();
        #[cfg(target_arch = "wasm32")]
        {
            let current_lang = lang();
            if let Some(window) = web_sys::window() {
                if let Some(doc) = window.document() {
                    let title = match current_lang {
                        Language::En => "Holt Lab — Hardware & Embedded Engineering Studio",
                        Language::Ko => "Holt Lab — 하드웨어 & 임베디드 엔지니어링 랩",
                    };
                    doc.set_title(title);
                    if let Some(html_el) = doc.document_element() {
                        let code = match current_lang {
                            Language::En => "en",
                            Language::Ko => "ko",
                        };
                        let _ = html_el.set_attribute("lang", code);
                    }
                    if let Ok(Some(meta_desc)) = doc.query_selector("meta[name='description']") {
                        let desc = match current_lang {
                            Language::En => "Holt Lab — Hardware, Mechanical & Embedded Engineering Studio. Unified Turnkey Hardware & Embedded Execution.",
                            Language::Ko => "Holt Lab — 하드웨어, 기구 및 임베디드 엔지니어링 랩. 단일 리드 턴키 하드웨어 개발.",
                        };
                        let _ = meta_desc.set_attribute("content", desc);
                    }
                }
            }
        }
    });

    rsx! {
        Home {}
    }
}
