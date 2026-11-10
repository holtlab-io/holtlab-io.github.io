//! ## Overview
//! Studio turnkey value proposition section explaining the unified turnkey hardware architecture.
//! Highlights the contrast between fragmented multi-agency ping-pong and Holt Lab's closed-loop co-design execution.
//!
//! ## Search Tags
//! #studio, #turnkey, #methodology, #co-design, #architecture
//!
//! ## Submodules
//! None. This module provides atomic studio methodology components.

use dioxus::prelude::*;
use crate::i18n::{studio::get_studio_strings, Language, StudioStrings};

/// Primary studio turnkey value proposition section rendered at `#studio`.
#[component]
pub fn StudioTurnkey() -> Element {
    let lang = use_context::<Signal<Language>>();
    let strings = get_studio_strings(lang());

    rsx! {
        section {
            id: "studio",
            class: "bg-[#FAFAFA] py-16 sm:py-24 border-t border-zinc-200 font-sans scroll-mt-20 relative",
            div { id: "turnkey", class: "absolute -top-20" }
            div {
                class: "max-w-[1500px] mx-auto px-4 sm:px-8",
                div {
                    class: "flex flex-col md:flex-row md:items-end justify-between border-b border-zinc-200 pb-6 mb-12 gap-4",
                    div {
                        class: "space-y-1.5",
                        span { class: "text-xs font-mono text-zinc-400 lowercase", "{strings.eyebrow}" }
                        h2 { class: "text-2xl sm:text-4xl font-bold font-sans tracking-tight text-zinc-950 lowercase", "{strings.title}" }
                    }
                    div {
                        class: "text-xs font-mono text-zinc-500 lowercase",
                        "{strings.subtitle}"
                    }
                }

                StudioComparisonGrid { strings }
                TurnkeyPipelineGrid { strings }
            }
        }
    }
}

/// Comparison grid contrasting traditional agency ping-pong with Holt Lab's unified loop.
#[component]
fn StudioComparisonGrid(strings: StudioStrings) -> Element {
    rsx! {
        div {
            class: "grid grid-cols-1 lg:grid-cols-2 gap-6 mb-16",

            // Fragmented Multi-Agency Model
            div {
                class: "studio-card p-6 sm:p-8 rounded-xs bg-white border border-zinc-200 border-l-4 border-l-red-400 space-y-4 shadow-2xs hover:border-zinc-300",
                div {
                    class: "flex items-center justify-between font-mono text-xs",
                    span { class: "text-red-600 font-bold uppercase", "{strings.traditional.badge}" }
                    span { class: "text-zinc-400", "{strings.traditional.metric}" }
                }
                h3 { class: "text-lg font-bold text-zinc-950 lowercase font-sans", "{strings.traditional.heading}" }
                p {
                    class: "text-xs sm:text-sm text-zinc-600 leading-relaxed font-normal",
                    "{strings.traditional.body}"
                }
                div {
                    class: "space-y-2 pt-2 text-xs font-mono text-zinc-500 border-t border-zinc-100",
                    div { "{strings.traditional.bullet_1}" }
                    div { "{strings.traditional.bullet_2}" }
                    div { "{strings.traditional.bullet_3}" }
                }
            }

            // Holt Lab Turnkey Model
            div {
                class: "studio-card p-6 sm:p-8 rounded-xs bg-white border border-zinc-200 border-l-4 border-l-[#E64A19] space-y-4 shadow-2xs hover:border-zinc-300",
                div {
                    class: "flex items-center justify-between font-mono text-xs",
                    span { class: "text-[#E64A19] font-bold uppercase", "{strings.turnkey.badge}" }
                    span { class: "text-emerald-600 font-bold", "{strings.turnkey.metric}" }
                }
                h3 { class: "text-lg font-bold text-zinc-950 lowercase font-sans", "{strings.turnkey.heading}" }
                p {
                    class: "text-xs sm:text-sm text-zinc-600 leading-relaxed font-normal",
                    "{strings.turnkey.body}"
                }
                div {
                    class: "space-y-2 pt-2 text-xs font-mono text-zinc-700 border-t border-zinc-100",
                    div { "{strings.turnkey.bullet_1}" }
                    div { "{strings.turnkey.bullet_2}" }
                    div { "{strings.turnkey.bullet_3}" }
                }
            }
        }
    }
}

/// 4-phase turnkey engineering pipeline breakdown.
#[component]
fn TurnkeyPipelineGrid(strings: StudioStrings) -> Element {
    rsx! {
        div {
            class: "grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-6 pt-4 text-xs font-mono lowercase",
            for phase in strings.pipeline.iter() {
                div {
                    key: "{phase.phase_tag}",
                    class: "studio-card p-5 rounded-xs bg-white border border-zinc-200 hover:border-zinc-300 space-y-2 select-none",
                    span { class: "text-[#E64A19] font-bold block", "{phase.phase_tag}" }
                    h4 { class: "text-sm font-bold text-zinc-950", "{phase.title}" }
                    p { class: "text-zinc-600 font-sans leading-relaxed text-xs", "{phase.desc}" }
                }
            }
        }
    }
}
