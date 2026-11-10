//! ## Overview
//! Interactive selection buttons for project scope and client organization category.
//!
//! ## Search Tags
//! #contact, #selectors, #scope, #buttons, #options

use dioxus::prelude::*;

/// Selector buttons for development scope.
#[component]
pub fn ScopeButtons(mut selected: Signal<usize>) -> Element {
    rsx! {
        div {
            class: "space-y-2",
            label { class: "block text-[11px] font-semibold uppercase tracking-wider text-zinc-700", "01 // 희망 개발 범위 (PROJECT SCOPE)" }
            div {
                class: "grid grid-cols-2 gap-2",
                button {
                    r#type: "button",
                    onclick: move |_| selected.set(0),
                    class: if selected() == 0 {
                        "p-2.5 rounded-md border-2 border-zinc-900 bg-zinc-100 text-zinc-900 font-semibold text-left transition-all"
                    } else {
                        "p-2.5 rounded-md border border-zinc-200 bg-zinc-50/50 text-zinc-600 hover:border-zinc-300 hover:text-zinc-900 text-left transition-all"
                    },
                    "[+] 턴키 완주 (기구+회로+펌웨어)"
                }
                button {
                    r#type: "button",
                    onclick: move |_| selected.set(1),
                    class: if selected() == 1 {
                        "p-2.5 rounded-md border-2 border-zinc-900 bg-zinc-100 text-zinc-900 font-semibold text-left transition-all"
                    } else {
                        "p-2.5 rounded-md border border-zinc-200 bg-zinc-50/50 text-zinc-600 hover:border-zinc-300 hover:text-zinc-900 text-left transition-all"
                    },
                    "[+] 회로 & 4/6층 PCB (ECAD)"
                }
                button {
                    r#type: "button",
                    onclick: move |_| selected.set(2),
                    class: if selected() == 2 {
                        "p-2.5 rounded-md border-2 border-zinc-900 bg-zinc-100 text-zinc-900 font-semibold text-left transition-all"
                    } else {
                        "p-2.5 rounded-md border border-zinc-200 bg-zinc-50/50 text-zinc-600 hover:border-zinc-300 hover:text-zinc-900 text-left transition-all"
                    },
                    "[+] 3D 기구 & 외함 (MCAD)"
                }
                button {
                    r#type: "button",
                    onclick: move |_| selected.set(3),
                    class: if selected() == 3 {
                        "p-2.5 rounded-md border-2 border-zinc-900 bg-zinc-100 text-zinc-900 font-semibold text-left transition-all"
                    } else {
                        "p-2.5 rounded-md border border-zinc-200 bg-zinc-50/50 text-zinc-600 hover:border-zinc-300 hover:text-zinc-900 text-left transition-all"
                    },
                    "[+] 임베디드 펌웨어 개발"
                }
            }
        }
    }
}

/// Selector buttons for client organizational category.
#[component]
pub fn TargetButtons(mut selected: Signal<usize>) -> Element {
    rsx! {
        div {
            class: "space-y-2",
            label { class: "block text-[11px] font-semibold uppercase tracking-wider text-zinc-700", "02 // 고객 유형 (CLIENT TYPE)" }
            div {
                class: "grid grid-cols-2 sm:grid-cols-4 gap-2",
                button {
                    r#type: "button",
                    onclick: move |_| selected.set(0),
                    class: if selected() == 0 {
                        "p-2 rounded-md border-2 border-zinc-900 bg-zinc-100 text-zinc-900 font-semibold text-center"
                    } else {
                        "p-2 rounded-md border border-zinc-200 bg-zinc-50/50 text-zinc-600 hover:border-zinc-300 hover:text-zinc-900 text-center"
                    },
                    "예창패/청창사"
                }
                button {
                    r#type: "button",
                    onclick: move |_| selected.set(1),
                    class: if selected() == 1 {
                        "p-2 rounded-md border-2 border-zinc-900 bg-zinc-100 text-zinc-900 font-semibold text-center"
                    } else {
                        "p-2 rounded-md border border-zinc-200 bg-zinc-50/50 text-zinc-600 hover:border-zinc-300 hover:text-zinc-900 text-center"
                    },
                    "초기 스타트업"
                }
                button {
                    r#type: "button",
                    onclick: move |_| selected.set(2),
                    class: if selected() == 2 {
                        "p-2 rounded-md border-2 border-zinc-900 bg-zinc-100 text-zinc-900 font-semibold text-center"
                    } else {
                        "p-2 rounded-md border border-zinc-200 bg-zinc-50/50 text-zinc-600 hover:border-zinc-300 hover:text-zinc-900 text-center"
                    },
                    "기업 연구소"
                }
                button {
                    r#type: "button",
                    onclick: move |_| selected.set(3),
                    class: if selected() == 3 {
                        "p-2 rounded-md border-2 border-zinc-900 bg-zinc-100 text-zinc-900 font-semibold text-center"
                    } else {
                        "p-2 rounded-md border border-zinc-200 bg-zinc-50/50 text-zinc-600 hover:border-zinc-300 hover:text-zinc-900 text-center"
                    },
                    "개인/기타"
                }
            }
        }
    }
}
