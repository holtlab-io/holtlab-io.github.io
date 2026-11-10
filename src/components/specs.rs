//! ## Overview
//! High-density Swiss engineering specification matrix table.
//! Presents exacting tolerances, dimensions, materials, silicon specs, and firmware telemetry without marketing fluff.
//!
//! ## Search Tags
//! #specs, #matrix, #tolerances, #stackup, #swiss-engineering
//!
//! ## Submodules
//! None. This module provides atomic specification matrix layout components.

use dioxus::prelude::*;
use crate::i18n::{specs::{get_spec_rows, get_specs_section_strings}, Language};

/// Primary engineering specification table component rendered at `#specs`.
#[component]
pub fn SpecsMatrix() -> Element {
    let lang = use_context::<Signal<Language>>();
    let section_strings = get_specs_section_strings(lang());
    let spec_rows = get_spec_rows(lang());

    rsx! {
        section {
            id: "specs",
            class: "bg-white py-16 sm:py-24 border-t border-zinc-200 font-sans scroll-mt-20",
            div {
                class: "max-w-[1600px] mx-auto px-4 sm:px-8",

                // Header
                div {
                    class: "flex flex-col md:flex-row md:items-end justify-between border-b border-zinc-200 pb-6 mb-8 gap-4",
                    div {
                        class: "flex flex-col",
                        span { class: "text-xs font-mono text-zinc-400 lowercase", "{section_strings.eyebrow}" }
                        h2 { class: "text-2xl sm:text-3xl font-bold lowercase text-zinc-950 tracking-tight mt-1", "{section_strings.title}" }
                    }
                    div {
                        class: "text-xs font-mono text-zinc-500 lowercase",
                        "{section_strings.subtitle}"
                    }
                }

                // Swiss 2-Column Key-Value Grid
                div {
                    class: "border border-zinc-200 divide-y divide-zinc-200 text-xs sm:text-sm font-mono lowercase",
                    for row in spec_rows.iter() {
                        div {
                            key: "{row.key}",
                            class: "grid grid-cols-1 md:grid-cols-3 lg:grid-cols-4 p-4 sm:p-5 hover:bg-zinc-50/50 transition-colors gap-2",
                            div {
                                class: "text-zinc-500 font-medium",
                                "{row.key}"
                            }
                            div {
                                class: "md:col-span-2 lg:col-span-2 text-zinc-950 font-bold",
                                "{row.val}"
                            }
                            div {
                                class: "text-zinc-400 font-sans text-xs pt-0.5",
                                "{row.note}"
                            }
                        }
                    }
                }
            }
        }
    }
}
