//! ## Overview
//! Studio engineering protocol and turnkey hardware philosophy section.
//! Highlights the single lead architect model, eliminate handoff friction, and deterministic verification gates.
//!
//! ## Search Tags
//! #philosophy, #protocol, #turnkey, #architecture, #manifesto

use dioxus::prelude::*;

/// Primary studio engineering protocol and philosophy section rendered at `#about`.
#[component]
pub fn Philosophy() -> Element {
    rsx! {
        section {
            id: "about",
            class: "bg-[#FAFAFA] py-16 sm:py-24 border-t border-zinc-200 font-sans",
            div {
                class: "max-w-[1600px] mx-auto px-4 sm:px-8",
                div {
                    class: "grid grid-cols-1 lg:grid-cols-12 gap-12 lg:gap-16",
                    PhilosophyHeader {}
                    PhilosophyBody {}
                }
            }
        }
    }
}

/// Left column header lockup with protocol title.
#[component]
fn PhilosophyHeader() -> Element {
    rsx! {
        div {
            class: "lg:col-span-4 flex flex-col space-y-3",
            span { class: "text-xs font-mono text-zinc-400 lowercase", "engineering philosophy" }
            h2 {
                class: "text-2xl sm:text-3xl font-bold lowercase text-zinc-950 tracking-tight leading-tight",
                "holt lab protocol."
            }
            span {
                class: "text-xs font-mono text-zinc-500 lowercase leading-relaxed",
                "hardware · mechanical · firmware turnkey"
            }
        }
    }
}

/// Right column body containing manifesto paragraph and 4 disciplines grid.
#[component]
fn PhilosophyBody() -> Element {
    rsx! {
        div {
            class: "lg:col-span-8 space-y-10",
            p {
                class: "text-base sm:text-lg font-sans text-zinc-800 leading-relaxed font-normal",
                "we design complete physical computing devices from silicon to screen. no fragmented vendor pass-through. no collision handoffs. every millimeter of chassis geometry, copper trace, and bare-metal clock cycle is engineered by a single lead architect with 0.00 mm³ tolerance."
            }
            DisciplinesGrid {}
        }
    }
}

/// 4 turnkey engineering disciplines breakdown.
#[component]
fn DisciplinesGrid() -> Element {
    rsx! {
        div {
            class: "grid grid-cols-1 md:grid-cols-2 gap-8 border-t border-zinc-200 pt-8 text-xs font-mono lowercase",
            div {
                class: "space-y-1.5",
                span { class: "text-zinc-950 font-bold block", "01 // parametric mcad" }
                p { class: "text-zinc-600 font-sans leading-relaxed text-xs", "build123d & opencascade b-rep solid kernel. cots 3d step models locked before cad freeze to ensure 0.00 mm³ volumetric interference." }
            }
            div {
                class: "space-y-1.5",
                span { class: "text-zinc-950 font-bold block", "02 // multi-layer ecad" }
                p { class: "text-zinc-600 font-sans leading-relaxed text-xs", "kicad 9 controlled impedance routing. 50Ω single-ended & 90Ω differential usb matched. zero-touch automated factory drc." }
            }
            div {
                class: "space-y-1.5",
                span { class: "text-zinc-950 font-bold block", "03 // bare-metal firmware" }
                p { class: "text-zinc-600 font-sans leading-relaxed text-xs", "rust esp-hal & embassy async framework. direct register manipulation, ahb gdma zero-copy pipelining, and low-jitter interrupt sleep." }
            }
            div {
                class: "space-y-1.5",
                span { class: "text-zinc-950 font-bold block", "04 // factory dispatch & bring-up" }
                p { class: "text-zinc-600 font-sans leading-relaxed text-xs", "direct dispatch to jlcpcb pcba & jlc3dp mjf automated production lines. turnkey bring-up on remote tailscale bench with zero physical rework." }
            }
        }
    }
}
