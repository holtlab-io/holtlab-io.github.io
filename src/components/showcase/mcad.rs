//! ## Overview
//! MCAD case study tab displaying Build123d B-Rep 3D CAD modeling, 0.00 mm³ collision verification, and nylon MJF enclosure specs.
//!
//! ## Search Tags
//! #showcase, #mcad, #cad, #brep, #enclosure, #mjf

use dioxus::prelude::*;

/// MCAD technical dossier view presenting enclosure engineering metrics and exploded assembly.
#[component]
pub fn McadTab() -> Element {
    rsx! {
        div {
            class: "grid grid-cols-1 lg:grid-cols-12 gap-8 items-center",
            McadSpecs {}
            McadVisual {}
        }
    }
}

/// MCAD technical specifications and parameter matrix.
#[component]
fn McadSpecs() -> Element {
    rsx! {
        div {
            class: "lg:col-span-7 space-y-6",
            div {
                class: "flex flex-wrap items-center gap-2 text-xs font-mono",
                span { class: "px-2 py-0.5 rounded bg-zinc-100 text-zinc-900 border border-zinc-200 font-bold", "MCAD // BUILD123D B-REP" }
                span { class: "text-zinc-300", "•" }
                span { class: "text-zinc-700 font-semibold", "COLLISION: 0.00 mm³" }
                span { class: "text-zinc-300", "•" }
                span { class: "text-zinc-500", "TOLERANCE: ±0.05mm" }
            }
            h3 {
                class: "text-xl sm:text-2xl font-bold text-zinc-900 font-sans",
                "0.00mm 정밀 공차의 산업용 MJF 나일론 외함"
            }
            p {
                class: "text-zinc-600 text-xs sm:text-sm font-sans leading-relaxed",
                "ESP32-C3 모듈, 2.0인치 터치 LCD, 400mAh 리튬폴리머 배터리, 1027 진동 모터가 13.78mm 두께 안에 0.01mm의 간섭 없이 정밀 결합되도록 오픈캐스케이드 B-Rep 솔리드 커널 기반으로 설계되었습니다."
            }
            div {
                class: "border border-zinc-200 rounded-lg divide-y divide-zinc-200 text-xs font-mono bg-zinc-50/50",
                div {
                    class: "grid grid-cols-2 p-2.5 sm:px-4",
                    span { class: "text-zinc-500 uppercase", "외형 치수 (W×H×D)" },
                    span { class: "text-zinc-900 font-semibold", "43.40 × 69.20 × 13.78 mm" }
                }
                div {
                    class: "grid grid-cols-2 p-2.5 sm:px-4",
                    span { class: "text-zinc-500 uppercase", "제작 공법 & 소재" },
                    span { class: "text-zinc-900 font-semibold", "HP MJF Nylon PA12 (블랙)" }
                }
                div {
                    class: "grid grid-cols-2 p-2.5 sm:px-4",
                    span { class: "text-zinc-500 uppercase", "파스너 체결" },
                    span { class: "text-zinc-900 font-semibold", "M1.4 4점 나사산 체결 보스" }
                }
                div {
                    class: "grid grid-cols-2 p-2.5 sm:px-4",
                    span { class: "text-zinc-500 uppercase", "디스플레이 마운트" },
                    span { class: "text-zinc-900 font-semibold", "듀얼 개구부 일체형 베젤 립" }
                }
            }
        }
    }
}

/// MCAD exploded assembly render viewport.
#[component]
fn McadVisual() -> Element {
    rsx! {
        div {
            class: "lg:col-span-5 relative rounded-xl border border-zinc-200 aspect-square sm:aspect-[4/3] flex items-center justify-center p-3 overflow-hidden shadow-2xs bg-zinc-100",
            div {
                class: "w-full h-full rounded-lg bg-[#141820] flex items-center justify-center relative p-2 overflow-hidden",
                img {
                    src: "assets/showcase/mcad_exploded.png",
                    alt: "MCAD Exploded Assembly View",
                    class: "w-full h-full object-contain"
                }
                div {
                    class: "absolute bottom-2 right-2 px-2 py-0.5 rounded bg-black/60 border border-zinc-700 text-[10px] text-zinc-300 font-mono",
                    "EXPLODED ASSEMBLY // STEP VERIFIED"
                }
            }
        }
    }
}
