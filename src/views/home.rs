//! ## Overview
//! Primary studio overview landing page view for Holt Lab.
//! Orchestrates the studio hero platform, core capabilities grid, featured demonstrator teaser, turnkey summary, and direct inquiry CTA.
//!
//! ## Search Tags
//! #home, #landing, #overview, #studio, #capabilities, #teaser
//!
//! ## Submodules
//! None. This module provides atomic view composition.

use dioxus::prelude::*;
use crate::components::{
    capabilities::StudioCapabilities,
    hero::HeroDemonstrator,
    teasers::{DemonstratorTeaser, InquiryCtaBanner, StudioTurnkeyTeaser},
};

/// Primary studio overview view composing foundational engineering capability and demonstrator sections.
#[component]
pub fn Home() -> Element {
    rsx! {
        div {
            class: "w-full",
            HeroDemonstrator {}
            StudioCapabilities {}
            DemonstratorTeaser {}
            StudioTurnkeyTeaser {}
            InquiryCtaBanner {}
        }
    }
}
