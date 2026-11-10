//! ## Overview
//! Sleek minimal pill selection buttons for choosing the target engineering discipline.
//! Displays selected and unselected pill states with zero line wrap issues on mobile devices.
//!
//! ## Search Tags
//! #contact, #selectors, #discipline, #pills, #buttons
//!
//! ## Submodules
//! None. This module provides atomic discipline pill selector components.

use dioxus::prelude::*;
use crate::i18n::{contact::get_contact_strings, Language};

/// Minimal pill buttons for selecting the engineering development discipline.
#[component]
pub fn ScopeButtons(mut selected: Signal<usize>) -> Element {
    let lang = use_context::<Signal<Language>>();
    let selectors = get_contact_strings(lang()).selectors;

    rsx! {
        div {
            class: "space-y-2.5",
            label { class: "block text-xs font-mono text-zinc-500 lowercase", "{selectors.discipline_label}" }
            div {
                class: "flex flex-wrap gap-2 text-xs",
                for (idx, scope) in selectors.scopes.iter().enumerate() {
                    button {
                        key: "{scope}",
                        r#type: "button",
                        onclick: move |_| selected.set(idx),
                        class: if selected() == idx {
                            "inline-flex items-center gap-1.5 px-3.5 py-2 rounded-full border border-zinc-950 bg-zinc-950 text-white font-medium text-xs shadow-2xs whitespace-nowrap transition-all cursor-pointer"
                        } else {
                            "inline-flex items-center gap-1.5 px-3.5 py-2 rounded-full border border-zinc-200 bg-white hover:border-zinc-300 text-zinc-700 hover:text-zinc-950 font-normal text-xs whitespace-nowrap transition-all cursor-pointer"
                        },
                        span {
                            class: if selected() == idx { "text-white text-[11px] leading-none" } else { "text-zinc-400 text-[11px] leading-none" },
                            if selected() == idx { "●" } else { "○" }
                        }
                        span { class: "tracking-tight", "{scope}" }
                    }
                }
            }
        }
    }
}
