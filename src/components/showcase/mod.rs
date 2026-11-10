//! ## Overview
//! Engineering case study showcase module for the Win95 Minesweeper Touch Keyring reference project.
//! Provides tabbed deep-dive inspection benches for 3D MCAD, 4-layer ECAD, and bare-metal firmware.
//!
//! ## Search Tags
//! #showcase, #casestudy, #keyring, #mcad, #ecad, #firmware
//!
//! ## Submodules
//! - [`mcad`] (`mcad.rs`): MCAD Build123d B-Rep case study tab.
//! - [`ecad`] (`ecad.rs`): ECAD 4-layer PCBA case study tab.
//! - [`firmware`] (`firmware.rs`): Bare-metal firmware and GDMA case study tab.

use dioxus::prelude::*;

pub mod mcad;
pub mod ecad;
pub mod firmware;

pub use mcad::McadTab;
pub use ecad::EcadTab;
pub use firmware::FirmwareTab;

/// Flagship case study section organizing the multi-domain inspection bench.
#[component]
pub fn Showcase() -> Element {
    let active_tab = use_signal(|| 0); // 0: MCAD, 1: ECAD, 2: Firmware

    rsx! {
        section {
            id: "showcase",
            class: "py-20 bg-[#F4F4F5] relative border-b border-zinc-200 font-mono",
            div {
                class: "max-w-7xl mx-auto px-4 sm:px-6 lg:px-8",
                ShowcaseHeader {}
                ShowcaseTabs { active_tab }

                // High-Density Engineering Inspection Bench
                div {
                    class: "rounded-xl border border-zinc-200 bg-white p-5 sm:p-8 shadow-xs relative",
                    if active_tab() == 0 {
                        McadTab {}
                    } else if active_tab() == 1 {
                        EcadTab {}
                    } else {
                        FirmwareTab {}
                    }
                }
            }
        }
    }
}

/// Case study section header with title and description.
#[component]
fn ShowcaseHeader() -> Element {
    rsx! {
        div {
            class: "max-w-3xl space-y-3 mb-10",
            div {
                class: "inline-flex items-center gap-2 px-2.5 py-1 rounded-full bg-zinc-200/60 border border-zinc-300/80 text-zinc-700 text-xs tracking-wider uppercase",
                "FLAGSHIP CASE STUDY // PROVEN REFERENCE"
            }
            h2 {
                class: "text-2xl sm:text-4xl font-bold text-zinc-900 tracking-tight font-sans",
                "Win95 지뢰찾기 터치 키링"
            }
            p {
                class: "text-zinc-600 text-sm sm:text-base leading-relaxed font-sans",
                "3D 기구 모델링부터 4층 PCB 제조, ESP32-C3 베어메탈 DMA 펌웨어까지 단 한 명의 엔지니어가 완성한 "
                span { class: "text-zinc-900 font-medium", "실물 턴키 개발 레퍼런스" }
                "입니다."
            }
        }
    }
}

/// Tab selector bar for switching case study disciplines.
#[component]
fn ShowcaseTabs(mut active_tab: Signal<usize>) -> Element {
    rsx! {
        div {
            class: "flex flex-wrap gap-2 mb-8 text-xs",
            button {
                r#type: "button",
                onclick: move |_| active_tab.set(0),
                class: if active_tab() == 0 {
                    "px-4 py-2 rounded-md bg-zinc-900 text-white font-bold border border-zinc-900 shadow-xs transition-all tactile-button"
                } else {
                    "px-4 py-2 rounded-md bg-white text-zinc-600 hover:text-zinc-900 border border-zinc-300 hover:border-zinc-400 shadow-2xs transition-all tactile-button"
                },
                "01 // MCAD B-REP 외함"
            }
            button {
                r#type: "button",
                onclick: move |_| active_tab.set(1),
                class: if active_tab() == 1 {
                    "px-4 py-2 rounded-md bg-zinc-900 text-white font-bold border border-zinc-900 shadow-xs transition-all tactile-button"
                } else {
                    "px-4 py-2 rounded-md bg-white text-zinc-600 hover:text-zinc-900 border border-zinc-300 hover:border-zinc-400 shadow-2xs transition-all tactile-button"
                },
                "02 // ECAD 4층 PCBA"
            }
            button {
                r#type: "button",
                onclick: move |_| active_tab.set(2),
                class: if active_tab() == 2 {
                    "px-4 py-2 rounded-md bg-zinc-900 text-white font-bold border border-zinc-900 shadow-xs transition-all tactile-button"
                } else {
                    "px-4 py-2 rounded-md bg-white text-zinc-600 hover:text-zinc-900 border border-zinc-300 hover:border-zinc-400 shadow-2xs transition-all tactile-button"
                },
                "03 // 펌웨어 60Hz GDMA"
            }
        }
    }
}
