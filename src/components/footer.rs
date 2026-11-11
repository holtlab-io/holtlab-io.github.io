//! ## Overview
//! Minimalist wireframe footer displaying brand identity, turnkey verification standards, and coordinates.
//! Features links to GitHub organization, YouTube channel, and direct studio dispatch email.
//!
//! ## Search Tags
//! #footer, #branding, #links, #copyright, #channels
//!
//! ## Submodules
//! None. This module provides atomic footer layout components.

use dioxus::prelude::*;
use crate::i18n::{hero::get_footer_strings, Language};

/// Primary footer component displaying brand information, channels, and copyright notes.
#[component]
pub fn Footer() -> Element {
    let lang = use_context::<Signal<Language>>();
    let footer_strings = get_footer_strings(lang());

    rsx! {
        footer {
            class: "py-12 bg-white border-t border-zinc-200 text-zinc-500 text-xs font-mono select-none",
            div {
                class: "max-w-[1500px] mx-auto px-4 sm:px-8",
                div {
                    class: "flex flex-col md:flex-row items-center justify-between gap-6 pb-8 border-b border-zinc-100",
                    // Brand Info
                    div {
                        class: "flex items-center gap-3",
                        span {
                            class: "font-bold text-zinc-950 font-sans tracking-tight text-sm lowercase",
                            "holt lab"
                        }
                        span { class: "text-zinc-300", "//" }
                        span {
                            class: "text-zinc-500 text-xs font-sans",
                            "{footer_strings.studio_desc}"
                        }
                    }

                    // Social & Coordinates
                    div {
                        class: "flex items-center gap-6 text-xs",
                        a {
                            href: "https://github.com/holtlab-io",
                            target: "_blank",
                            class: "text-zinc-500 hover:text-zinc-950 transition-colors flex items-center gap-1",
                            span { "GitHub" }
                            span { class: "text-[10px] text-zinc-400", "↗" }
                        }
                        a {
                            href: "https://youtube.com/@holt-lab",
                            target: "_blank",
                            class: "text-zinc-500 hover:text-zinc-950 transition-colors flex items-center gap-1",
                            span { "YouTube" }
                            span { class: "text-[10px] text-zinc-400", "↗" }
                        }
                        a {
                            href: "mailto:contact@holtlab.io",
                            class: "text-zinc-500 hover:text-zinc-950 transition-colors",
                            "contact@holtlab.io"
                        }
                    }
                }

                // Bottom Copyright
                div {
                    class: "pt-6 flex flex-col sm:flex-row items-center justify-between gap-3 text-zinc-400 text-[11px]",
                    div {
                        "{footer_strings.clearance_note}"
                    }
                    div {
                        "{footer_strings.copyright}"
                    }
                }
            }
        }
    }
}
