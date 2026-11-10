//! ## Overview
//! Minimalist wireframe footer displaying brand identity, verified tolerance standards, and coordinates.
//! Follows the Teenage Engineering understated typographic convention.
//!
//! ## Search Tags
//! #footer, #branding, #links, #copyright, #channels

use dioxus::prelude::*;

/// Primary footer component displaying brand information, channels, and copyright notes.
#[component]
pub fn Footer() -> Element {
    rsx! {
        footer {
            class: "py-12 bg-white border-t border-zinc-200 text-zinc-500 text-xs font-mono lowercase select-none",
            div {
                class: "max-w-[1600px] mx-auto px-4 sm:px-8",
                div {
                    class: "flex flex-col md:flex-row items-center justify-between gap-6 pb-8 border-b border-zinc-100",
                    // Brand Info
                    div {
                        class: "flex items-center gap-3",
                        span {
                            class: "font-bold text-zinc-950 font-sans tracking-tight text-sm",
                            "holt lab"
                        }
                        span { class: "text-zinc-300", "//" }
                        span {
                            class: "text-zinc-500 text-xs",
                            "boutique hardware & embedded laboratory"
                        }
                    }

                    // Social & Coordinates
                    div {
                        class: "flex items-center gap-6 text-xs",
                        a {
                            href: "https://github.com/imwoo90",
                            target: "_blank",
                            class: "text-zinc-500 hover:text-zinc-950 transition-colors flex items-center gap-1",
                            span { "github" }
                            span { class: "text-[10px] text-zinc-400", "↗" }
                        }
                        a {
                            href: "https://youtube.com/@holt-lab",
                            target: "_blank",
                            class: "text-zinc-500 hover:text-zinc-950 transition-colors flex items-center gap-1",
                            span { "youtube" }
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
                        "all systems verified 0.00 mm³ physical clearance · seoul, korea"
                    }
                    div {
                        "© 2026 holt lab. all rights reserved."
                    }
                }
            }
        }
    }
}
