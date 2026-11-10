//! ## Overview
//! Monumental exploded-view hero banner illustrating the keyring's internal mechanical layers.
//! Features the full-assembly exploded 3D render with Framework-inspired subsystem callout tags.
//!
//! ## Search Tags
//! #exploded, #teardown, #cad, #render, #framework-style
//!
//! ## Submodules
//! None. This module provides atomic visual banner presentation components.

use dioxus::prelude::*;

const EXPLODED_CALLOUTS: [(&str, &str); 4] = [
    ("TOP", "PA12 Front Bezel & Glass"),
    ("MID-1", "2.0\" IPS & CST816D Touch"),
    ("MID-2", "4-Layer PCBA & ESP32-C3"),
    ("BASE", "LiPo, Motor & Rear Enclosure"),
];

/// Exploded-view hero showcase presenting the complete pulled-apart machine.
#[component]
pub fn ExplodedHeroBanner() -> Element {
    rsx! {
        div {
            class: "w-full rounded-xs border border-zinc-200 bg-gradient-to-b from-zinc-50 to-white p-6 sm:p-10 mb-12 sm:mb-16 shadow-2xs relative overflow-hidden",
            div {
                class: "flex flex-col md:flex-row md:items-end justify-between border-b border-zinc-200/80 pb-6 mb-8 gap-4",
                div {
                    class: "space-y-1.5",
                    span { class: "text-xs font-mono text-[#E64A19] font-semibold tracking-wider uppercase", "ANATOMICAL ARCHITECTURE" }
                    h3 { class: "text-2xl sm:text-3xl font-bold font-sans tracking-tight text-zinc-950 lowercase", "pulled apart to the bare copper." }
                }
                div {
                    class: "flex flex-wrap items-center gap-2",
                    for (tag, label) in EXPLODED_CALLOUTS.iter() {
                        div {
                            key: "{tag}",
                            class: "px-2.5 py-1 rounded-xs bg-white border border-zinc-200 text-zinc-600 font-mono text-[11px]",
                            span { class: "font-bold text-zinc-900 mr-1.5", "{tag}" }
                            span { "{label}" }
                        }
                    }
                }
            }

            // Exploded 3D Render
            div {
                class: "relative flex justify-center items-center py-4 select-none",
                img {
                    src: "assets/showcase/mcad_exploded.png",
                    alt: "Win95 Minesweeper Touch Keyring 3D Exploded View",
                    class: "w-full max-h-[500px] object-contain drop-shadow-xl"
                }
            }

            div {
                class: "border-t border-zinc-200/80 pt-4 flex flex-col sm:flex-row items-center justify-between text-[11px] font-mono text-zinc-400 gap-2",
                span { "PARAMETRIC CAD // OPENCASCADE B-REP" }
                span { "23 PAIRWISE CHECKS // 0.000 mm³ COLLISION" }
                span { "JLC3DP MJF NYLON & JLCPCB PCBA" }
            }
        }
    }
}
