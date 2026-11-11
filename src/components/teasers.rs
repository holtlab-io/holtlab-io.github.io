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

/// Featured hardware case study showcase rendered in high-contrast dark studio tones.
#[component]
pub fn DemonstratorTeaser() -> Element {
    let lang = use_context::<Signal<Language>>();
    let bundle = get_home_teaser_bundle(lang());
    let strings = bundle.demonstrator;

    rsx! {
        section {
            class: "py-16 sm:py-24 bg-zinc-900 text-white border-t border-zinc-800 font-sans relative overflow-hidden",
            div { class: "absolute inset-0 bg-tech-grid opacity-10 pointer-events-none" }
            div {
                class: "max-w-[1400px] mx-auto px-4 sm:px-8 relative z-10",
                div {
                    class: "grid grid-cols-1 lg:grid-cols-12 gap-10 lg:gap-12 items-center reveal-on-scroll delay-100",
                    DemonstratorTextColumn { strings }
                    DemonstratorVisualColumn { lang: lang(), status_badge: strings.status_badge }
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
            class: "lg:col-span-6 space-y-6",
            div {
                class: "inline-flex items-center gap-2 px-3 py-1 rounded-full bg-zinc-800/90 border border-zinc-700 text-zinc-300 font-mono text-[11px] font-semibold tracking-wider uppercase",
                span { class: "w-2 h-2 rounded-full bg-[#E64A19] animate-pulse" }
                "{strings.eyebrow}"
            }
            h2 {
                class: "text-2xl sm:text-4xl lg:text-5xl font-bold font-sans tracking-tight text-white leading-tight break-keep",
                "{strings.title}"
            }
            p {
                class: "text-sm sm:text-base text-zinc-300 font-normal leading-relaxed max-w-2xl break-keep",
                "{strings.subtitle}"
            }
            div {
                class: "flex flex-wrap gap-2 pt-2",
                for pill in strings.pills.iter() {
                    span {
                        key: "{pill}",
                        class: "px-3 py-1.5 rounded-xs bg-zinc-800/80 border border-zinc-700/90 text-zinc-200 font-mono text-xs font-medium",
                        "{pill}"
                    }
                }
            }
            div {
                class: "pt-4",
                Link {
                    to: Route::Demonstrator {},
                    class: "tactile-button inline-flex items-center gap-2 px-6 py-3.5 rounded-xs bg-[#E64A19] hover:bg-[#c93c12] text-white font-mono text-xs font-semibold tracking-wider transition-colors shadow-sm",
                    span { "{strings.cta}" }
                }
            }
        }
    }
}

/// Visual column displaying real powered-on prototype and internal 4-layer SMT hardware photos.
#[component]
fn DemonstratorVisualColumn(lang: Language, status_badge: &'static str) -> Element {
    let (front_caption, inner_caption) = match lang {
        Language::En => ("LIVE 60Hz IPS UI", "4L PCBA & PA12 FIT"),
        Language::Ko => ("실기 60Hz 구동 화면", "4층 기판 · 외함 내부"),
    };

    rsx! {
        div {
            class: "lg:col-span-6 flex flex-col gap-3",
            div {
                class: "grid grid-cols-2 gap-3 sm:gap-4",
                div {
                    class: "bg-zinc-950 border border-zinc-800 rounded-xs p-2.5 sm:p-3 shadow-lg overflow-hidden group",
                    div {
                        class: "aspect-[4/5] overflow-hidden rounded-2xs bg-zinc-900",
                        img {
                            src: "/assets/showcase/real_front_on.jpg",
                            alt: "Win95 Minesweeper Touch Keyring Powered-On Prototype",
                            class: "w-full h-full object-cover transition-transform duration-500 group-hover:scale-[1.03]",
                        }
                    }
                    div {
                        class: "pt-2.5 flex items-center justify-between font-mono text-[10px] text-zinc-400",
                        span { class: "truncate", "{front_caption}" }
                        span { class: "w-1.5 h-1.5 rounded-full bg-emerald-400 shrink-0" }
                    }
                }
                div {
                    class: "bg-zinc-950 border border-zinc-800 rounded-xs p-2.5 sm:p-3 shadow-lg overflow-hidden group",
                    div {
                        class: "aspect-[4/5] overflow-hidden rounded-2xs bg-zinc-900",
                        img {
                            src: "/assets/showcase/real_knolling_top.jpg",
                            alt: "4-Layer SMT PCBA and MJF PA12 Internal Ribs",
                            class: "w-full h-full object-cover transition-transform duration-500 group-hover:scale-[1.03]",
                        }
                    }
                    div {
                        class: "pt-2.5 flex items-center justify-between font-mono text-[10px] text-zinc-400",
                        span { class: "truncate", "{inner_caption}" }
                        span { class: "text-zinc-500 shrink-0", "REV A" }
                    }
                }
            }
            div {
                class: "px-3.5 py-2.5 rounded-xs bg-zinc-950/90 border border-zinc-800 flex items-center justify-between font-mono text-[11px]",
                span { class: "text-zinc-400", "MJF PA12 // 4-LAYER SMT PCBA" }
                span { class: "text-emerald-400 font-semibold", "{status_badge}" }
            }
        }
    }
}

/// Editorial methodology summary section comparing fragmented outsourcing with unified engineering.
#[component]
pub fn StudioTurnkeyTeaser() -> Element {
    let lang = use_context::<Signal<Language>>();
    let bundle = get_home_teaser_bundle(lang());
    let strings = bundle.studio;

    rsx! {
        section {
            class: "py-16 sm:py-24 bg-white border-t border-zinc-200 font-sans relative overflow-hidden",
            div {
                class: "max-w-[1400px] mx-auto px-4 sm:px-8 relative z-10",
                div {
                    class: "border-l-2 border-[#E64A19] pl-6 sm:pl-10 py-2 flex flex-col md:flex-row md:items-end justify-between gap-8 reveal-on-scroll delay-200",
                    div {
                        class: "space-y-4 max-w-3xl",
                        div {
                            class: "inline-flex items-center gap-2 font-mono text-xs text-[#E64A19] font-bold uppercase tracking-wider",
                            "{strings.eyebrow}"
                        }
                        h3 {
                            class: "text-2xl sm:text-4xl font-bold font-sans tracking-tight text-zinc-950 leading-tight break-keep",
                            "{strings.title}"
                        }
                        p {
                            class: "text-sm sm:text-base text-zinc-600 font-normal leading-relaxed break-keep",
                            "{strings.subtitle}"
                        }
                    }
                    div {
                        class: "shrink-0",
                        Link {
                            to: Route::StudioPage {},
                            class: "tactile-button inline-flex items-center gap-2 px-6 py-3.5 rounded-xs bg-zinc-950 hover:bg-zinc-800 text-white font-mono text-xs font-semibold tracking-wider transition-colors shadow-2xs",
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
