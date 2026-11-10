//! ## Overview
//! Interactive selection buttons for project scope and client organization category.
//! Provides quiet lowercase options for engineering disciplines and organizational scope.
//!
//! ## Search Tags
//! #contact, #selectors, #scope, #buttons, #options

use dioxus::prelude::*;

/// Selector buttons for development scope.
#[component]
pub fn ScopeButtons(mut selected: Signal<usize>) -> Element {
    let scopes = [
        "01 // turnkey product (mcad+ecad+fw)",
        "02 // 4-layer pcba (kicad ecad)",
        "03 // 3d chassis & enclosure (mcad)",
        "04 // bare-metal firmware (rust/c)",
    ];

    rsx! {
        div {
            class: "space-y-2",
            label { class: "block text-xs font-mono text-zinc-500 lowercase", "engineering discipline" }
            div {
                class: "grid grid-cols-1 sm:grid-cols-2 gap-2 text-xs font-mono lowercase",
                for (idx, scope) in scopes.iter().enumerate() {
                    button {
                        r#type: "button",
                        onclick: move |_| selected.set(idx),
                        class: if selected() == idx {
                            "p-3 rounded-xs border-2 border-zinc-950 bg-zinc-100 text-zinc-950 font-bold text-left transition-all"
                        } else {
                            "p-3 rounded-xs border border-zinc-200 bg-zinc-50/50 text-zinc-600 hover:border-zinc-300 hover:text-zinc-950 text-left transition-all"
                        },
                        "{scope}"
                    }
                }
            }
        }
    }
}

/// Selector buttons for client organizational category.
#[component]
pub fn TargetButtons(mut selected: Signal<usize>) -> Element {
    let targets = [
        "turnkey hardware commission",
        "early-stage startup",
        "r&d laboratory",
        "direct component bring-up",
    ];

    rsx! {
        div {
            class: "space-y-2",
            label { class: "block text-xs font-mono text-zinc-500 lowercase", "project profile" }
            div {
                class: "grid grid-cols-2 sm:grid-cols-4 gap-2 text-xs font-mono lowercase",
                for (idx, target) in targets.iter().enumerate() {
                    button {
                        r#type: "button",
                        onclick: move |_| selected.set(idx),
                        class: if selected() == idx {
                            "p-2.5 rounded-xs border-2 border-zinc-950 bg-zinc-100 text-zinc-950 font-bold text-center"
                        } else {
                            "p-2.5 rounded-xs border border-zinc-200 bg-zinc-50/50 text-zinc-600 hover:border-zinc-300 hover:text-zinc-950 text-center"
                        },
                        "{target}"
                    }
                }
            }
        }
    }
}
