//! ## Overview
//! Mobile navigation drawer and responsive menu overlay for the Holt Lab header.
//! Provides accessible touch navigation anchors, language switcher, and direct studio inquiry trigger on small viewports.
//!
//! ## Search Tags
//! #mobile, #drawer, #hamburger, #responsive, #navigation, #language-toggle
//!
//! ## Submodules
//! None. This module provides atomic mobile overlay components.

use dioxus::prelude::*;
use crate::i18n::{hero::get_nav_strings, Language};

/// Mobile drawer overlay presenting full-screen navigation anchors and language switcher.
#[component]
pub fn MobileDrawer(mut is_open: Signal<bool>) -> Element {
    let lang = use_context::<Signal<Language>>();
    let nav_strings = get_nav_strings(lang());

    rsx! {
        div {
            class: "lg:hidden bg-white/98 backdrop-blur-md border-b border-zinc-200 px-6 py-6 font-mono text-xs lowercase transition-all animate-fadeIn",
            MobileLangToggle { lang }
            MobileNavList { is_open, items: nav_strings.mobile_items }
            MobileDrawerFooter {}
        }
    }
}

/// Mobile language switch bar inside the drawer.
#[component]
fn MobileLangToggle(mut lang: Signal<Language>) -> Element {
    rsx! {
        div {
            class: "flex items-center justify-between pb-4 mb-4 border-b border-zinc-100",
            span { class: "text-[11px] text-zinc-500 font-mono", "LANGUAGE // 언어" }
            div {
                class: "inline-flex items-center rounded-xs border border-zinc-200 bg-zinc-50 p-0.5 font-mono text-[11px] font-semibold",
                button {
                    r#type: "button",
                    aria_label: "Switch to Korean",
                    onclick: move |_| {
                        lang.set(Language::Ko);
                        Language::Ko.save();
                    },
                    class: if lang() == Language::Ko {
                        "px-3 py-1 rounded-2xs bg-zinc-950 text-white shadow-2xs transition-all"
                    } else {
                        "px-3 py-1 text-zinc-500 hover:text-zinc-950 transition-colors"
                    },
                    "KR"
                }
                button {
                    r#type: "button",
                    aria_label: "Switch to English",
                    onclick: move |_| {
                        lang.set(Language::En);
                        Language::En.save();
                    },
                    class: if lang() == Language::En {
                        "px-3 py-1 rounded-2xs bg-zinc-950 text-white shadow-2xs transition-all"
                    } else {
                        "px-3 py-1 text-zinc-500 hover:text-zinc-950 transition-colors"
                    },
                    "EN"
                }
            }
        }
    }
}

/// Navigation anchor list inside the mobile drawer.
#[component]
fn MobileNavList(
    mut is_open: Signal<bool>,
    items: [(&'static str, &'static str, &'static str); 5],
) -> Element {
    rsx! {
        div {
            class: "flex flex-col divide-y divide-zinc-100",
            for (anchor, label, desc) in items.iter() {
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
    }
}

/// Minimalist footer telemetry strip inside the mobile drawer.
#[component]
fn MobileDrawerFooter() -> Element {
    rsx! {
        div {
            class: "pt-4 mt-2 border-t border-zinc-200 flex items-center justify-between text-[10px] text-zinc-400",
            span { "HOLT LAB // 0.00 mm³ CAD" }
            span { "ESP32-C3 RISC-V @ 160MHz" }
        }
    }
}
