//! ## Overview
//! Desktop navigation link row and studio dispatch CTA button for the Holt Lab header.
//! Routes visitors across home, demonstrator, studio, and inquiry pages with bilingual language toggle.
//!
//! ## Search Tags
//! #navigation, #router, #links, #cta, #dispatch, #framework-style, #language-toggle
//!
//! ## Submodules
//! None. This module provides atomic desktop navigation components.

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

/// Desktop navigation link strip, language toggle, and dispatch CTA button.
#[component]
pub fn NavLinks() -> Element {
    let lang = use_context::<Signal<Language>>();
    let nav_strings = get_nav_strings(lang());
    let current_route = use_route::<Route>();

    rsx! {
        div {
            class: "hidden lg:flex items-center gap-5 xl:gap-7 select-none",
            nav {
                class: "flex items-center gap-4 xl:gap-6 text-xs font-mono lowercase tracking-wide",
                for (path, label, idx) in nav_strings.nav_items.iter() {
                    Link {
                        key: "{path}",
                        to: route_for_path(path),
                        class: if current_route == route_for_path(path) {
                            "group flex items-baseline gap-1 text-zinc-950 font-bold transition-colors py-1 border-b-2 border-[#E64A19]"
                        } else {
                            "group flex items-baseline gap-1 text-zinc-600 hover:text-zinc-950 transition-colors py-1"
                        },
                        span {
                            class: if current_route == route_for_path(path) {
                                "text-[9px] text-[#E64A19] font-bold"
                            } else {
                                "text-[9px] text-zinc-400 group-hover:text-[#E64A19] transition-colors"
                            },
                            "{idx}."
                        }
                        span { "{label}" }
                    }
                }
            }

            NavLangToggle { lang }

            Link {
                to: Route::ContactPage {},
                class: "inline-flex items-center gap-2 px-3.5 py-1.5 rounded-xs bg-[#E64A19] hover:bg-[#c93c12] text-white font-mono text-xs font-semibold tracking-wider transition-colors shadow-2xs shrink-0",
                span { "{nav_strings.cta_inquire}" }
                span { class: "text-[11px]", "→" }
            }
        }
    }
}

/// Single-button tactile pill toggle switching immediately between Korean and English.
#[component]
fn NavLangToggle(mut lang: Signal<Language>) -> Element {
    let is_ko = lang() == Language::Ko;

    rsx! {
        button {
            r#type: "button",
            aria_label: if is_ko { "Switch language to English" } else { "한국어로 언어 전환" },
            title: if is_ko { "Switch to English (EN)" } else { "한국어로 전환 (KR)" },
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
                    "px-2 py-0.5 rounded-2xs bg-zinc-950 text-white shadow-2xs transition-all"
                } else {
                    "px-2 py-0.5 text-zinc-500 hover:text-zinc-950 transition-colors"
                },
                "KR"
            }
            span {
                class: if !is_ko {
                    "px-2 py-0.5 rounded-2xs bg-zinc-950 text-white shadow-2xs transition-all"
                } else {
                    "px-2 py-0.5 text-zinc-500 hover:text-zinc-950 transition-colors"
                },
                "EN"
            }
        }
    }
}
