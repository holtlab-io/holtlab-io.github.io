//! ## Overview
//! ECAD case study tab displaying KiCad 9 4-layer PCB design, impedance control, RF antenna matching, and battery PCM circuitry.
//!
//! ## Search Tags
//! #showcase, #ecad, #pcba, #kicad, #circuit, #hardware

use dioxus::prelude::*;

/// ECAD technical dossier view presenting circuit architecture and isometric PCBA render.
#[component]
pub fn EcadTab() -> Element {
    rsx! {
        div {
            class: "grid grid-cols-1 lg:grid-cols-12 gap-8 items-center",
            EcadSpecs {}
            EcadVisual {}
        }
    }
}

/// ECAD technical specifications and parameter matrix.
#[component]
fn EcadSpecs() -> Element {
    rsx! {
        div {
            class: "lg:col-span-7 space-y-6",
            div {
                class: "flex flex-wrap items-center gap-2 text-xs font-mono",
                span { class: "px-2 py-0.5 rounded bg-zinc-100 text-zinc-900 border border-zinc-200 font-bold", "ECAD // KICAD 9 4-LAYER" }
                span { class: "text-zinc-300", "•" }
                span { class: "text-zinc-700 font-semibold", "STACKUP: JLC04161H-7628" }
                span { class: "text-zinc-300", "•" }
                span { class: "text-zinc-500", "IMPEDANCE: 50Ω RF / 90Ω USB" }
            }
            h3 {
                class: "text-xl sm:text-2xl font-bold text-zinc-900 font-sans",
                "4층 고밀도 PCB & 하드웨어 PCM 보호 회로"
            }
            p {
                class: "text-zinc-600 text-xs sm:text-sm font-sans leading-relaxed",
                "JLCPCB 4층 스택업 기반으로 ESP32-C3의 2.4GHz RF 안테나 매칭, 26.67MHz SPI 고속 버스 배선, DW01A+8205A 하드웨어 PCM 배터리 보호 회로를 40×64.6mm 초소형 보드에 직접 설계 및 실장했습니다."
            }
            div {
                class: "border border-zinc-200 rounded-lg divide-y divide-zinc-200 text-xs font-mono bg-zinc-50/50",
                div {
                    class: "grid grid-cols-2 p-2.5 sm:px-4",
                    span { class: "text-zinc-500 uppercase", "PCB 크기 & 층수" },
                    span { class: "text-zinc-900 font-semibold", "40.0 × 64.6 mm (4-Layer 1.6mm)" }
                }
                div {
                    class: "grid grid-cols-2 p-2.5 sm:px-4",
                    span { class: "text-zinc-500 uppercase", "메인 프로세서" },
                    span { class: "text-zinc-900 font-semibold", "ESP32-C3-WROOM-02-N4 (160MHz)" }
                }
                div {
                    class: "grid grid-cols-2 p-2.5 sm:px-4",
                    span { class: "text-zinc-500 uppercase", "디스플레이 & 터치 IC" },
                    span { class: "text-zinc-900 font-semibold", "ST7789T3 (SPI) + CST816D (I2C)" }
                }
                div {
                    class: "grid grid-cols-2 p-2.5 sm:px-4",
                    span { class: "text-zinc-500 uppercase", "배터리 PCM 컷오프" },
                    span { class: "text-zinc-900 font-semibold", "DW01A + 8205A (2.45V 컷오프)" }
                }
            }
        }
    }
}

/// ECAD isometric PCB render viewport.
#[component]
fn EcadVisual() -> Element {
    rsx! {
        div {
            class: "lg:col-span-5 relative rounded-xl border border-zinc-200 aspect-square sm:aspect-[4/3] flex items-center justify-center p-3 overflow-hidden shadow-2xs bg-zinc-100",
            div {
                class: "w-full h-full rounded-lg bg-white flex items-center justify-center relative p-3 overflow-hidden border border-zinc-200 shadow-2xs",
                img {
                    src: "assets/showcase/pcb_iso.png",
                    alt: "ECAD 4-Layer PCB Render",
                    class: "w-full h-full object-contain"
                }
                div {
                    class: "absolute bottom-2.5 right-2.5 px-2.5 py-1 rounded bg-white/95 backdrop-blur-xs border border-zinc-200 text-[10px] text-zinc-700 font-mono font-medium tracking-wider shadow-2xs",
                    "4-LAYER ISOMETRIC PCBA"
                }
            }
        }
    }
}
