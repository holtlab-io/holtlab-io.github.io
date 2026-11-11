//! ## Overview
//! Studio teaser and summary components rendered on the primary overview page.
//! Includes the flagship demonstrator preview card, turnkey methodology summary, and direct inquiry dispatch banner.
//!
//! ## Search Tags
//! #teasers, #summary, #demonstrator-card, #inquiry-banner, #showcase
//!
//! ## Submodules
//! None. This module provides atomic presentation teaser components.

use dioxus::prelude::*;
use crate::i18n::{teasers::get_home_teaser_bundle, teasers::DemonstratorTeaserStrings, Language};
use crate::routes::Route;

/// Featured flagship demonstrator teaser card highlighting Opus 01.
#[component]
pub fn DemonstratorTeaser() -> Element {
    let lang = use_context::<Signal<Language>>();
    let bundle = get_home_teaser_bundle(lang());
    let strings = bundle.demonstrator;

    rsx! {
        section {
            class: "py-16 sm:py-24 bg-white border-t border-zinc-200 font-sans relative overflow-hidden",
            div { class: "absolute inset-0 bg-tech-dots opacity-30 pointer-events-none" }
            div {
                class: "max-w-[1500px] mx-auto px-4 sm:px-8 relative z-10",
                div {
                    class: "studio-card rounded-xs bg-[#FAFAFA] border border-zinc-200/90 p-6 sm:p-10 lg:p-12 shadow-sm relative overflow-hidden reveal-on-scroll delay-100",
                    div {
                        class: "grid grid-cols-1 lg:grid-cols-12 gap-8 lg:gap-12 items-center",
                        DemonstratorTextColumn { strings }
                        DemonstratorVisualColumn { status_badge: strings.status_badge }
                    }
                }
            }
        }
    }
}

/// Text column containing eyebrow, headline, description, pills, and inspect CTA button.
#[component]
fn DemonstratorTextColumn(strings: DemonstratorTeaserStrings) -> Element {
    rsx! {
        div {
            class: "lg:col-span-7 space-y-6",
            div {
                class: "inline-flex items-center gap-2 px-3 py-1 rounded-full bg-white border border-zinc-200 text-zinc-700 font-mono text-[11px] font-semibold tracking-wider uppercase shadow-2xs",
                span { class: "w-2 h-2 rounded-full bg-[#E64A19] animate-pulse" }
                "{strings.eyebrow}"
            }
            h2 {
                class: "text-2xl sm:text-4xl lg:text-5xl font-bold font-sans tracking-tight text-zinc-950 leading-tight break-keep",
                "{strings.title}"
            }
            p {
                class: "text-sm sm:text-base text-zinc-600 font-normal leading-relaxed max-w-2xl break-keep",
                "{strings.subtitle}"
            }
            div {
                class: "flex flex-wrap gap-2 pt-2",
                for pill in strings.pills.iter() {
                    span {
                        key: "{pill}",
                        class: "px-3 py-1.5 rounded-xs bg-white border border-zinc-200 text-zinc-800 font-mono text-xs font-medium shadow-2xs",
                        "{pill}"
                    }
                }
            }
            div {
                class: "pt-4",
                Link {
                    to: Route::Demonstrator {},
                    class: "tactile-button inline-flex items-center gap-2 px-6 py-3.5 rounded-xs bg-zinc-950 hover:bg-zinc-800 text-white font-mono text-xs font-semibold tracking-wider transition-colors shadow-xs",
                    span { "{strings.cta}" }
                }
            }
        }
    }
}

/// Visual column displaying the exploded assembly render with telemetry badges.
#[component]
fn DemonstratorVisualColumn(status_badge: &'static str) -> Element {
    rsx! {
        div {
            class: "lg:col-span-5 flex justify-center items-center relative",
            div {
                class: "w-full max-w-md bg-white border border-zinc-200/80 rounded-xs p-4 shadow-sm relative overflow-hidden group",
                img {
                    src: "/assets/showcase/mcad_exploded.png",
                    alt: "Case Study 01 Exploded Assembly Visual",
                    class: "w-full h-auto object-contain transition-transform duration-500 group-hover:scale-105 drop-shadow-md",
                }
                div {
                    class: "mt-3 pt-3 border-t border-zinc-100 flex items-center justify-between font-mono text-[10px] text-zinc-400",
                    span { "MJF PA12 // 4-LAYER PCBA" }
                    span { class: "text-emerald-600 font-bold", "{status_badge}" }
                }
            }
        }
    }
}

/// Turnkey methodology teaser summary card comparing fragmented models with Holt Lab.
#[component]
pub fn StudioTurnkeyTeaser() -> Element {
    let lang = use_context::<Signal<Language>>();
    let bundle = get_home_teaser_bundle(lang());
    let strings = bundle.studio;

    rsx! {
        section {
            class: "py-16 sm:py-20 bg-[#FAFAFA] border-t border-zinc-200 font-sans relative overflow-hidden",
            div { class: "max-w-[1500px] mx-auto px-4 sm:px-8 relative z-10",
                div {
                    class: "studio-card rounded-xs bg-white border border-zinc-200 p-6 sm:p-10 shadow-2xs flex flex-col md:flex-row md:items-center justify-between gap-8 reveal-on-scroll delay-200",
                    div {
                        class: "space-y-3 max-w-3xl",
                        div {
                            class: "inline-flex items-center gap-2 font-mono text-xs text-[#E64A19] font-bold uppercase tracking-wider",
                            span { class: "w-1.5 h-1.5 rounded-full bg-[#E64A19]" }
                            "{strings.eyebrow}"
                        }
                        h3 {
                            class: "text-xl sm:text-3xl font-bold font-sans tracking-tight text-zinc-950 break-keep",
                            "{strings.title}"
                        }
                        p {
                            class: "text-xs sm:text-sm text-zinc-600 font-normal leading-relaxed break-keep",
                            "{strings.subtitle}"
                        }
                    }
                    div {
                        class: "shrink-0",
                        Link {
                            to: Route::StudioPage {},
                            class: "tactile-button inline-flex items-center gap-2 px-6 py-3.5 rounded-xs bg-white hover:bg-zinc-50 border border-zinc-300 text-zinc-900 font-mono text-xs font-semibold tracking-wider transition-colors shadow-2xs",
                            span { "{strings.cta}" }
                        }
                    }
                }
            }
        }
    }
}

/// Direct client consultation and project brief submission call-to-action banner.
#[component]
pub fn InquiryCtaBanner() -> Element {
    let lang = use_context::<Signal<Language>>();
    let bundle = get_home_teaser_bundle(lang());
    let strings = bundle.inquiry;

    rsx! {
        section {
            class: "py-16 sm:py-24 bg-zinc-950 text-white font-sans relative overflow-hidden border-t border-zinc-800",
            div { class: "absolute inset-0 bg-tech-grid opacity-10 pointer-events-none" }
            div {
                class: "max-w-[1400px] mx-auto px-4 sm:px-8 text-center relative z-10 flex flex-col items-center space-y-6 reveal-on-scroll delay-100",
                div {
                    class: "inline-flex items-center gap-2 px-3 py-1 rounded-full bg-zinc-900 border border-zinc-800 text-zinc-400 font-mono text-[11px] font-semibold tracking-wider uppercase",
                    span { class: "w-2 h-2 rounded-full bg-[#E64A19] animate-pulse" }
                    "{strings.eyebrow}"
                }
                h2 {
                    class: "text-3xl sm:text-5xl font-bold font-sans tracking-tight text-white leading-tight max-w-3xl",
                    "{strings.title}"
                }
                p {
                    class: "text-sm sm:text-base text-zinc-400 font-normal leading-relaxed max-w-2xl",
                    "{strings.subtitle}"
                }
                div {
                    class: "pt-4",
                    Link {
                        to: Route::ContactPage {},
                        class: "tactile-button inline-flex items-center gap-2 px-8 py-4 rounded-xs bg-[#E64A19] hover:bg-[#c93c12] text-white font-mono text-xs sm:text-sm font-semibold tracking-wider transition-colors shadow-md",
                        span { "{strings.cta}" }
                    }
                }
            }
        }
    }
}
