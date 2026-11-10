//! ## Overview
//! Desktop navigation link row and studio dispatch CTA button for the Holt Lab header.
//! Routes visitors to demonstrator, teardown, specifications, studio turnkey, and inquiry sections with language switcher.
//!
//! ## Search Tags
//! #navigation, #anchors, #cta, #dispatch, #framework-style, #language-toggle
//!
//! ## Submodules
//! None. This module provides atomic desktop navigation components.

use dioxus::prelude::*;
use crate::i18n::{hero::get_nav_strings, Language};

/// Desktop navigation link strip, language toggle, and dispatch CTA button.
#[component]
pub fn NavLinks() -> Element {
    let mut lang = use_context::<Signal<Language>>();
    let nav_strings = get_nav_strings(lang());

    rsx! {
        div {
            class: "hidden lg:flex items-center gap-5 xl:gap-7 select-none",
            nav {
                class: "flex items-center gap-4 xl:gap-6 text-xs font-mono lowercase tracking-wide",
                for (anchor, label, idx) in nav_strings.nav_items.iter() {
                    a {
                        key: "{anchor}",
                        href: "{anchor}",
                        class: "group flex items-baseline gap-1 text-zinc-600 hover:text-zinc-950 transition-colors py-1",
                        span { class: "text-[9px] text-zinc-400 group-hover:text-[#E64A19] transition-colors", "{idx}." }
                        span { "{label}" }
                    }
                }
            }

            div {
                class: "inline-flex items-center rounded-xs border border-zinc-200 bg-zinc-50 p-0.5 font-mono text-[11px] font-semibold",
                button {
                    r#type: "button",
                    aria_label: "Switch to Korean",
                    aria_pressed: if lang() == Language::Ko { "true" } else { "false" },
                    onclick: move |_| {
                        lang.set(Language::Ko);
                        Language::Ko.save();
                    },
                    class: if lang() == Language::Ko {
                        "px-2 py-0.5 rounded-2xs bg-zinc-950 text-white shadow-2xs transition-all"
                    } else {
                        "px-2 py-0.5 text-zinc-500 hover:text-zinc-950 transition-colors"
                    },
                    "KR"
                }
                button {
                    r#type: "button",
                    aria_label: "Switch to English",
                    aria_pressed: if lang() == Language::En { "true" } else { "false" },
                    onclick: move |_| {
                        lang.set(Language::En);
                        Language::En.save();
                    },
                    class: if lang() == Language::En {
                        "px-2 py-0.5 rounded-2xs bg-zinc-950 text-white shadow-2xs transition-all"
                    } else {
                        "px-2 py-0.5 text-zinc-500 hover:text-zinc-950 transition-colors"
                    },
                    "EN"
                }
            }

            a {
                href: "#inquire",
                class: "inline-flex items-center gap-2 px-3.5 py-1.5 rounded-xs bg-[#E64A19] hover:bg-[#c93c12] text-white font-mono text-xs font-semibold tracking-wider transition-colors shadow-2xs shrink-0",
                span { "{nav_strings.cta_inquire}" }
                span { class: "text-[11px]", "→" }
            }
        }
    }
}
