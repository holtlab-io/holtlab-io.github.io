//! ## Overview
//! Primary landing page view for the Holt Lab web application.
//! Orchestrates the Teenage Engineering inspired hardware catalog, spec matrix, and inquiry dispatch.
//!
//! ## Search Tags
//! #home, #landing, #catalog, #view, #layout

use dioxus::prelude::*;
use crate::components::{
    navbar::Navbar,
    catalog::Catalog,
    specs::SpecsMatrix,
    philosophy::Philosophy,
    contact::Contact,
    footer::Footer,
};

/// Primary single-page catalog view composing all structural sections.
#[component]
pub fn Home() -> Element {
    let active_filter = use_signal(|| "all");

    rsx! {
        div {
            class: "min-h-screen bg-white text-zinc-900 flex flex-col selection:bg-zinc-900 selection:text-white",
            Navbar { active_filter }
            main {
                class: "flex-grow",
                Catalog { active_filter }
                SpecsMatrix {}
                Philosophy {}
                Contact {}
            }
            Footer {}
        }
    }
}
