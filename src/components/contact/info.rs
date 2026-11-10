//! ## Overview
//! Contact info and value assurance column presenting Holt Lab direct communication channels and SLAs.
//! Employs a quiet Swiss layout with lowercase labels and clear institutional coordinates.
//!
//! ## Search Tags
//! #contact, #info, #sla, #guarantee, #email

use dioxus::prelude::*;

/// Left column displaying direct contact information and institutional coordinates.
#[component]
pub fn ContactInfo() -> Element {
    rsx! {
        div {
            class: "lg:col-span-5 space-y-6 font-sans",
            div {
                class: "space-y-3",
                span { class: "text-xs font-mono text-zinc-400 lowercase", "direct inquiry" }
                h2 {
                    class: "text-2xl sm:text-3xl font-bold lowercase text-zinc-950 tracking-tight leading-tight",
                    "studio inquiry."
                }
                p {
                    class: "text-zinc-600 text-xs sm:text-sm font-sans leading-relaxed pt-1",
                    "we accept selected turnkey hardware commissions, multi-layer pcba layout, and bare-metal firmware development. every inquiry is evaluated directly by our lead hardware architect within 24 hours."
                }
            }

            // Direct Coordinates Cards
            div {
                class: "space-y-3 text-xs font-mono lowercase pt-2",
                div {
                    class: "p-4 rounded-xs border border-zinc-200 bg-white flex items-center justify-between shadow-2xs",
                    div {
                        div { class: "text-zinc-400 text-[10px] uppercase font-mono", "direct contact // email" }
                        a {
                            href: "mailto:contact@holtlab.io",
                            class: "text-sm font-bold text-zinc-950 hover:underline transition-colors mt-0.5 block",
                            "contact@holtlab.io"
                        }
                    }
                    span { class: "text-zinc-400 text-xs font-mono", "→" }
                }

                div {
                    class: "p-4 rounded-xs border border-zinc-200 bg-white flex items-center justify-between shadow-2xs",
                    div {
                        div { class: "text-zinc-400 text-[10px] uppercase font-mono", "studio location" }
                        div { class: "text-xs font-semibold text-zinc-800 mt-0.5", "seoul, korea // remote worldwide" }
                    }
                    span { class: "text-zinc-900 text-xs font-bold font-mono", "ACTIVE" }
                }

                div {
                    class: "p-4 rounded-xs border border-zinc-200 bg-white flex items-center justify-between shadow-2xs",
                    div {
                        div { class: "text-zinc-400 text-[10px] uppercase font-mono", "response commitment" }
                        div { class: "text-xs font-semibold text-zinc-800 mt-0.5", "24h initial architectural review" }
                    }
                    span { class: "text-zinc-600 text-xs font-mono font-medium", "24H" }
                }
            }
        }
    }
}
