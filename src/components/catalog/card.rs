//! ## Overview
//! Single hardware catalog card component displaying neutral gray backdrop, product render, and tactile plus sign.
//! Faithfully mirrors the Teenage Engineering Store card aesthetic with crisp lowercase typography.
//!
//! ## Search Tags
//! #card, #catalog, #hardware, #render, #ui

use dioxus::prelude::*;
use super::data::HardwareUnit;

/// Single hardware catalog card item with light neutral gray backdrop and delicate plus sign.
#[component]
pub fn CatalogCard(unit: HardwareUnit, on_select: EventHandler<HardwareUnit>) -> Element {
    let u_click = unit.clone();

    rsx! {
        div {
            class: "group cursor-pointer select-none",
            onclick: move |_| on_select.call(u_click.clone()),
            // Neutral light gray box matching Teenage Engineering Store
            div {
                class: "bg-[#F2F2F2] aspect-[4/3] rounded-xs flex items-center justify-center p-6 sm:p-10 overflow-hidden group-hover:bg-[#EAEAEA] transition-colors relative",
                img {
                    src: "{unit.image}",
                    alt: "{unit.title}",
                    class: "max-h-full max-w-full object-contain filter drop-shadow-xs group-hover:scale-[1.02] transition-transform duration-200"
                }
            }
            // Bottom metadata row
            div {
                class: "mt-3 flex items-start justify-between gap-4",
                div {
                    class: "flex flex-col",
                    span {
                        class: "text-sm font-sans font-medium text-zinc-900 lowercase leading-tight group-hover:text-zinc-600 transition-colors",
                        "{unit.title}"
                    }
                    span {
                        class: "text-xs font-mono text-zinc-500 lowercase mt-1",
                        "{unit.subtitle}"
                    }
                }
                button {
                    r#type: "button",
                    aria_label: "Inspect Hardware Specifications",
                    class: "text-lg font-light text-zinc-400 group-hover:text-zinc-950 transition-colors px-1 shrink-0",
                    "+"
                }
            }
        }
    }
}
