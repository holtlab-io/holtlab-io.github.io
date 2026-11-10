//! ## Overview
//! Interactive CAD workbench console component presenting 4-view engineering inspection modes.
//! Provides ISO projection, front bezel elevation, cross-section anatomy, and wireframe views with real-time telemetry.
//!
//! ## Search Tags
//! #workbench, #cad, #console, #inspection, #telemetry, #demonstrator
//!
//! ## Submodules
//! None. This module provides atomic CAD inspection console components.

use dioxus::prelude::*;
use crate::i18n::{hero::get_hero_strings, HeroStrings, Language};

/// Interactive 4-view CAD inspection console for the hardware demonstrator showcase.
#[component]
pub fn CadWorkbench() -> Element {
    let lang = use_context::<Signal<Language>>();
    let strings = get_hero_strings(lang());
    let mut active_mode_idx = use_signal(|| 1usize);
    let active_mode = strings.console_modes[active_mode_idx()];

    rsx! {
        div {
            class: "w-full max-w-5xl mx-auto bg-white border border-zinc-200 rounded-xs shadow-md p-4 sm:p-6 my-4 select-none relative overflow-hidden reveal-on-scroll delay-100",
            div {
                class: "w-full flex flex-col sm:flex-row sm:items-center justify-between border-b border-zinc-200/80 pb-3 mb-4 gap-2 text-[11px] font-mono text-zinc-500",
                div {
                    class: "flex items-center gap-2 truncate",
                    span { class: "relative flex h-2 w-2 shrink-0",
                        span { class: "animate-ping absolute inline-flex h-full w-full rounded-full bg-emerald-400 opacity-75" }
                        span { class: "relative inline-flex rounded-full h-2 w-2 bg-emerald-500" }
                    }
                    span { class: "font-semibold text-zinc-800 truncate", "{strings.console_status}" }
                }
                div {
                    class: "inline-flex items-center gap-1 bg-zinc-100/90 p-0.5 rounded-xs border border-zinc-200/80 self-start sm:self-auto",
                    for (idx, (mode_key, label, _, _)) in strings.console_modes.iter().enumerate() {
                        button {
                            key: "{mode_key}",
                            r#type: "button",
                            onclick: move |_| active_mode_idx.set(idx),
                            class: if active_mode_idx() == idx {
                                "px-2.5 py-1 rounded-2xs bg-zinc-950 text-white font-bold shadow-2xs text-[10px] tracking-wider transition-all"
                            } else {
                                "px-2.5 py-1 text-zinc-600 hover:text-zinc-950 text-[10px] tracking-wider transition-colors"
                            },
                            "{label}"
                        }
                    }
                }
            }
            WorkbenchViewport { strings, active_mode }
        }
    }
}

/// Viewport displaying active CAD render, crosshairs, telemetry badge, and Swiss dimension callouts.
#[component]
fn WorkbenchViewport(strings: HeroStrings, active_mode: (&'static str, &'static str, &'static str, &'static str)) -> Element {
    let (_, _, telemetry, img_src) = active_mode;

    rsx! {
        div {
            class: "relative z-10 flex flex-col items-center py-2 bg-tech-grid rounded-xs border border-zinc-100 overflow-hidden",
            div { class: "absolute top-2 left-2 text-zinc-300 font-mono text-xs select-none pointer-events-none", "+" }
            div { class: "absolute top-2 right-2 text-zinc-300 font-mono text-xs select-none pointer-events-none", "+" }
            div { class: "absolute bottom-12 left-2 text-zinc-300 font-mono text-xs select-none pointer-events-none", "+" }
            div { class: "absolute bottom-12 right-2 text-zinc-300 font-mono text-xs select-none pointer-events-none", "+" }

            div {
                class: "w-full px-4 pt-1 flex items-center justify-between text-[10px] font-mono text-zinc-400 select-none",
                span {
                    class: "px-2 py-0.5 rounded-xs bg-zinc-900/90 text-white font-medium tracking-wider backdrop-blur-xs",
                    "{telemetry}"
                }
                span {
                    class: "hidden sm:inline-block text-[9px] text-zinc-400 uppercase tracking-widest",
                    "{strings.console_sub}"
                }
            }

            div {
                key: "{img_src}",
                class: "w-full flex justify-center items-center py-4 min-h-[320px] sm:min-h-[460px] animate-fadeIn",
                img {
                    src: "{img_src}",
                    alt: "Holt Lab Engineering CAD Inspection Model",
                    class: "w-full max-h-[460px] object-contain drop-shadow-xl transition-transform duration-500 hover:scale-[1.015]"
                }
            }

            div {
                class: "w-full flex items-center justify-between text-[11px] font-mono text-zinc-400 pt-3 border-t border-zinc-200/80 px-4 gap-2 bg-white/80 backdrop-blur-xs",
                span { class: "truncate", "{strings.width_label}" }
                span { class: "truncate font-medium text-zinc-600", "{strings.material_label}" }
                span { class: "truncate text-right", "{strings.height_label}" }
            }
        }
    }
}
