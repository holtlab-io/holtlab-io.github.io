//! ## Overview
//! Monumental exploded-view hero banner illustrating the keyring's internal mechanical layers.
//! Features the full-assembly exploded 3D render with localized subsystem callout tags.
//!
//! ## Search Tags
//! #exploded, #teardown, #cad, #render, #framework-style
//!
//! ## Submodules
//! None. This module provides atomic visual banner presentation components.

use dioxus::prelude::*;
use crate::i18n::{teardown::get_teardown_section_strings, Language};

/// Exploded-view hero showcase presenting the complete pulled-apart machine.
#[component]
pub fn ExplodedHeroBanner() -> Element {
    let lang = use_context::<Signal<Language>>();
    let strings = get_teardown_section_strings(lang());

    rsx! {
        div {
            class: "w-full rounded-xs border border-zinc-200 bg-gradient-to-b from-zinc-50 to-white p-6 sm:p-10 mb-12 sm:mb-16 shadow-2xs relative overflow-hidden",
            div {
                class: "flex flex-col md:flex-row md:items-end justify-between border-b border-zinc-200/80 pb-6 mb-8 gap-4",
                div {
                    class: "space-y-1.5",
                    span { class: "text-xs font-mono text-[#E64A19] font-semibold tracking-wider uppercase", "{strings.exploded_eyebrow}" }
                    h3 { class: "text-2xl sm:text-3xl font-bold font-sans tracking-tight text-zinc-950 lowercase", "{strings.exploded_title}" }
                }
                div {
                    class: "flex flex-wrap items-center gap-2",
                    for (tag, label) in strings.callouts.iter() {
                        div {
                            key: "{tag}",
                            class: "px-2.5 py-1 rounded-xs bg-white border border-zinc-200 text-zinc-600 font-mono text-[11px]",
                            span { class: "font-bold text-zinc-900 mr-1.5", "{tag}" }
                            span { "{label}" }
                        }
                    }
                }
            }

            div {
                class: "relative flex justify-center items-center py-6 select-none bg-tech-grid rounded-xs border border-zinc-100 my-2",
                div { class: "absolute top-2 left-2 text-zinc-300 font-mono text-xs select-none pointer-events-none", "+" }
                div { class: "absolute top-2 right-2 text-zinc-300 font-mono text-xs select-none pointer-events-none", "+" }
                div { class: "absolute bottom-2 left-2 text-zinc-300 font-mono text-xs select-none pointer-events-none", "+" }
                div { class: "absolute bottom-2 right-2 text-zinc-300 font-mono text-xs select-none pointer-events-none", "+" }
                img {
                    src: "assets/showcase/mcad_exploded.png",
                    alt: "Win95 Minesweeper Touch Keyring 3D Exploded View",
                    class: "w-full max-h-[500px] object-contain drop-shadow-xl transition-transform duration-700 hover:scale-[1.01]"
                }
            }

            div {
                class: "border-t border-zinc-200/80 pt-4 flex flex-col sm:flex-row items-center justify-between text-[11px] font-mono text-zinc-400 gap-2",
                span { "{strings.bottom_strip[0]}" }
                span { "{strings.bottom_strip[1]}" }
                span { "{strings.bottom_strip[2]}" }
            }
        }
    }
}
