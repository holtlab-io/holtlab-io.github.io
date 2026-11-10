//! ## Overview
//! Interactive subsystem inspector card component for the anatomical teardown section.
//! Renders the selected layer's high-resolution visual, engineering telemetry, and technical achievements.
//!
//! ## Search Tags
//! #teardown, #inspector, #subsystem, #specs, #interactive
//!
//! ## Submodules
//! None. This module provides atomic interactive layer viewer components.

use dioxus::prelude::*;
use super::data::TeardownLayer;
use crate::i18n::Language;

/// Interactive subsystem inspector displaying visual assets and verified specifications.
#[component]
pub fn LayerInspector(layer: TeardownLayer) -> Element {
    let lang = use_context::<Signal<Language>>();
    let is_ko = lang() == Language::Ko;
    let audit_label = if is_ko { "홀트 랩 감사 통과" } else { "HOLT LAB AUDIT PASS" };

    rsx! {
        div {
            class: "grid grid-cols-1 lg:grid-cols-12 gap-8 lg:gap-12 items-center bg-white border border-zinc-200 rounded-xs p-6 sm:p-10 shadow-2xs font-sans",

            // Visual Column (Left)
            div {
                class: "lg:col-span-6 flex flex-col justify-between bg-zinc-50 border border-zinc-200/80 rounded-xs p-4 sm:p-6 relative overflow-hidden group min-h-[380px]",
                div {
                    class: "w-full flex flex-wrap items-center justify-between gap-2 z-10 mb-2",
                    div {
                        class: "px-2.5 py-1 rounded-xs bg-zinc-950 text-white font-mono text-[10px] font-bold tracking-wider shrink-0",
                        "LAYER {layer.number}"
                    }
                    div {
                        class: "px-2.5 py-1 rounded-xs bg-emerald-50 text-emerald-700 border border-emerald-200 font-mono text-[10px] font-semibold truncate",
                        "{layer.verification_badge}"
                    }
                }
                div {
                    class: "flex-1 flex items-center justify-center py-2",
                    img {
                        src: "{layer.image_src}",
                        alt: "{layer.image_alt}",
                        class: "max-h-[320px] sm:max-h-[360px] w-full object-contain drop-shadow-md transition-transform duration-500 group-hover:scale-105"
                    }
                }
                div {
                    class: "w-full pt-3 border-t border-zinc-200/60 flex items-center justify-between text-[10px] font-mono text-zinc-400 gap-2",
                    span { class: "truncate", "{layer.bus_or_material}" }
                    span { class: "shrink-0", "{audit_label}" }
                }
            }

            // Specs & Details Column (Right)
            div {
                class: "lg:col-span-6 flex flex-col space-y-6",
                LayerDetailsHeader { layer }
                LayerSpecsGrid { layer }
                LayerBulletList { layer }
            }
        }
    }
}

/// Header portion of the layer details view.
#[component]
fn LayerDetailsHeader(layer: TeardownLayer) -> Element {
    rsx! {
        div {
            class: "space-y-2 border-b border-zinc-200 pb-5",
            div {
                class: "text-xs font-mono text-[#E64A19] font-semibold tracking-wider uppercase",
                "SUBSYSTEM {layer.number}"
            }
            h3 {
                class: "text-2xl sm:text-3xl font-bold font-sans tracking-tight text-zinc-950 lowercase",
                "{layer.title}"
            }
            p {
                class: "text-xs sm:text-sm font-mono text-zinc-500 lowercase leading-relaxed",
                "{layer.subtitle}"
            }
        }
    }
}

/// 2-metric key telemetry grid for the layer.
#[component]
fn LayerSpecsGrid(layer: TeardownLayer) -> Element {
    let lang = use_context::<Signal<Language>>();
    let is_ko = lang() == Language::Ko;
    let spec_title = if is_ko { "주요 사양" } else { "PRIMARY SPEC" };
    let tol_title = if is_ko { "보조 공차" } else { "SECONDARY TOLERANCE" };

    rsx! {
        div {
            class: "grid grid-cols-1 sm:grid-cols-2 gap-3 text-xs font-mono lowercase",
            div {
                class: "p-3 rounded-xs bg-zinc-50 border border-zinc-200",
                span { class: "block text-[10px] text-zinc-400 uppercase tracking-wider mb-1", "{spec_title}" }
                span { class: "font-bold text-zinc-900", "{layer.primary_spec}" }
            }
            div {
                class: "p-3 rounded-xs bg-zinc-50 border border-zinc-200",
                span { class: "block text-[10px] text-zinc-400 uppercase tracking-wider mb-1", "{tol_title}" }
                span { class: "font-bold text-zinc-900", "{layer.secondary_spec}" }
            }
        }
    }
}

/// Technical bullet features and deep architecture summary.
#[component]
fn LayerBulletList(layer: TeardownLayer) -> Element {
    rsx! {
        div {
            class: "space-y-4 pt-1",
            p {
                class: "text-xs sm:text-sm text-zinc-700 leading-relaxed font-normal",
                "{layer.deep_summary}"
            }
            ul {
                class: "space-y-2 text-xs font-mono text-zinc-600 border-t border-zinc-100 pt-4",
                li {
                    class: "flex items-start gap-2",
                    span { class: "text-[#E64A19] font-bold", "•" }
                    span { "{layer.bullet_1}" }
                }
                li {
                    class: "flex items-start gap-2",
                    span { class: "text-[#E64A19] font-bold", "•" }
                    span { "{layer.bullet_2}" }
                }
                li {
                    class: "flex items-start gap-2",
                    span { class: "text-[#E64A19] font-bold", "•" }
                    span { "{layer.bullet_3}" }
                }
            }
        }
    }
}
