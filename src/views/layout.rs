//! ## Overview
//! Studio application layout component providing persistent navigation and structural framing.
//! Manages route transition scroll resets, DOM scroll reveal triggers, and page-level footer.
//!
//! ## Search Tags
//! #layout, #shell, #router, #navigation, #outlet
//!
//! ## Submodules
//! None. This module provides atomic layout frame components.

use dioxus::prelude::*;
use crate::components::{footer::Footer, navbar::Navbar};
use crate::routes::Route;

/// Top-level application layout frame wrapping routed views with sticky navbar and footer.
#[component]
pub fn StudioLayout() -> Element {
    let route = use_route::<Route>();

    use_effect(use_reactive!(|(route,)| {
        let _ = &route;
        #[cfg(target_arch = "wasm32")]
        {
            if let Some(w) = web_sys::window() {
                w.scroll_to_with_x_and_y(0.0, 0.0);
            }
            let _ = js_sys::eval("if (window.attachScrollReveals) window.attachScrollReveals();");
        }
    }));

    rsx! {
        div {
            class: "min-h-screen bg-[#FAFAFA] text-zinc-900 flex flex-col selection:bg-zinc-900 selection:text-white",
            Navbar {}
            main {
                class: "flex-grow",
                Outlet::<Route> {}
            }
            Footer {}
        }
    }
}
