//! ## Overview
//! Studio engineering capabilities section presenting the 4 core disciplines of unified turnkey hardware.
//! Highlights mechanical CAD, electrical PCBA, embedded firmware, and rapid prototyping capabilities.
//!
//! ## Search Tags
//! #capabilities, #disciplines, #mcad, #ecad, #firmware, #dfm, #studio
//!
//! ## Submodules
//! None. This module provides atomic studio capability presentation components.

use dioxus::prelude::*;
use crate::i18n::{capabilities::get_capabilities_strings, CapabilitiesStrings, CapabilityCard, Language};

/// Primary studio engineering capabilities section rendered at `#capabilities`.
#[component]
pub fn StudioCapabilities() -> Element {
    let lang = use_context::<Signal<Language>>();
    let strings = get_capabilities_strings(lang());

    rsx! {
        section {
            id: "capabilities",
            class: "bg-white py-16 sm:py-24 border-t border-zinc-200 font-sans scroll-mt-20 relative overflow-hidden",
            div {
                class: "absolute inset-0 bg-tech-dots opacity-35 pointer-events-none"
            }
            div {
                class: "max-w-[1500px] mx-auto px-4 sm:px-8 relative z-10",
                CapabilitiesHeader { strings }
                CapabilitiesGrid { cards: strings.cards }
            }
        }
    }
}

/// Header portion of the capabilities section with eyebrow, headline, and subtitle.
#[component]
fn CapabilitiesHeader(strings: CapabilitiesStrings) -> Element {
    rsx! {
        div {
            class: "flex flex-col md:flex-row md:items-end justify-between border-b border-zinc-200 pb-6 mb-12 gap-4 reveal-on-scroll",
            div {
                class: "space-y-1.5",
                span { class: "text-xs font-mono text-zinc-400 lowercase", "{strings.eyebrow}" }
                h2 { class: "text-2xl sm:text-4xl font-bold font-sans tracking-tight text-zinc-950 lowercase", "{strings.title}" }
            }
            div {
                class: "text-xs font-mono text-zinc-500 max-w-md lowercase",
                "{strings.subtitle}"
            }
        }
    }
}

/// 4-card responsive grid displaying the core engineering disciplines.
#[component]
fn CapabilitiesGrid(cards: [CapabilityCard; 4]) -> Element {
    rsx! {
        div {
            class: "grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-6",
            for card in cards.iter() {
                CapabilityCardItem { key: "{card.number}", card: *card }
            }
        }
    }
}

/// Single interactive capability card with hover lift, status glow, and telemetry.
#[component]
fn CapabilityCardItem(card: CapabilityCard) -> Element {
    rsx! {
        div {
            class: "studio-card group p-6 sm:p-7 rounded-xs bg-[#FAFAFA] hover:bg-white border border-zinc-200/90 hover:border-zinc-400 flex flex-col justify-between space-y-6 shadow-2xs hover:shadow-md select-none relative overflow-hidden border-t-2 border-t-transparent hover:border-t-[#E64A19] transition-all reveal-on-scroll delay-100",
            div { class: "absolute top-2 right-2 text-zinc-300 font-mono text-[10px] opacity-0 group-hover:opacity-100 transition-opacity select-none", "+" }
            div {
                class: "space-y-4",
                div {
                    class: "flex items-center justify-between font-mono text-xs",
                    span { class: "text-lg font-bold text-zinc-950 group-hover:text-[#E64A19] transition-colors", "{card.number}" }
                    span { class: "text-[10px] text-zinc-400 tracking-wider truncate max-w-[180px]", "{card.tag}" }
                }
                h3 {
                    class: "text-base sm:text-lg font-bold text-zinc-950 font-sans tracking-tight lowercase leading-snug group-hover:text-zinc-800 transition-colors",
                    "{card.title}"
                }
                p {
                    class: "text-xs sm:text-[13px] text-zinc-600 leading-relaxed font-normal",
                    "{card.desc}"
                }
            }
            div {
                class: "border-t border-zinc-200/60 pt-4 flex items-center justify-between text-[11px] font-mono text-zinc-500",
                div {
                    class: "flex items-center gap-2 truncate",
                    span { class: "w-1.5 h-1.5 rounded-full bg-emerald-500 shrink-0 group-hover:scale-125 transition-transform" }
                    span { class: "truncate", "{card.metric}" }
                }
                span { class: "text-[9px] font-bold text-emerald-700 bg-emerald-50 border border-emerald-200/80 px-1 py-0.5 rounded-2xs shrink-0", "PASS" }
            }
        }
    }
}
