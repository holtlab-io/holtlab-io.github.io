//! ## Overview
//! Interactive anatomical engineering teardown section modeled after Framework's core UX.
//! Allows visitors to inspect the hardware across 4 architectural layers from enclosure to bare-metal Rust.
//!
//! ## Search Tags
//! #teardown, #subsystems, #layers, #framework-style, #hardware
//!
//! ## Submodules
//! - [`data`] (`data.rs`): Subsystem teardown data definitions and engineering layer records.
//! - [`exploded_hero`] (`exploded_hero.rs`): Exploded-view full assembly hero visual banner.
//! - [`layer_card`] (`layer_card.rs`): Subsystem layer inspector with visual assets and verified specifications.

use dioxus::prelude::*;

pub mod data;
pub mod exploded_hero;
pub mod layer_card;

pub use data::{TeardownLayer, TEARDOWN_LAYERS};
pub use exploded_hero::ExplodedHeroBanner;
pub use layer_card::LayerInspector;

/// Primary interactive anatomical engineering teardown section rendered at `#teardown`.
#[component]
pub fn TeardownSection() -> Element {
    let mut active_layer_id = use_signal(|| 0usize);

    let active_layer = TEARDOWN_LAYERS
        .iter()
        .find(|l| l.id == active_layer_id())
        .copied()
        .unwrap_or(TEARDOWN_LAYERS[0]);

    rsx! {
        section {
            id: "teardown",
            class: "bg-white py-16 sm:py-24 border-t border-zinc-200 font-sans",
            div {
                class: "max-w-[1500px] mx-auto px-4 sm:px-8",

                // Section Header
                div {
                    class: "flex flex-col md:flex-row md:items-end justify-between border-b border-zinc-200 pb-6 mb-8 gap-4",
                    div {
                        class: "space-y-1.5",
                        span { class: "text-xs font-mono text-zinc-400 lowercase", "engineering audit" }
                        h2 { class: "text-2xl sm:text-4xl font-bold font-sans tracking-tight text-zinc-950 lowercase", "anatomical engineering teardown" }
                    }
                    div {
                        class: "text-xs font-mono text-zinc-500 lowercase",
                        "radical transparency // 4 discrete subsystems"
                    }
                }

                // Exploded-View Visual Showcase Banner
                ExplodedHeroBanner {}

                // Interactive 4-Layer Tabs Selector
                div {
                    class: "grid grid-cols-2 md:grid-cols-4 gap-2 sm:gap-3 mb-6 select-none",
                    for layer in TEARDOWN_LAYERS.iter() {
                        button {
                            key: "{layer.id}",
                            r#type: "button",
                            onclick: move |_| active_layer_id.set(layer.id),
                            class: if active_layer_id() == layer.id {
                                "p-3 sm:p-4 rounded-xs bg-zinc-950 text-white font-mono text-left transition-all border border-zinc-950 shadow-xs"
                            } else {
                                "p-3 sm:p-4 rounded-xs bg-zinc-50 text-zinc-700 hover:bg-zinc-100 font-mono text-left transition-all border border-zinc-200"
                            },
                            div {
                                class: "flex items-center justify-between text-[10px] opacity-70 mb-1",
                                span { "LAYER {layer.number}" }
                                if active_layer_id() == layer.id {
                                    span { class: "text-[#E64A19] font-bold", "● ACTIVE" }
                                }
                            }
                            div {
                                class: "text-xs sm:text-sm font-bold truncate",
                                "{layer.title}"
                            }
                        }
                    }
                }

                // Active Subsystem Inspector
                LayerInspector { layer: active_layer }
            }
        }
    }
}
