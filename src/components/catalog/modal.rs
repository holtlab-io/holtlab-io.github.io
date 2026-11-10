//! ## Overview
//! Swiss engineering specification modal overlay for interactive hardware inspection.
//! Renders dimensions, silicon parameters, bus speeds, tolerances, and direct commission links.
//!
//! ## Search Tags
//! #modal, #inspector, #specs, #datasheet, #overlay

use dioxus::prelude::*;
use super::data::HardwareUnit;

/// Swiss hardware inspection modal overlay displaying deep engineering specifications.
#[component]
pub fn HardwareModal(unit: HardwareUnit, on_close: EventHandler<()>) -> Element {
    rsx! {
        div {
            class: "fixed inset-0 z-50 bg-black/40 backdrop-blur-xs flex items-center justify-center p-4 sm:p-6 overflow-y-auto animate-fadeIn",
            onclick: move |_| on_close.call(()),
            div {
                class: "bg-white rounded-lg max-w-3xl w-full p-6 sm:p-8 shadow-2xl relative space-y-6 my-auto max-h-[90vh] overflow-y-auto",
                onclick: move |e| e.stop_propagation(),

                ModalHeader { unit: unit.clone(), on_close }
                ModalPreview { unit: unit.clone() }
                ModalSpecTable { unit: unit.clone() }
                ModalFooter { on_close }
            }
        }
    }
}

/// Modal header with product title and close button.
#[component]
fn ModalHeader(unit: HardwareUnit, on_close: EventHandler<()>) -> Element {
    rsx! {
        div {
            class: "flex items-start justify-between border-b border-zinc-200 pb-4",
            div {
                class: "flex flex-col",
                span { class: "text-xs font-mono text-zinc-400 uppercase tracking-wider", "SPECIFICATION // {unit.category}" }
                h3 { class: "text-lg sm:text-xl font-bold font-sans lowercase text-zinc-950 mt-1", "{unit.title}" }
            }
            button {
                r#type: "button",
                aria_label: "Close Modal",
                class: "text-xl font-mono text-zinc-400 hover:text-zinc-950 transition-colors p-1",
                onclick: move |_| on_close.call(()),
                "✕"
            }
        }
    }
}

/// Image preview block within modal.
#[component]
fn ModalPreview(unit: HardwareUnit) -> Element {
    rsx! {
        div {
            class: "bg-[#F4F4F4] aspect-video rounded-xs flex items-center justify-center p-6 overflow-hidden",
            img {
                src: "{unit.image}",
                alt: "{unit.title}",
                class: "max-h-full max-w-full object-contain filter drop-shadow-sm"
            }
        }
    }
}

/// Key-value engineering table inside modal.
#[component]
fn ModalSpecTable(unit: HardwareUnit) -> Element {
    rsx! {
        div {
            class: "border border-zinc-200 divide-y divide-zinc-200 text-xs font-mono lowercase",
            div { class: "grid grid-cols-3 p-2.5", span { class: "text-zinc-400 font-medium", "dimensions" }, span { class: "col-span-2 text-zinc-900 font-bold", "{unit.dimensions}" } }
            div { class: "grid grid-cols-3 p-2.5", span { class: "text-zinc-400 font-medium", "silicon & bus" }, span { class: "col-span-2 text-zinc-900", "{unit.silicon}" } }
            div { class: "grid grid-cols-3 p-2.5", span { class: "text-zinc-400 font-medium", "display / touch" }, span { class: "col-span-2 text-zinc-900", "{unit.display}" } }
            div { class: "grid grid-cols-3 p-2.5", span { class: "text-zinc-400 font-medium", "enclosure" }, span { class: "col-span-2 text-zinc-900", "{unit.material}" } }
            div { class: "grid grid-cols-3 p-2.5", span { class: "text-zinc-400 font-medium", "pcba stackup" }, span { class: "col-span-2 text-zinc-900", "{unit.stackup}" } }
            div { class: "grid grid-cols-3 p-2.5", span { class: "text-zinc-400 font-medium", "firmware" }, span { class: "col-span-2 text-zinc-900", "{unit.firmware}" } }
            div { class: "grid grid-cols-3 p-2.5", span { class: "text-zinc-400 font-medium", "cad tolerance" }, span { class: "col-span-2 text-zinc-900 font-bold text-zinc-950", "{unit.tolerance}" } }
        }
    }
}

/// Modal footer with CTA and dismiss button.
#[component]
fn ModalFooter(on_close: EventHandler<()>) -> Element {
    rsx! {
        div {
            class: "flex items-center justify-between pt-2",
            a {
                href: "#contact",
                onclick: move |_| on_close.call(()),
                class: "inline-flex items-center gap-2 px-5 py-2.5 rounded-sm bg-zinc-950 hover:bg-zinc-800 text-white text-xs font-mono uppercase tracking-wider transition-colors",
                "inquire for this hardware architecture →"
            }
            button {
                r#type: "button",
                onclick: move |_| on_close.call(()),
                class: "text-xs font-mono text-zinc-500 hover:text-zinc-950 lowercase",
                "dismiss [esc]"
            }
        }
    }
}
