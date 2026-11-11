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
use crate::routes::Route;

/// Primary studio hero section rendered at `#hero`.
#[component]
pub fn HeroDemonstrator() -> Element {
    let lang = use_context::<Signal<Language>>();
    let strings = get_hero_strings(lang());

    rsx! {
        section {
            id: "hero",
            class: "relative bg-[#FAFAFA] pt-12 sm:pt-20 pb-16 sm:pb-24 border-b border-zinc-200 overflow-hidden font-sans scroll-mt-20",
            div {
                class: "absolute inset-0 bg-tech-grid opacity-60 pointer-events-none"
            }
            div {
                class: "absolute inset-0 bg-[radial-gradient(ellipse_80%_60%_at_50%_-10%,rgba(230,74,25,0.08),rgba(255,255,255,0))] pointer-events-none"
            }
            div {
                class: "max-w-[1400px] mx-auto px-4 sm:px-8 flex flex-col items-center text-center relative z-10",
                HeroHeaderBlock { strings }
                HeroSpecsPills { strings }
                HeroActionLinks { strings }
                HeroOverviewVisual { strings }
            }
        }
    }
}

/// Header text block with studio eyebrow, headline, and engineering subtitle.
#[component]
fn HeroHeaderBlock(strings: HeroStrings) -> Element {
    rsx! {
        div {
            class: "max-w-4xl flex flex-col items-center space-y-4 reveal-on-scroll",
            div {
                class: "inline-flex items-center gap-2 px-3 py-1 rounded-full bg-white border border-zinc-200 text-zinc-700 font-mono text-[11px] font-semibold tracking-wider uppercase shadow-2xs",
                span { class: "w-2 h-2 rounded-full bg-[#E64A19] animate-pulse" }
                "{strings.eyebrow}"
            }
            h1 {
                class: "text-2xl sm:text-5xl md:text-6xl font-bold font-sans tracking-tight text-zinc-950 leading-snug sm:leading-[1.15] break-keep",
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
            class: "flex flex-wrap items-center justify-center gap-2 pt-6 pb-2 reveal-on-scroll delay-100",
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
            class: "flex flex-col sm:flex-row items-center gap-4 pt-6 pb-8 reveal-on-scroll delay-200",
            Link {
                to: Route::Demonstrator {},
                class: "tactile-button w-full sm:w-auto inline-flex items-center justify-center gap-2 px-6 py-3.5 rounded-xs bg-zinc-950 hover:bg-zinc-800 text-white font-mono text-xs font-semibold tracking-wider transition-colors shadow-xs",
                span { "{strings.cta_casestudy}" }
                span { "→" }
            }
            Link {
                to: Route::ContactPage {},
                class: "tactile-button w-full sm:w-auto inline-flex items-center justify-center gap-2 px-6 py-3.5 rounded-xs bg-white hover:bg-zinc-50 border border-zinc-300 text-zinc-900 font-mono text-xs font-semibold tracking-wider transition-colors shadow-2xs",
                span { "{strings.cta_inquire}" }
                span { "→" }
            }
        }
    }
}

/// Studio hardware visual preview pairing 3D CAD design with fabricated SMT PCBA and MJF hardware.
#[component]
fn HeroOverviewVisual(strings: HeroStrings) -> Element {
    let lang = use_context::<Signal<Language>>();

    rsx! {
        div {
            class: "w-full max-w-5xl bg-white border border-zinc-200 rounded-xs shadow-md p-4 sm:p-6 my-2 select-none relative overflow-hidden reveal-on-scroll delay-300",
            div {
                class: "w-full flex items-center justify-between border-b border-zinc-200/80 pb-3 mb-4 text-[11px] font-mono text-zinc-500",
                div {
                    class: "flex items-center gap-2 truncate",
                    span { class: "relative flex h-2 w-2 shrink-0",
                        span { class: "animate-ping absolute inline-flex h-full w-full rounded-full bg-emerald-400 opacity-75" }
                        span { class: "relative inline-flex rounded-full h-2 w-2 bg-emerald-500" }
                    }
                    span { class: "font-semibold text-zinc-800 truncate", "{strings.hero_preview_status}" }
                }
                span {
                    class: "hidden sm:inline-block text-[10px] text-zinc-400 font-mono tracking-wider shrink-0",
                    "{strings.hero_preview_tag}"
                }
            }
            HeroSplitPanels { lang: lang() }
            div {
                class: "w-full flex flex-col sm:flex-row items-center justify-between text-[10px] sm:text-[11px] font-mono text-zinc-500 pt-3 mt-4 border-t border-zinc-200/80 gap-1.5 sm:gap-2 text-center sm:text-left",
                span { class: "break-keep", "{strings.hero_preview_notes[0]}" }
                span { class: "break-keep font-medium text-zinc-700", "{strings.hero_preview_notes[1]}" }
                span { class: "break-keep text-right", "{strings.hero_preview_notes[2]}" }
            }
        }
    }
}

/// Side-by-side 3D CAD assembly and physical 4-layer SMT PCBA / MJF PA12 enclosure panels.
#[component]
fn HeroSplitPanels(lang: Language) -> Element {
    let (cad_label, real_label) = match lang {
        Language::En => (
            "01 // 3D CAD & ENCLOSURE GEOMETRY",
            "02 // FABRICATED 4-LAYER PCBA & MJF PA12",
        ),
        Language::Ko => (
            "01 // 3D 기구 · 적층 구조 설계",
            "02 // 4층 SMT 기판 및 나일론 외함 실물",
        ),
    };

    rsx! {
        div {
            class: "grid grid-cols-1 md:grid-cols-2 gap-4",
            div {
                class: "relative flex flex-col justify-between bg-tech-grid rounded-xs border border-zinc-200/80 p-4 min-h-[250px] sm:min-h-[310px] overflow-hidden",
                span {
                    class: "self-start px-2 py-0.5 rounded-2xs bg-white/90 border border-zinc-200 font-mono text-[10px] font-semibold text-zinc-700",
                    "{cad_label}"
                }
                div {
                    class: "flex-1 flex items-center justify-center py-2",
                    img {
                        src: "/assets/showcase/mcad_iso_front.png",
                        alt: "Holt Lab 3D CAD Enclosure Design",
                        class: "w-full max-h-[240px] sm:max-h-[260px] object-contain drop-shadow-lg transition-transform duration-500 hover:scale-[1.02]",
                    }
                }
            }
            div {
                class: "relative flex flex-col justify-between bg-zinc-900 rounded-xs border border-zinc-800 overflow-hidden min-h-[250px] sm:min-h-[310px] group",
                img {
                    src: "/assets/showcase/real_pcba_shell_iso.jpg",
                    alt: "Holt Lab Fabricated 4-Layer PCBA and MJF PA12 Enclosure",
                    class: "w-full h-full max-h-[310px] object-cover transition-transform duration-500 group-hover:scale-[1.02]",
                }
                div {
                    class: "absolute top-4 left-4",
                    span {
                        class: "px-2 py-0.5 rounded-2xs bg-zinc-950/85 border border-zinc-700 backdrop-blur-xs font-mono text-[10px] font-semibold text-zinc-100",
                        "{real_label}"
                    }
                }
            }
        }
    }
}
