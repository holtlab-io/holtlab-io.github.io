//! ## Overview
//! Desktop navigation link row and studio dispatch CTA button for the Holt Lab header.
//! Routes visitors to demonstrator, teardown, specifications, studio turnkey, and inquiry sections.
//!
//! ## Search Tags
//! #navigation, #anchors, #cta, #dispatch, #framework-style
//!
//! ## Submodules
//! None. This module provides atomic desktop navigation components.

use dioxus::prelude::*;

const NAV_ITEMS: [(&str, &str, &str); 4] = [
    ("#demonstrator", "demonstrator", "01"),
    ("#teardown", "teardown", "02"),
    ("#specs", "specs", "03"),
    ("#studio", "studio", "04"),
];

/// Desktop navigation link strip and dispatch CTA button.
#[component]
pub fn NavLinks() -> Element {
    rsx! {
        div {
            class: "hidden md:flex items-center gap-7 lg:gap-9 select-none",
            nav {
                class: "flex items-center gap-6 lg:gap-8 text-xs font-mono lowercase tracking-wide",
                for (anchor, label, idx) in NAV_ITEMS.iter() {
                    a {
                        key: "{anchor}",
                        href: "{anchor}",
                        class: "group flex items-baseline gap-1 text-zinc-600 hover:text-zinc-950 transition-colors py-1",
                        span { class: "text-[9px] text-zinc-400 group-hover:text-[#E64A19] transition-colors", "{idx}." }
                        span { "{label}" }
                    }
                }
            }
            a {
                href: "#inquire",
                class: "inline-flex items-center gap-2 px-3.5 py-1.5 rounded-xs bg-[#E64A19] hover:bg-[#c93c12] text-white font-mono text-xs font-semibold tracking-wider transition-colors shadow-2xs",
                span { "inquire // 의뢰하기" }
                span { class: "text-[11px]", "→" }
            }
        }
    }
}
