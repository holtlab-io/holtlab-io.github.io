//! ## Overview
//! Industrial minimal navigation bar modeled after Framework's hardware demonstrator architecture.
//! Encapsulates the studio brand lockup, anchor navigation links, and direct inquiry CTA button.
//!
//! ## Search Tags
//! #navbar, #header, #navigation, #framework-style, #branding
//!
//! ## Submodules
//! - [`brand`] (`brand.rs`): Brand logo lockup, studio descriptor, and turnkey badge.
//! - [`mobile`][mod@mobile] (`mobile.rs`): Responsive mobile navigation drawer and touch menu.
//! - [`nav_links`] (`nav_links.rs`): Desktop navigation anchors and matte vermilion inquiry CTA.

use dioxus::prelude::*;
use crate::routes::Route;

pub mod brand;
pub mod mobile;
pub mod nav_links;

pub use brand::{NavBrand, NavStudioBadge};
pub use mobile::MobileDrawer;
pub use nav_links::NavLinks;

/// Primary navigation bar component rendered sticky at the top of the viewport.
#[component]
pub fn Navbar() -> Element {
    let mut is_mobile_open = use_signal(|| false);

    rsx! {
        header {
            class: "sticky top-0 left-0 right-0 z-50 bg-white/90 backdrop-blur-md border-b border-zinc-200/80 transition-all duration-200",
            div {
                class: "max-w-[1500px] mx-auto px-4 sm:px-8 h-16 sm:h-18 flex items-center justify-between gap-4",
                div {
                    class: "flex items-center gap-4",
                    NavBrand {}
                    NavStudioBadge {}
                }
                NavLinks {}
                div {
                    class: "lg:hidden flex items-center gap-2",
                    Link {
                        to: Route::ContactPage {},
                        class: "px-2.5 py-1 rounded-xs bg-[#E64A19] text-white font-mono text-[11px] font-semibold tracking-wider",
                        "inquire"
                    }
                    button {
                        r#type: "button",
                        aria_label: "Toggle Navigation Menu",
                        class: "p-2 text-zinc-900 font-mono text-base select-none",
                        onclick: move |_| is_mobile_open.set(!is_mobile_open()),
                        if is_mobile_open() { "✕" } else { "☰" }
                    }
                }
            }

            if is_mobile_open() {
                MobileDrawer { is_open: is_mobile_open }
            }
        }
    }
}
