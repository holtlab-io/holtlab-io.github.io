//! ## Overview
//! Brand logo lockup and visual identity components for the Holt Lab header.
//! Features lowercase industrial brand typography paired with the localized hardware engineering studio descriptor.
//!
//! ## Search Tags
//! #brand, #logo, #identity, #typography, #studio-mark, #router
//!
//! ## Submodules
//! None. This module provides pure leaf brand presentation components.

use dioxus::prelude::*;
use crate::i18n::{hero::get_nav_strings, Language};
use crate::routes::Route;

/// Primary brand identity lockup rendering lowercase holt lab with the studio descriptor.
#[component]
pub fn NavBrand() -> Element {
    let lang = use_context::<Signal<Language>>();
    let nav_strings = get_nav_strings(lang());

    rsx! {
        Link {
            to: Route::Home {},
            class: "flex items-center gap-3 select-none group shrink-0",
            div {
                class: "w-8 h-8 rounded-xs bg-zinc-950 flex items-center justify-center text-white shadow-2xs group-hover:bg-[#E64A19] transition-colors",
                svg {
                    view_box: "0 0 24 24",
                    fill: "none",
                    class: "w-4 h-4",
                    "aria-hidden": "true",
                    path {
                        d: "M5.5 4.5V19.5M18.5 4.5V19.5M5.5 12H13.5L15.5 9.5H18.5",
                        stroke: "currentColor",
                        stroke_width: "2.2",
                        stroke_linecap: "square",
                        stroke_linejoin: "miter",
                    }
                    rect {
                        x: "11",
                        y: "14.5",
                        width: "3",
                        height: "3",
                        fill: "#E64A19",
                        class: "group-hover:fill-white transition-colors",
                    }
                }
            }
            div {
                class: "flex flex-col",
                span {
                    class: "text-lg sm:text-xl font-bold font-sans tracking-tight leading-none lowercase text-zinc-950 group-hover:text-zinc-700 transition-colors",
                    "holt lab"
                }
                span {
                    class: "hidden sm:inline-block text-[10px] sm:text-[11px] font-mono text-zinc-400 leading-tight tracking-tight pt-0.5 truncate max-w-[280px] md:max-w-none",
                    "{nav_strings.descriptor}"
                }
            }
        }
    }
}

/// Geometric studio glyph badge indicator for desktop viewports.
#[component]
pub fn NavStudioBadge() -> Element {
    let lang = use_context::<Signal<Language>>();
    let nav_strings = get_nav_strings(lang());

    rsx! {
        div {
            class: "hidden 2xl:flex items-center gap-2 pl-3 border-l border-zinc-200 text-[10px] font-mono text-zinc-400 select-none",
            span { class: "w-1.5 h-1.5 rounded-full bg-emerald-500 animate-pulse" }
            span { "{nav_strings.turnkey_badge}" }
        }
    }
}
