//! ## Overview
//! Minimalist 404 error fallback view displayed when an unknown route segment is requested.
//! Provides clear guidance and an immediate return path to the primary studio overview.
//!
//! ## Search Tags
//! #404, #not-found, #error, #fallback, #router
//!
//! ## Submodules
//! None. This module provides atomic view composition.

use dioxus::prelude::*;
use crate::routes::Route;

/// Fallback 404 error page displayed when an unmatched route is requested.
#[component]
pub fn NotFound(_segments: Vec<String>) -> Element {
    rsx! {
        div {
            class: "min-h-[60vh] flex flex-col items-center justify-center text-center p-8 font-sans",
            div {
                class: "space-y-4 max-w-md",
                span { class: "inline-block px-3 py-1 rounded-full bg-zinc-100 text-zinc-600 font-mono text-xs font-semibold tracking-wider", "404 // NOT FOUND" }
                h1 { class: "text-3xl sm:text-4xl font-bold tracking-tight text-zinc-950", "Specification Not Found" }
                p { class: "text-sm text-zinc-600 leading-relaxed", "The requested hardware engineering specification or route does not exist." }
                div {
                    class: "pt-4",
                    Link {
                        to: Route::Home {},
                        class: "tactile-button inline-flex items-center gap-2 px-5 py-2.5 rounded-xs bg-zinc-950 text-white font-mono text-xs font-semibold tracking-wider hover:bg-zinc-800 transition-colors shadow-xs",
                        "← return to studio overview"
                    }
                }
            }
        }
    }
}
