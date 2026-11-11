//! ## Overview
//! Contact info and value assurance column presenting Holt Lab direct communication channels and SLAs.
//! Employs a quiet Swiss layout with localized coordinates and response commitment.
//!
//! ## Search Tags
//! #contact, #info, #sla, #guarantee, #email
//!
//! ## Submodules
//! None. This module provides atomic contact information components.

use dioxus::prelude::*;
use crate::i18n::{contact::get_contact_strings, Language};

/// Left column displaying direct contact information and institutional coordinates.
#[component]
pub fn ContactInfo() -> Element {
    let lang = use_context::<Signal<Language>>();
    let info = get_contact_strings(lang()).info;

    rsx! {
        div {
            class: "lg:col-span-5 space-y-6 font-sans reveal-on-scroll",
            div {
                class: "space-y-3",
                span { class: "text-xs font-mono text-zinc-400 uppercase tracking-wider", "{info.eyebrow}" }
                h2 {
                    class: "text-2xl sm:text-3xl font-bold text-zinc-950 tracking-tight leading-tight break-keep",
                    "{info.title}"
                }
                p {
                    class: "text-zinc-600 text-xs sm:text-sm font-sans leading-relaxed pt-1 break-keep",
                    "{info.body}"
                }
            }

            div {
                class: "space-y-3 text-xs font-mono pt-2",
                div {
                    class: "p-4 rounded-xs border border-zinc-200 bg-white flex items-center justify-between shadow-2xs",
                    div {
                        div { class: "text-zinc-400 text-[10px] uppercase font-mono", "{info.email_label}" }
                        a {
                            href: "mailto:contact@holtlab.io",
                            class: "text-sm font-bold text-zinc-950 hover:underline transition-colors mt-0.5 block",
                            "contact@holtlab.io"
                        }
                    }
                    span { class: "text-zinc-400 text-xs font-mono", "→" }
                }

                div {
                    class: "p-4 rounded-xs border border-zinc-200 bg-white flex items-center justify-between shadow-2xs",
                    div {
                        div { class: "text-zinc-400 text-[10px] uppercase font-mono", "{info.location_label}" }
                        div { class: "text-xs font-semibold text-zinc-800 mt-0.5", "{info.location_val}" }
                    }
                    span { class: "text-zinc-900 text-xs font-bold font-mono", "{info.location_badge}" }
                }

                div {
                    class: "p-4 rounded-xs border border-zinc-200 bg-white flex items-center justify-between shadow-2xs",
                    div {
                        div { class: "text-zinc-400 text-[10px] uppercase font-mono", "{info.sla_label}" }
                        div { class: "text-xs font-semibold text-zinc-800 mt-0.5", "{info.sla_val}" }
                    }
                    span { class: "text-zinc-600 text-xs font-mono font-medium", "{info.sla_badge}" }
                }
            }
        }
    }
}
