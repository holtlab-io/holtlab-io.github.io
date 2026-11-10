//! ## Overview
//! Mobile navigation drawer and responsive menu overlay for the Holt Lab header.
//! Provides accessible touch navigation anchors and direct studio inquiry trigger on small viewports.
//!
//! ## Search Tags
//! #mobile, #drawer, #hamburger, #responsive, #navigation
//!
//! ## Submodules
//! None. This module provides atomic mobile overlay components.

use dioxus::prelude::*;

const MOBILE_ITEMS: [(&str, &str, &str); 5] = [
    ("#demonstrator", "demonstrator", "flagship hardware machine"),
    ("#teardown", "teardown", "4-layer engineering teardown"),
    ("#specs", "specs", "monospaced swiss spec matrix"),
    ("#studio", "studio", "1-person turnkey model"),
    ("#inquire", "inquire // 의뢰하기", "direct architect dispatch"),
];

/// Mobile drawer overlay presenting full-screen navigation anchors.
#[component]
pub fn MobileDrawer(mut is_open: Signal<bool>) -> Element {
    rsx! {
        div {
            class: "md:hidden bg-white/98 backdrop-blur-md border-b border-zinc-200 px-6 py-6 font-mono text-xs lowercase transition-all animate-fadeIn",
            div {
                class: "flex flex-col divide-y divide-zinc-100",
                for (anchor, label, desc) in MOBILE_ITEMS.iter() {
                    a {
                        key: "{anchor}",
                        href: "{anchor}",
                        onclick: move |_| is_open.set(false),
                        class: "py-3 flex items-center justify-between group hover:text-zinc-950 transition-colors",
                        div {
                            class: "flex flex-col",
                            span {
                                class: if *anchor == "#inquire" {
                                    "text-sm font-bold text-[#E64A19] group-hover:underline"
                                } else {
                                    "text-sm font-medium text-zinc-900 group-hover:text-[#E64A19]"
                                },
                                "{label}"
                            }
                            span { class: "text-[10px] text-zinc-400 font-sans pt-0.5", "{desc}" }
                        }
                        span { class: "text-zinc-400 group-hover:translate-x-0.5 transition-transform", "→" }
                    }
                }
            }
            div {
                class: "pt-4 mt-2 border-t border-zinc-200 flex items-center justify-between text-[10px] text-zinc-400",
                span { "HOLT LAB // 0.00 mm³ CAD" }
                span { "ESP32-C3 RISC-V @ 160MHz" }
            }
        }
    }
}
