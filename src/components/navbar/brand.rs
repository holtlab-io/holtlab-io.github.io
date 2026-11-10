//! ## Overview
//! Brand logo lockup, technical Korean micro-manifesto, and geometric studio mark components.
//! Encapsulates visual identity anchors adhering to the Teenage Engineering design language.
//!
//! ## Search Tags
//! #brand, #logo, #manifesto, #glyph, #identity

use dioxus::prelude::*;

const MANIFESTO: &str = "홀트랩은 미래의 기구와\n하드웨어를 직접 직조\n하는 엔지니어링 랩.\n우리의 단일 미션은\n0.00 mm³ 정밀 공학\n과 베어메탈 펌웨어로\n온전한 제품을 빚는것.\n공학과 미학의 조화를\n경험해 보십시오.";

/// Brand logo typography with lowercase stacked lockup.
#[component]
pub fn NavBrand() -> Element {
    rsx! {
        a {
            href: "#",
            class: "flex flex-col select-none group shrink-0 pt-0.5",
            span {
                class: "text-xl sm:text-[22px] font-normal font-sans tracking-tight leading-[0.92] lowercase text-zinc-950 group-hover:text-zinc-600 transition-colors",
                "holt"
            }
            span {
                class: "text-xl sm:text-[22px] font-normal font-sans tracking-tight leading-[0.92] lowercase text-zinc-950 group-hover:text-zinc-600 transition-colors",
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
            class: "hidden xl:block text-[8px] font-mono leading-[11px] text-zinc-400 select-none tracking-wider whitespace-pre",
            "{MANIFESTO}"
        }
    }
}

/// Geometric Bauhaus studio glyph lockup.
#[component]
pub fn NavStudioMark() -> Element {
    rsx! {
        div {
            class: "shrink-0 flex items-start select-none relative pt-0.5",
            svg {
                class: "w-12 h-12 fill-zinc-950",
                view_box: "0 0 48 48",
                rect { x: "4", y: "6", width: "8", height: "36" }
                rect { x: "12", y: "20", width: "12", height: "8" }
                rect { x: "24", y: "6", width: "8", height: "36" }
                circle { cx: "40", cy: "24", r: "5.5" }
            }
            span {
                class: "text-[9px] font-sans font-bold text-zinc-950 absolute -top-1 -right-2 select-none",
                "®"
            }
        }
    }
}
