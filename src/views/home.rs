//! ## Overview
//! Primary landing page view for the Holt Lab hardware demonstrator showcase.
//! Orchestrates the Framework-inspired hero demonstrator, anatomical teardown, spec matrix, studio turnkey, and inquiry dispatch.
//!
//! ## Search Tags
//! #home, #landing, #demonstrator, #framework-style, #showcase
//!
//! ## Submodules
//! None. This module provides atomic view composition.

use dioxus::prelude::*;
use crate::components::{
    contact::Contact,
    footer::Footer,
    hero::HeroDemonstrator,
    navbar::Navbar,
    specs::SpecsMatrix,
    studio::StudioTurnkey,
    teardown::TeardownSection,
};

/// Primary single-page demonstrator view composing all structural engineering sections.
#[component]
pub fn Home() -> Element {
    rsx! {
        div {
            class: "min-h-screen bg-white text-zinc-900 flex flex-col selection:bg-zinc-900 selection:text-white",
            Navbar {}
            main {
                class: "flex-grow",
                HeroDemonstrator {}
                TeardownSection {}
                SpecsMatrix {}
                StudioTurnkey {}
                Contact {}
            }
            Footer {}
        }
    }
}
