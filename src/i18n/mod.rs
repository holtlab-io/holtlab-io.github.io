//! ## Overview
//! Internationalization (i18n) module managing bilingual Korean/English localization.
//! Provides browser/OS auto-detection, URL query parameter parsing, and localStorage persistence.
//!
//! ## Search Tags
//! #i18n, #localization, #bilingual, #korean, #english
//!
//! ## Submodules
//! - [`capabilities`] (`capabilities.rs`): Studio engineering capability cards and discipline models.
//! - [`contact`] (`contact.rs`): Inquiry form inputs, scopes, and buffer texts.
//! - [`hero`] (`hero.rs`): Flagship hero headings, quick spec pills, and navbar/footer labels.
//! - [`specs`] (`specs.rs`): 10-row Swiss engineering specification matrix table.
//! - [`studio`] (`studio.rs`): Studio turnkey comparison cards and 4-phase engineering pipeline.
//! - [`teardown`] (`teardown/mod.rs`): 4-layer anatomical engineering teardown models and telemetry.
//! - [`teasers`] (`teasers.rs`): Localized presentation strings for studio teasers and summary cards.

pub mod capabilities;
pub mod contact;
pub mod hero;
pub mod specs;
pub mod studio;
pub mod teardown;
pub mod teasers;

pub use capabilities::{CapabilitiesStrings, CapabilityCard};
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
    /// Detects the target language from URL query parameters, localStorage, or browser navigator.
    pub fn detect() -> Self {
        #[cfg(target_arch = "wasm32")]
        {
            if let Some(window) = web_sys::window() {
                // 1. Explicit URL query parameters take highest priority (?lang=ko or ?lang=en)
                if let Ok(location) = window.location().search() {
                    let search_lower = location.to_ascii_lowercase();
                    if search_lower.contains("lang=ko") {
                        let detected = Language::Ko;
                        detected.save();
                        return detected;
                    } else if search_lower.contains("lang=en") {
                        let detected = Language::En;
                        detected.save();
                        return detected;
                    }
                }

                // 2. Check localStorage key 'holtlab_lang'
                if let Ok(Some(storage)) = window.local_storage() {
                    if let Ok(Some(val)) = storage.get_item("holtlab_lang") {
                        let val_lower = val.to_ascii_lowercase();
                        if val_lower.starts_with("ko") {
                            return Language::Ko;
                        } else if val_lower.starts_with("en") {
                            return Language::En;
                        }
                    }
                }

                // 3. Fallback to navigator.language
                if let Some(lang) = window.navigator().language() {
                    if lang.to_ascii_lowercase().starts_with("ko") {
                        return Language::Ko;
                    }
                }
            }
        }
        Language::En
    }

    /// Persists the selected language to localStorage and updates URL query parameters.
    pub fn save(&self) {
        #[cfg(target_arch = "wasm32")]
        {
            if let Some(window) = web_sys::window() {
                let code = match self {
                    Language::Ko => "ko",
                    Language::En => "en",
                };
                if let Ok(Some(storage)) = window.local_storage() {
                    let _ = storage.set_item("holtlab_lang", code);
                }
                if let Ok(history) = window.history() {
                    let hash = window.location().hash().unwrap_or_default();
                    let new_url = format!("?lang={}{}", code, hash);
                    let _ = history.replace_state_with_url(&wasm_bindgen::JsValue::NULL, "", Some(&new_url));
                }
            }
        }
    }
}
