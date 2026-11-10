//! ## Overview
//! Internationalization (i18n) module managing bilingual Korean/English localization.
//! Provides browser/OS auto-detection, URL query parameter parsing, and localStorage persistence.
//!
//! ## Search Tags
//! #i18n, #localization, #bilingual, #korean, #english
//!
//! ## Submodules
//! - [`contact`] (`contact.rs`): Inquiry form inputs, scopes, target profiles, and buffer texts.
//! - [`hero`] (`hero.rs`): Flagship hero headings, quick spec pills, and navbar/footer labels.
//! - [`specs`] (`specs.rs`): 10-row Swiss engineering specification matrix table.
//! - [`studio`] (`studio.rs`): Studio turnkey comparison cards and 4-phase engineering pipeline.
//! - [`teardown`] (`teardown/mod.rs`): 4-layer anatomical engineering teardown models and telemetry.

pub mod contact;
pub mod hero;
pub mod specs;
pub mod studio;
pub mod teardown;

pub use contact::{ContactBufferStrings, ContactFormStrings, ContactInfoStrings, ContactSelectorStrings, ContactStrings};
pub use hero::{HeroStrings, NavStrings};
pub use specs::SpecRowData;
pub use studio::StudioStrings;
pub use teardown::TeardownLayerData;

/// Supported application localization languages.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub enum Language {
    /// English locale (default for international visitors).
    #[default]
    En,
    /// Korean locale (default for Korean browser/OS environments).
    Ko,
}

impl Language {
    /// Detects the target language from localStorage, URL query parameters, or browser navigator.
    pub fn detect() -> Self {
        #[cfg(target_arch = "wasm32")]
        {
            if let Some(window) = web_sys::window() {
                // 1. Check localStorage key 'holtlab_lang'
                if let Ok(Some(storage)) = window.local_storage() {
                    if let Ok(Some(val)) = storage.get_item("holtlab_lang") {
                        if val.eq_ignore_ascii_case("ko") {
                            return Language::Ko;
                        } else if val.eq_ignore_ascii_case("en") {
                            return Language::En;
                        }
                    }
                }

                // 2. Check URL query parameters (e.g. ?lang=ko or ?lang=en)
                if let Ok(location) = window.location().search() {
                    if location.contains("lang=ko") {
                        return Language::Ko;
                    } else if location.contains("lang=en") {
                        return Language::En;
                    }
                }

                // 3. Fallback to navigator.language
                if let Some(lang) = window.navigator().language() {
                    if lang.to_lowercase().starts_with("ko") {
                        return Language::Ko;
                    }
                }
            }
        }
        Language::En
    }

    /// Persists the selected language to localStorage under the key 'holtlab_lang'.
    pub fn save(&self) {
        #[cfg(target_arch = "wasm32")]
        {
            if let Some(window) = web_sys::window() {
                if let Ok(Some(storage)) = window.local_storage() {
                    let code = match self {
                        Language::Ko => "ko",
                        Language::En => "en",
                    };
                    let _ = storage.set_item("holtlab_lang", code);
                }
            }
        }
    }
}
