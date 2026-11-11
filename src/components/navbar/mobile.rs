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
use crate::routes::Route;

fn route_for_path(path: &str) -> Route {
    match path {
        "/demonstrator" => Route::Demonstrator {},
        "/studio" => Route::StudioPage {},
        "/contact" => Route::ContactPage {},
        _ => Route::Home {},
    }
}

/// Mobile drawer overlay presenting full-screen navigation anchors and language switcher.
#[component]
pub fn MobileDrawer(mut is_open: Signal<bool>) -> Element {
    let lang = use_context::<Signal<Language>>();
    let nav_strings = get_nav_strings(lang());

    rsx! {
        div {
            class: "lg:hidden bg-white/98 backdrop-blur-md border-b border-zinc-200 px-6 py-6 font-mono text-xs transition-all animate-fadeIn",
            MobileLangToggle { lang }
            MobileNavList { is_open, items: nav_strings.mobile_items }
            MobileDrawerFooter {}
        }
    }
}

/// Mobile language switch bar inside the drawer using a unified toggle pill.
#[component]
fn MobileLangToggle(mut lang: Signal<Language>) -> Element {
    let is_ko = lang() == Language::Ko;

    rsx! {
        div {
            class: "flex items-center justify-between pb-4 mb-4 border-b border-zinc-100",
            span {
                class: "text-[11px] text-zinc-500 font-mono",
                if is_ko { "언어 설정" } else { "LANGUAGE" }
            }
            button {
                r#type: "button",
                aria_label: if is_ko { "Switch language to English" } else { "한국어로 언어 전환" },
                onclick: move |_| {
                    let next = if lang() == Language::Ko {
                        Language::En
                    } else {
                        Language::Ko
                    };
                    lang.set(next);
                    next.save();
                },
                class: "inline-flex items-center rounded-xs border border-zinc-200 hover:border-zinc-400 bg-zinc-50 p-0.5 font-mono text-[11px] font-semibold cursor-pointer transition-colors select-none",
                span {
                    class: if is_ko {
                        "px-3 py-1 rounded-2xs bg-zinc-950 text-white shadow-2xs transition-all"
                    } else {
                        "px-3 py-1 text-zinc-500 hover:text-zinc-950 transition-colors"
                    },
                    "KR"
                }
                span {
                    class: if !is_ko {
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
    items: [(&'static str, &'static str, &'static str); 4],
) -> Element {
    rsx! {
        div {
            class: "flex flex-col divide-y divide-zinc-100",
            for (path, label, desc) in items.iter() {
                Link {
                    key: "{path}",
                    to: route_for_path(path),
                    onclick: move |_| is_open.set(false),
                    class: "py-3 flex items-center justify-between group hover:text-zinc-950 transition-colors",
                    div {
                        class: "flex flex-col",
                        span {
                            class: if *path == "/contact" {
                                "text-sm font-bold text-[#E64A19] group-hover:underline"
                            } else {
                                "text-sm font-medium text-zinc-900 group-hover:text-[#E64A19]"
                            },
                            "{label}"
                        }
                        span { class: "text-[10px] text-zinc-400 font-sans pt-0.5", "{desc}" }
                    }
                    span { class: "text-zinc-300 font-mono text-xs group-hover:text-[#E64A19] transition-colors", "→" }
                }
            }
        }
    }
}

/// Secondary status footer inside the mobile drawer.
#[component]
fn MobileDrawerFooter() -> Element {
    rsx! {
        div {
            class: "mt-6 pt-4 border-t border-zinc-100 flex items-center justify-between text-[11px] text-zinc-400 font-mono",
            span { "holtlab.io@gmail.com" }
            span { class: "text-zinc-500 font-medium", "SEOUL, KR" }
        }
    }
}
