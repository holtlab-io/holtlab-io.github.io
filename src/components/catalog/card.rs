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
            // Neutral light gray box matching Teenage Engineering Store (#F5F5F5)
            div {
                class: "bg-[#F5F5F5] aspect-[16/10] rounded-none flex items-center justify-center p-8 sm:p-12 overflow-hidden group-hover:bg-[#EFEFEF] transition-colors relative",
                img {
                    src: "{unit.image}",
                    alt: "{unit.title}",
                    class: "max-h-full max-w-full object-contain filter drop-shadow-xs group-hover:scale-[1.02] transition-transform duration-200"
                }
            }
            // Bottom metadata row
            div {
                class: "mt-3.5 flex items-start justify-between gap-4",
                div {
                    class: "flex flex-col",
                    span {
                        class: "text-[15px] sm:text-base font-sans font-normal text-zinc-950 lowercase leading-snug tracking-tight group-hover:text-zinc-600 transition-colors",
                        "{unit.title}"
                    }
                    span {
                        class: "text-xs sm:text-[13px] font-sans text-zinc-500 lowercase mt-0.5 leading-snug",
                        "{unit.subtitle}"
                    }
                }
                button {
                    r#type: "button",
                    aria_label: "Inspect Hardware Specifications",
                    class: "text-base font-light text-zinc-400 group-hover:text-zinc-950 transition-colors px-1 shrink-0 pt-0.5",
                    "+"
                }
            }
        }
    }
}
