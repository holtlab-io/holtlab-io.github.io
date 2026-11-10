//! ## Overview
//! Interactive selection buttons for project scope and client organization category.
//! Provides localized options for engineering disciplines and organizational scope.
//!
//! ## Search Tags
//! #contact, #selectors, #scope, #buttons, #options
//!
//! ## Submodules
//! None. This module provides atomic selector buttons components.

use dioxus::prelude::*;
use crate::i18n::{contact::get_contact_strings, Language};

/// Selector buttons for development scope.
#[component]
pub fn ScopeButtons(mut selected: Signal<usize>) -> Element {
    let lang = use_context::<Signal<Language>>();
    let selectors = get_contact_strings(lang()).selectors;

    rsx! {
        div {
            class: "space-y-2",
            label { class: "block text-xs font-mono text-zinc-500 lowercase", "{selectors.discipline_label}" }
            div {
                class: "grid grid-cols-1 sm:grid-cols-2 gap-2 text-xs font-mono lowercase",
                for (idx, scope) in selectors.scopes.iter().enumerate() {
                    button {
                        key: "{scope}",
                        r#type: "button",
                        onclick: move |_| selected.set(idx),
                        class: if selected() == idx {
                            "p-3 rounded-xs border-2 border-zinc-950 bg-zinc-100 text-zinc-950 font-bold text-left transition-all"
                        } else {
                            "p-3 rounded-xs border border-zinc-200 bg-zinc-50/50 text-zinc-600 hover:border-zinc-300 hover:text-zinc-950 text-left transition-all"
                        },
                        "{scope}"
                    }
                }
            }
        }
    }
}

/// Selector buttons for client organizational category.
#[component]
pub fn TargetButtons(mut selected: Signal<usize>) -> Element {
    let lang = use_context::<Signal<Language>>();
    let selectors = get_contact_strings(lang()).selectors;

    rsx! {
        div {
            class: "space-y-2",
            label { class: "block text-xs font-mono text-zinc-500 lowercase", "{selectors.profile_label}" }
            div {
                class: "grid grid-cols-2 sm:grid-cols-4 gap-2 text-xs font-mono lowercase",
                for (idx, target) in selectors.targets.iter().enumerate() {
                    button {
                        key: "{target}",
                        r#type: "button",
                        onclick: move |_| selected.set(idx),
                        class: if selected() == idx {
                            "p-2.5 rounded-xs border-2 border-zinc-950 bg-zinc-100 text-zinc-950 font-bold text-center"
                        } else {
                            "p-2.5 rounded-xs border border-zinc-200 bg-zinc-50/50 text-zinc-600 hover:border-zinc-300 hover:text-zinc-950 text-center"
                        },
                        "{target}"
                    }
                }
            }
        }
    }
}
