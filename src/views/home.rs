//! ## Overview
//! Primary landing page view for the Holt Lab hardware engineering studio platform.
//! Orchestrates the studio hero platform, core capabilities, flagship case study, specifications, turnkey model, and inquiry.
//!
//! ## Search Tags
//! #home, #landing, #studio, #platform, #capabilities, #case-study
//!
//! ## Submodules
//! None. This module provides atomic view composition.

use dioxus::prelude::*;
use crate::components::{
    capabilities::StudioCapabilities,
    contact::Contact,
    footer::Footer,
    hero::HeroDemonstrator,
    navbar::Navbar,
    specs::SpecsMatrix,
    studio::StudioTurnkey,
    teardown::TeardownSection,
};

/// Primary single-page studio view composing all structural engineering sections.
#[component]
pub fn Home() -> Element {
    rsx! {
        div {
            class: "min-h-screen bg-[#FAFAFA] text-zinc-900 flex flex-col selection:bg-zinc-900 selection:text-white",
            Navbar {}
            main {
                class: "flex-grow",
                HeroDemonstrator {}
                StudioCapabilities {}
                TeardownSection {}
                SpecsMatrix {}
                StudioTurnkey {}
                Contact {}
            }
            Footer {}
        }
    }
}
