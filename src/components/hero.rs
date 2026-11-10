//! ## Overview
//! Monumental hero demonstrator showcase modeled after Framework's single-hardware architecture.
//! Centers Holt Lab's flagship Win95 Minesweeper Touch Keyring with localized telemetry and quick spec badges.
//!
//! ## Search Tags
//! #hero, #flagship, #demonstrator, #framework-style, #keyring
//!
//! ## Submodules
//! None. This module provides atomic hero presentation components.

use dioxus::prelude::*;
use crate::i18n::{hero::get_hero_strings, HeroStrings, Language};

/// Primary monumental hero section rendered at `#demonstrator`.
#[component]
pub fn HeroDemonstrator() -> Element {
    let lang = use_context::<Signal<Language>>();
    let strings = get_hero_strings(lang());

    rsx! {
        section {
            id: "demonstrator",
            class: "relative bg-gradient-to-b from-white via-zinc-50/50 to-white pt-10 sm:pt-16 pb-20 sm:pb-28 border-b border-zinc-200 overflow-hidden font-sans scroll-mt-20",
            div {
                class: "max-w-[1400px] mx-auto px-4 sm:px-8 flex flex-col items-center text-center",
                HeroHeaderBlock { strings }
                HeroSpecsPills { strings }
                HeroMonumentalVisual { strings }
                HeroActionLinks { strings }
            }
        }
    }
}

/// Header text block with eyebrow, headline, and engineering subtitle.
#[component]
fn HeroHeaderBlock(strings: HeroStrings) -> Element {
    rsx! {
        div {
            class: "max-w-4xl flex flex-col items-center space-y-4",
            div {
                class: "inline-flex items-center gap-2 px-3 py-1 rounded-full bg-zinc-100 border border-zinc-200 text-zinc-700 font-mono text-[11px] font-semibold tracking-wider uppercase",
                span { class: "w-2 h-2 rounded-full bg-[#E64A19]" }
                "{strings.eyebrow}"
            }
            h1 {
                class: "text-3xl sm:text-5xl md:text-6xl font-bold font-sans tracking-tight text-zinc-950 lowercase leading-[1.08]",
                "{strings.title}"
            }
            p {
                class: "text-base sm:text-lg md:text-xl text-zinc-600 font-normal leading-relaxed max-w-3xl pt-2",
                "{strings.subtitle}"
            }
        }
    }
}

/// Pill bar displaying the 5 core physical and electrical achievements.
#[component]
fn HeroSpecsPills(strings: HeroStrings) -> Element {
    rsx! {
        div {
            class: "flex flex-wrap items-center justify-center gap-2 pt-6 pb-8",
            for spec in strings.pills.iter() {
                span {
                    key: "{spec}",
                    class: "px-3 py-1 rounded-xs bg-white border border-zinc-300 text-zinc-800 font-mono text-xs sm:text-[13px] font-medium shadow-2xs",
                    "{spec}"
                }
            }
        }
    }
}

/// Center visual stage presenting the full-scale isolated hardware render with technical boundary indicators.
#[component]
fn HeroMonumentalVisual(strings: HeroStrings) -> Element {
    rsx! {
        div {
            class: "relative w-full max-w-2xl my-4 group select-none",
            div {
                class: "absolute -inset-4 bg-radial from-zinc-200/40 via-transparent to-transparent rounded-full blur-2xl pointer-events-none"
            }
            div {
                class: "relative z-10 flex flex-col items-center",
                img {
                    src: "assets/showcase/mcad_front.png",
                    alt: "Win95 Minesweeper Touch Keyring Monumental Front View",
                    class: "w-full max-h-[520px] object-contain drop-shadow-2xl transition-transform duration-500 hover:scale-[1.015]"
                }
                div {
                    class: "w-full flex items-center justify-between text-[11px] font-mono text-zinc-400 pt-3 border-t border-zinc-200/80 px-2",
                    span { "{strings.width_label}" }
                    span { "{strings.material_label}" }
                    span { "{strings.height_label}" }
                }
            }
        }
    }
}

/// CTA links leading directly to interactive teardown and deep engineering specifications.
#[component]
fn HeroActionLinks(strings: HeroStrings) -> Element {
    rsx! {
        div {
            class: "flex flex-col sm:flex-row items-center gap-4 pt-8",
            a {
                href: "#teardown",
                class: "w-full sm:w-auto inline-flex items-center justify-center gap-2 px-6 py-3 rounded-xs bg-zinc-950 hover:bg-zinc-800 text-white font-mono text-xs font-semibold tracking-wider transition-colors shadow-xs",
                span { "{strings.cta_teardown}" }
                span { "↓" }
            }
            a {
                href: "#specs",
                class: "w-full sm:w-auto inline-flex items-center justify-center gap-2 px-6 py-3 rounded-xs bg-white hover:bg-zinc-50 border border-zinc-300 text-zinc-900 font-mono text-xs font-semibold tracking-wider transition-colors shadow-2xs",
                span { "{strings.cta_specs}" }
                span { "→" }
            }
        }
    }
}
