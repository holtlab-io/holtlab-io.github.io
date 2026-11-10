//! ## Overview
//! Studio hero section presenting Holt Lab's hardware engineering studio platform.
//! Positions Holt Lab as a unified turnkey R&D lab with interactive studio console visual.
//!
//! ## Search Tags
//! #hero, #studio, #platform, #turnkey, #hardware
//!
//! ## Submodules
//! None. This module provides atomic hero presentation components.

use dioxus::prelude::*;
use crate::i18n::{hero::get_hero_strings, HeroStrings, Language};

/// Primary studio hero section rendered at `#hero`.
#[component]
pub fn HeroDemonstrator() -> Element {
    let lang = use_context::<Signal<Language>>();
    let strings = get_hero_strings(lang());

    rsx! {
        section {
            id: "hero",
            class: "relative bg-[#FAFAFA] pt-12 sm:pt-20 pb-20 sm:pb-28 border-b border-zinc-200 overflow-hidden font-sans scroll-mt-20",
            div {
                class: "absolute inset-0 bg-[radial-gradient(ellipse_80%_60%_at_50%_-10%,rgba(230,74,25,0.07),rgba(255,255,255,0))] pointer-events-none"
            }
            div {
                class: "max-w-[1400px] mx-auto px-4 sm:px-8 flex flex-col items-center text-center relative z-10",
                HeroHeaderBlock { strings }
                HeroSpecsPills { strings }
                HeroActionLinks { strings }
                HeroStudioConsole { strings }
            }
        }
    }
}

/// Header text block with studio eyebrow, headline, and engineering subtitle.
#[component]
fn HeroHeaderBlock(strings: HeroStrings) -> Element {
    rsx! {
        div {
            class: "max-w-4xl flex flex-col items-center space-y-4",
            div {
                class: "inline-flex items-center gap-2 px-3 py-1 rounded-full bg-white border border-zinc-200 text-zinc-700 font-mono text-[11px] font-semibold tracking-wider uppercase shadow-2xs",
                span { class: "w-2 h-2 rounded-full bg-[#E64A19] animate-pulse" }
                "{strings.eyebrow}"
            }
            h1 {
                class: "text-3xl sm:text-5xl md:text-6xl font-bold font-sans tracking-tight text-zinc-950 leading-[1.12]",
                "{strings.title}"
            }
            p {
                class: "text-base sm:text-lg md:text-xl text-zinc-600 font-normal leading-relaxed max-w-3xl pt-2",
                "{strings.subtitle}"
            }
        }
    }
}

/// Pill bar displaying the 4 core studio engineering disciplines.
#[component]
fn HeroSpecsPills(strings: HeroStrings) -> Element {
    rsx! {
        div {
            class: "flex flex-wrap items-center justify-center gap-2 pt-6 pb-2",
            for spec in strings.pills.iter() {
                span {
                    key: "{spec}",
                    class: "px-3 py-1.5 rounded-xs bg-white border border-zinc-200/90 text-zinc-800 font-mono text-xs sm:text-[13px] font-medium shadow-2xs hover:border-zinc-400 transition-colors",
                    "{spec}"
                }
            }
        }
    }
}

/// CTA links leading directly to interactive case study and project inquiry.
#[component]
fn HeroActionLinks(strings: HeroStrings) -> Element {
    rsx! {
        div {
            class: "flex flex-col sm:flex-row items-center gap-4 pt-6 pb-10",
            a {
                href: "#case-study",
                class: "tactile-button w-full sm:w-auto inline-flex items-center justify-center gap-2 px-6 py-3.5 rounded-xs bg-zinc-950 hover:bg-zinc-800 text-white font-mono text-xs font-semibold tracking-wider transition-colors shadow-xs",
                span { "{strings.cta_casestudy}" }
                span { "↓" }
            }
            a {
                href: "#inquire",
                class: "tactile-button w-full sm:w-auto inline-flex items-center justify-center gap-2 px-6 py-3.5 rounded-xs bg-white hover:bg-zinc-50 border border-zinc-300 text-zinc-900 font-mono text-xs font-semibold tracking-wider transition-colors shadow-2xs",
                span { "{strings.cta_inquire}" }
                span { "→" }
            }
        }
    }
}

/// Studio workbench console presenting the flagship demonstrator with telemetry.
#[component]
fn HeroStudioConsole(strings: HeroStrings) -> Element {
    rsx! {
        div {
            class: "w-full max-w-4xl bg-white border border-zinc-200 rounded-xs shadow-md p-4 sm:p-6 my-2 group select-none relative overflow-hidden",
            div {
                class: "w-full flex items-center justify-between border-b border-zinc-200/80 pb-3 mb-4 text-[11px] font-mono text-zinc-500",
                div {
                    class: "flex items-center gap-2 truncate",
                    span { class: "relative flex h-2 w-2",
                        span { class: "animate-ping absolute inline-flex h-full w-full rounded-full bg-emerald-400 opacity-75" }
                        span { class: "relative inline-flex rounded-full h-2 w-2 bg-emerald-500" }
                    }
                    span { class: "font-semibold text-zinc-800 truncate", "{strings.console_status}" }
                }
                span {
                    class: "hidden sm:inline-block text-[10px] text-zinc-400 uppercase tracking-wider shrink-0",
                    "{strings.console_sub}"
                }
            }
            div {
                class: "relative z-10 flex flex-col items-center py-2",
                img {
                    src: "assets/showcase/mcad_front.png",
                    alt: "Holt Lab Flagship Demonstrator Front Hardware",
                    class: "w-full max-h-[480px] object-contain drop-shadow-xl transition-transform duration-500 hover:scale-[1.015]"
                }
                div {
                    class: "w-full flex items-center justify-between text-[11px] font-mono text-zinc-400 pt-4 border-t border-zinc-100 px-2 gap-2",
                    span { class: "truncate", "{strings.width_label}" }
                    span { class: "truncate", "{strings.material_label}" }
                    span { class: "truncate", "{strings.height_label}" }
                }
            }
        }
    }
}
