//! ## Overview
//! Deep flagship case study page view for Opus 01 (Win95 Minesweeper Touch Keyring).
//! Features the interactive 4-view CAD workbench, anatomical 4-layer exploded teardown, and Swiss engineering spec matrix.
//!
//! ## Search Tags
//! #demonstrator, #case-study, #teardown, #cad, #specs, #hardware
//!
//! ## Submodules
//! None. This module provides atomic view composition.

use dioxus::prelude::*;
use crate::components::{
    specs::SpecsMatrix,
    teardown::TeardownSection,
    teasers::InquiryCtaBanner,
    workbench::CadWorkbench,
};
use crate::i18n::{hero::get_hero_strings, Language};

/// Dedicated flagship hardware demonstrator page view for Opus 01.
#[component]
pub fn Demonstrator() -> Element {
    let lang = use_context::<Signal<Language>>();
    let strings = get_hero_strings(lang());

    rsx! {
        div {
            class: "w-full",
            section {
                id: "case-study-hero",
                class: "pt-12 sm:pt-16 pb-8 bg-[#FAFAFA] border-b border-zinc-200 text-center font-sans relative overflow-hidden",
                div { class: "absolute inset-0 bg-tech-grid opacity-40 pointer-events-none" }
                div {
                    class: "max-w-4xl mx-auto px-4 sm:px-8 relative z-10 space-y-4 reveal-on-scroll",
                    div {
                        class: "inline-flex items-center gap-2 px-3 py-1 rounded-full bg-white border border-zinc-200 text-zinc-700 font-mono text-[11px] font-semibold tracking-wider uppercase shadow-2xs",
                        span { class: "w-2 h-2 rounded-full bg-[#E64A19] animate-pulse" }
                        match lang() {
                            Language::En => "ENGINEERING SHOWCASE // CASE STUDY 01",
                            Language::Ko => "기술 쇼케이스 // 케이스 스터디 01",
                        }
                    }
                    h1 {
                        class: "text-3xl sm:text-5xl font-bold font-sans tracking-tight text-zinc-950 leading-tight break-keep",
                        "{strings.console_sub}"
                    }
                    p {
                        class: "text-sm sm:text-base text-zinc-600 font-normal leading-relaxed max-w-2xl mx-auto break-keep",
                        "{strings.case_study_subtitle}"
                    }
                }
            }

            section {
                class: "py-10 bg-[#FAFAFA] border-b border-zinc-200",
                div {
                    class: "max-w-[1500px] mx-auto px-4 sm:px-8",
                    CadWorkbench {}
                }
            }

            TeardownSection {}
            SpecsMatrix {}
            InquiryCtaBanner {}
        }
    }
}
