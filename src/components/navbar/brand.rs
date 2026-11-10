//! ## Overview
//! Brand logo lockup, technical Korean micro-manifesto, and geometric studio mark components.
//! Encapsulates visual identity anchors adhering to the Teenage Engineering design language.
//!
//! ## Search Tags
//! #brand, #logo, #manifesto, #glyph, #identity

use dioxus::prelude::*;

/// Brand logo typography with lowercase stacked lockup.
#[component]
pub fn NavBrand() -> Element {
    rsx! {
        a {
            href: "#",
            class: "flex flex-col select-none group shrink-0",
            span {
                class: "text-2xl font-bold font-sans tracking-tighter leading-none lowercase text-zinc-950 group-hover:text-zinc-600 transition-colors",
                "holt"
            }
            span {
                class: "text-2xl font-bold font-sans tracking-tighter leading-none lowercase text-zinc-950 group-hover:text-zinc-600 transition-colors",
                "lab"
            }
        }
    }
}

/// Micro Korean technical manifesto block replicating the Japanese text element from Teenage Engineering.
#[component]
pub fn NavKoreanManifesto() -> Element {
    rsx! {
        div {
            class: "hidden xl:block max-w-[170px] text-[9px] font-mono leading-[13px] text-zinc-400 select-none",
            "홀트랩은 물리적 정밀 공학과 베어메탈 펌웨어를 직접 직조하는 독립 하드웨어 랩입니다. 3D 기구와 4층 회로, 펌웨어를 단일 비전으로 직접 설계합니다."
        }
    }
}

/// Geometric Bauhaus studio glyph lockup.
#[component]
pub fn NavStudioMark() -> Element {
    rsx! {
        div {
            class: "shrink-0 flex items-center select-none",
            svg {
                class: "w-9 h-9 fill-zinc-950",
                view_box: "0 0 40 40",
                rect { x: "4", y: "8", width: "6", height: "24", rx: "1" }
                rect { x: "10", y: "17", width: "10", height: "6" }
                rect { x: "20", y: "8", width: "6", height: "24", rx: "1" }
                circle { cx: "32", cy: "20", r: "5" }
            }
        }
    }
}
