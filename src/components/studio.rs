//! ## Overview
//! Studio turnkey value proposition section explaining the 1-person unified hardware architecture.
//! Highlights the contrast between fragmented multi-agency ping-pong and Holt Lab's closed-loop co-design execution.
//!
//! ## Search Tags
//! #studio, #turnkey, #methodology, #co-design, #architecture
//!
//! ## Submodules
//! None. This module provides atomic studio methodology components.

use dioxus::prelude::*;

/// Primary studio turnkey value proposition section rendered at `#studio`.
#[component]
pub fn StudioTurnkey() -> Element {
    rsx! {
        section {
            id: "studio",
            class: "bg-[#FAFAFA] py-16 sm:py-24 border-t border-zinc-200 font-sans scroll-mt-20",
            div {
                class: "max-w-[1500px] mx-auto px-4 sm:px-8",
                div {
                    class: "flex flex-col md:flex-row md:items-end justify-between border-b border-zinc-200 pb-6 mb-12 gap-4",
                    div {
                        class: "space-y-1.5",
                        span { class: "text-xs font-mono text-zinc-400 lowercase", "studio turnkey model" }
                        h2 { class: "text-2xl sm:text-4xl font-bold font-sans tracking-tight text-zinc-950 lowercase", "why 1-person turnkey?" }
                    }
                    div {
                        class: "text-xs font-mono text-zinc-500 lowercase",
                        "eliminating agency ping-pong // silicon to screen"
                    }
                }

                StudioComparisonGrid {}
                TurnkeyPipelineGrid {}
            }
        }
    }
}

/// Comparison grid contrasting traditional agency ping-pong with Holt Lab's unified loop.
#[component]
fn StudioComparisonGrid() -> Element {
    rsx! {
        div {
            class: "grid grid-cols-1 lg:grid-cols-2 gap-6 mb-16",

            // Fragmented Multi-Agency Model
            div {
                class: "p-6 sm:p-8 rounded-xs bg-white border border-zinc-200 border-l-4 border-l-red-400 space-y-4 shadow-2xs",
                div {
                    class: "flex items-center justify-between font-mono text-xs",
                    span { class: "text-red-600 font-bold uppercase", "TRADITIONAL AGENCY FRAGMENTATION" }
                    span { class: "text-zinc-400", "6–12 MONTHS SLIPPAGE" }
                }
                h3 { class: "text-lg font-bold text-zinc-950 lowercase font-sans", "the multi-vendor ping-pong trap" }
                p {
                    class: "text-xs sm:text-sm text-zinc-600 leading-relaxed font-normal",
                    "mechanical teams design shells in a vacuum. electrical agencies route boards without COTS 3D models. firmware contractors receive physical prototypes with swapped pins, strapping bus lockups, and unrouted RC delays. weeks turn into months of finger-pointing and costly PCB respins."
                }
                div {
                    class: "space-y-2 pt-2 text-xs font-mono text-zinc-500 border-t border-zinc-100",
                    div { "✕ MCAD / ECAD clearance mismatches discovered after tooling" }
                    div { "✕ Firmware blocked by unexpected internal module resistance" }
                    div { "✕ 3+ PCB revisions required before initial firmware bring-up" }
                }
            }

            // Holt Lab Turnkey Model
            div {
                class: "p-6 sm:p-8 rounded-xs bg-white border border-zinc-200 border-l-4 border-l-[#E64A19] space-y-4 shadow-2xs",
                div {
                    class: "flex items-center justify-between font-mono text-xs",
                    span { class: "text-[#E64A19] font-bold uppercase", "HOLT LAB 1-PERSON TURNKEY" }
                    span { class: "text-emerald-600 font-bold", "0.00 mm³ REWORK-FREE" }
                }
                h3 { class: "text-lg font-bold text-zinc-950 lowercase font-sans", "one closed-loop lead architect" }
                p {
                    class: "text-xs sm:text-sm text-zinc-600 leading-relaxed font-normal",
                    "every millimeter of chassis geometry, copper impedance trace, and bare-metal clock cycle is engineered by a single architect. COTS 3D STEP models are locked before trace freeze. 0.000 mm³ collision and 0 ERC/DRC verification run deterministically before manufacturing orders."
                }
                div {
                    class: "space-y-2 pt-2 text-xs font-mono text-zinc-700 border-t border-zinc-100",
                    div { "✓ Zero-clearance interference: OpenCascade B-Rep Gate 1 check" }
                    div { "✓ Firmware-aware ECAD: GPIO strapping & RC delays solved in design" }
                    div { "✓ First-spin success: working hardware boots over remote Tailscale bench" }
                }
            }
        }
    }
}

/// 4-phase turnkey engineering pipeline breakdown.
#[component]
fn TurnkeyPipelineGrid() -> Element {
    rsx! {
        div {
            class: "grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-6 pt-4 text-xs font-mono lowercase",
            div {
                class: "p-5 rounded-xs bg-white border border-zinc-200 space-y-2",
                span { class: "text-[#E64A19] font-bold block", "01 // CO-DESIGN LOCK" }
                h4 { class: "text-sm font-bold text-zinc-950", "datasheets & 3d contract" }
                p { class: "text-zinc-600 font-sans leading-relaxed text-xs", "pinouts, power envelopes, and vendor COTS 3D STEP models frozen before drafting lines." }
            }
            div {
                class: "p-5 rounded-xs bg-white border border-zinc-200 space-y-2",
                span { class: "text-[#E64A19] font-bold block", "02 // CAD & 4-LAYER ECAD" }
                h4 { class: "text-sm font-bold text-zinc-950", "0.00 mm³ solid modeling" }
                p { class: "text-zinc-600 font-sans leading-relaxed text-xs", "build123d B-Rep solids paired with KiCad 9 controlled impedance routing and full DRC." }
            }
            div {
                class: "p-5 rounded-xs bg-white border border-zinc-200 space-y-2",
                span { class: "text-[#E64A19] font-bold block", "03 // FACTORY DISPATCH" }
                h4 { class: "text-sm font-bold text-zinc-950", "direct line fabrication" }
                p { class: "text-zinc-600 font-sans leading-relaxed text-xs", "automated JLCPCB PCBA SMT & JLC3DP MJF PA12 dispatch with live DOM tracking." }
            }
            div {
                class: "p-5 rounded-xs bg-white border border-zinc-200 space-y-2",
                span { class: "text-[#E64A19] font-bold block", "04 // BARE-METAL BRING-UP" }
                h4 { class: "text-sm font-bold text-zinc-950", "zero-rework firmware" }
                p { class: "text-zinc-600 font-sans leading-relaxed text-xs", "rust esp-hal + embassy runtime, DMA graphics, and Tailscale bench flashing." }
            }
        }
    }
}
