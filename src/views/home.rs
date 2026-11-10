//! ## Overview
//! Primary landing page view for the Holt Lab web application.
//! Orchestrates the full vertical section stack in a cohesive, off-white Swiss layout.
//!
//! ## Search Tags
//! #home, #landing, #view, #layout, #presentation

use dioxus::prelude::*;
use crate::components::{
    navbar::Navbar,
    hero::Hero,
    metrics::Metrics,
    comparison::Comparison,
    showcase::Showcase,
    services::Services,
    process::Process,
    contact::Contact,
    footer::Footer,
};

/// Primary single-page landing view composing all structural sections.
#[component]
pub fn Home() -> Element {
    rsx! {
        div {
            class: "min-h-screen bg-[#FAFAFA] text-zinc-900 flex flex-col selection:bg-orange-500 selection:text-white",
            Navbar {}
            main {
                class: "flex-grow",
                Hero {}
                Metrics {}
                Comparison {}
                Showcase {}
                Services {}
                Process {}
                Contact {}
            }
            Footer {}
        }
    }
}
