//! ## Overview
//! Minimalist wireframe navigation bar and subcategory filter strip modeled after Teenage Engineering.
//! Features lowercase typography, tactile line-art icons, micro-manifesto text, and live category filtering.
//!
//! ## Search Tags
//! #navbar, #header, #filter, #navigation, #teenage-engineering
//!
//! ## Submodules
//! - [`brand`] (`brand.rs`): Brand logo lockup, technical Korean micro-manifesto, and geometric studio mark.
//! - [`columns`] (`columns.rs`): Desktop 4-column wireframe navigation menu.
//! - [`filter`] (`filter.rs`): Subcategory filter strip and responsive mobile drawer.

use dioxus::prelude::*;

pub mod brand;
pub mod columns;
pub mod filter;

pub use brand::{NavBrand, NavKoreanManifesto, NavStudioMark};
pub use columns::NavColumns;
pub use filter::{FilterStrip, NavMobileDrawer};

/// Primary navigation bar component rendering the wireframe header and category filter strip.
#[component]
pub fn Navbar(active_filter: Signal<&'static str>) -> Element {
    let mut is_mobile_open = use_signal(|| false);

    rsx! {
        header {
            class: "sticky top-0 left-0 right-0 z-40 bg-white/95 backdrop-blur-md transition-all duration-200",
            div {
                class: "max-w-[1600px] mx-auto px-4 sm:px-8 pt-6 pb-2 flex items-start justify-between gap-4 md:gap-8",
                NavBrand {}
                NavColumns {}
                NavKoreanManifesto {}
                div {
                    class: "flex items-center gap-3",
                    NavStudioMark {}
                    button {
                        r#type: "button",
                        aria_label: "Toggle Navigation Menu",
                        class: "lg:hidden p-2 text-zinc-900 font-mono text-base select-none",
                        onclick: move |_| is_mobile_open.set(!is_mobile_open()),
                        if is_mobile_open() { "✕" } else { "☰" }
                    }
                }
            }

            FilterStrip { active_filter }

            if is_mobile_open() {
                NavMobileDrawer { is_open: is_mobile_open, active_filter }
            }
        }
    }
}
