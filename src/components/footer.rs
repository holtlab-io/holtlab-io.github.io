//! ## Overview
//! Studio brand footer presenting identity, copyright statements, and external community channels.
//!
//! ## Search Tags
//! #footer, #branding, #links, #copyright, #channels

use dioxus::prelude::*;

/// Primary footer component displaying brand information, social channels, and copyright notes.
#[component]
pub fn Footer() -> Element {
    rsx! {
        footer {
            class: "py-12 bg-[#F4F4F5] border-t border-zinc-200 text-zinc-500 text-xs font-mono",
            div {
                class: "max-w-7xl mx-auto px-4 sm:px-6 lg:px-8",
                div {
                    class: "flex flex-col md:flex-row items-center justify-between gap-6 pb-8 border-b border-zinc-200/80",
                    // Brand Info
                    div {
                        class: "flex items-center gap-3",
                        img {
                            src: "assets/branding/avatar.jpg",
                            alt: "Holt Lab Luwim Avatar",
                            class: "w-8 h-8 rounded-md object-cover border border-zinc-200"
                        }
                        div {
                            div {
                                class: "text-zinc-900 font-bold text-xs tracking-wider",
                                "HOLT LAB // BOUTIQUE HARDWARE STUDIO"
                            }
                            div {
                                class: "text-zinc-500 text-[10px]",
                                "Mechanical (MCAD) · Electronics (ECAD) · Bare-Metal Firmware"
                            }
                        }
                    }

                    // Social & Channel Links
                    div {
                        class: "flex items-center gap-6 text-xs",
                        a {
                            href: "https://github.com/holtlab-io",
                            target: "_blank",
                            class: "text-zinc-600 hover:text-zinc-950 transition-colors flex items-center gap-1",
                            span { "GITHUB" }
                            span { class: "text-[10px] text-zinc-400", "↗" }
                        }
                        a {
                            href: "https://youtube.com/@holt-lab",
                            target: "_blank",
                            class: "text-zinc-600 hover:text-zinc-950 transition-colors flex items-center gap-1",
                            span { "YOUTUBE" }
                            span { class: "text-[10px] text-zinc-400", "↗" }
                        }
                        a {
                            href: "mailto:contact@holtlab.io",
                            class: "text-zinc-600 hover:text-zinc-950 transition-colors flex items-center gap-1",
                            span { "contact@holtlab.io" }
                        }
                    }
                }

                // Bottom Copyright
                div {
                    class: "pt-6 flex flex-col sm:flex-row items-center justify-between gap-3 text-zinc-500 text-[10px]",
                    div {
                        "© 2026 HOLT LAB. ALL RIGHTS RESERVED. ZERO-PINGPONG HARDWARE ARCHITECTURE."
                    }
                    div {
                        "BUILT WITH RUST & DIOXUS 0.7 (WASM) // GITHUB PAGES"
                    }
                }
            }
        }
    }
}
