//! ## Overview
//! Firmware case study tab displaying 26.67MHz SPI bus stabilization, 60Hz Zero-Copy GDMA, and low-power power management.
//!
//! ## Search Tags
//! #showcase, #firmware, #gdma, #spi, #lowpower, #baremetal

use dioxus::prelude::*;

/// Firmware technical dossier view presenting timing parameters and mechanical blueprint.
#[component]
pub fn FirmwareTab() -> Element {
    rsx! {
        div {
            class: "grid grid-cols-1 lg:grid-cols-12 gap-8 items-center",
            FirmwareSpecs {}
            FirmwareVisual {}
        }
    }
}

/// Firmware technical specifications and timing parameter matrix.
#[component]
fn FirmwareSpecs() -> Element {
    rsx! {
        div {
            class: "lg:col-span-7 space-y-6",
            div {
                class: "flex flex-wrap items-center gap-2 text-xs font-mono",
                span { class: "px-2 py-0.5 rounded bg-zinc-100 text-zinc-900 border border-zinc-200 font-bold", "FIRMWARE // ZERO-COPY GDMA" }
                span { class: "text-zinc-300", "•" }
                span { class: "text-zinc-700 font-semibold", "CLK: 26.67 MHz (80MHz / 3)" }
                span { class: "text-zinc-300", "•" }
                span { class: "text-zinc-500", "LATENCY: 0.79ms / TILE" }
            }
            h3 {
                class: "text-xl sm:text-2xl font-bold text-zinc-900 font-sans",
                "26.67MHz SPI & Zero-Copy AHB GDMA 초고속 구동"
            }
            p {
                class: "text-zinc-600 text-xs sm:text-sm font-sans leading-relaxed",
                "캔 내부 499Ω 직렬 댐핑 저항 한계를 극복하고 26.67MHz 고속 SPI 버스를 안정화했습니다. 153.6KB SRAM 백버퍼와 AHB GDMA를 통해 CPU 개입 없이 60Hz 고속 부분 갱신을 실현했습니다."
            }
            div {
                class: "border border-zinc-200 rounded-lg divide-y divide-zinc-200 text-xs font-mono bg-zinc-50/50",
                div {
                    class: "grid grid-cols-2 p-2.5 sm:px-4",
                    span { class: "text-zinc-500 uppercase", "SPI 버스 클럭" },
                    span { class: "text-zinc-900 font-semibold", "26.67 MHz (80MHz / 3 Divider)" }
                }
                div {
                    class: "grid grid-cols-2 p-2.5 sm:px-4",
                    span { class: "text-zinc-500 uppercase", "부분 갱신 지연" },
                    span { class: "text-zinc-900 font-semibold", "타일당 0.79ms (Zero-Copy GDMA)" }
                }
                div {
                    class: "grid grid-cols-2 p-2.5 sm:px-4",
                    span { class: "text-zinc-500 uppercase", "터치 인터럽트" },
                    span { class: "text-zinc-900 font-semibold", "GPIO FallingEdge 하드웨어 IRQ" }
                }
                div {
                    class: "grid grid-cols-2 p-2.5 sm:px-4",
                    span { class: "text-zinc-500 uppercase", "전원 관리 아키텍처" },
                    span { class: "text-zinc-900 font-semibold", "3.30V ADC 모니터링 & 자동 딥슬립" }
                }
            }
        }
    }
}

/// Firmware blueprint schematic and timing diagram viewport.
#[component]
fn FirmwareVisual() -> Element {
    rsx! {
        div {
            class: "lg:col-span-5 relative rounded-xl border border-zinc-200 aspect-square sm:aspect-[4/3] flex items-center justify-center p-3 overflow-hidden shadow-2xs bg-zinc-100",
            div {
                class: "w-full h-full rounded-lg bg-white flex items-center justify-center relative p-2 overflow-hidden border border-zinc-200",
                img {
                    src: "assets/showcase/mcad_drawing.png",
                    alt: "Firmware Timing Blueprint",
                    class: "w-full h-full object-contain"
                }
                div {
                    class: "absolute bottom-2 right-2 px-2 py-0.5 rounded bg-white/90 border border-zinc-200 text-[10px] text-zinc-700 font-mono shadow-2xs",
                    "TIMING DIAGRAM & BLUEPRINT"
                }
            }
        }
    }
}
