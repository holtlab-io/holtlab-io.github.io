//! ## Overview
//! Studio turnkey engineering architecture and methodology page view.
//! Explains the unified turnkey co-design philosophy, compares fragmented agencies against Holt Lab, and details the 4-phase pipeline.
//!
//! ## Search Tags
//! #studio, #methodology, #turnkey, #pipeline, #co-design
//!
//! ## Submodules
//! None. This module provides atomic view composition.

use dioxus::prelude::*;
use crate::components::{
    studio::StudioTurnkey,
    teasers::InquiryCtaBanner,
};
use crate::i18n::{studio::get_studio_strings, Language};

/// Dedicated studio turnkey architecture and methodology page view.
#[component]
pub fn StudioPage() -> Element {
    let lang = use_context::<Signal<Language>>();
    let strings = get_studio_strings(lang());

    rsx! {
        div {
            class: "w-full",
            section {
                id: "studio-hero",
                class: "pt-12 sm:pt-16 pb-8 bg-[#FAFAFA] border-b border-zinc-200 text-center font-sans relative overflow-hidden",
                div { class: "absolute inset-0 bg-tech-dots opacity-35 pointer-events-none" }
                div {
                    class: "max-w-4xl mx-auto px-4 sm:px-8 relative z-10 space-y-4 reveal-on-scroll",
                    div {
                        class: "inline-flex items-center gap-2 px-3 py-1 rounded-full bg-white border border-zinc-200 text-zinc-700 font-mono text-[11px] font-semibold tracking-wider uppercase shadow-2xs",
                        span { class: "w-2 h-2 rounded-full bg-[#E64A19] animate-pulse" }
                        match lang() {
                            Language::En => "STUDIO ARCHITECTURE // METHODOLOGY",
                            Language::Ko => "스튜디오 아키텍처 // 엔지니어링 방법론",
                        }
                    }
                    h1 {
                        class: "text-3xl sm:text-5xl font-bold font-sans tracking-tight text-zinc-950 lowercase leading-tight",
                        "{strings.title}"
                    }
                    p {
                        class: "text-sm sm:text-base text-zinc-600 font-normal leading-relaxed max-w-2xl mx-auto",
                        "{strings.subtitle}"
                    }
                }
            }

            StudioTurnkey {}
            InquiryCtaBanner {}
        }
    }
}
