//! ## Overview
//! High-density Swiss engineering specification matrix table.
//! Presents exacting tolerances, dimensions, materials, silicon specs, and firmware telemetry without marketing fluff.
//!
//! ## Search Tags
//! #specs, #matrix, #tolerances, #stackup, #swiss-engineering

use dioxus::prelude::*;

/// Complete engineering specification row record.
struct SpecRow {
    key: &'static str,
    val: &'static str,
    note: &'static str,
}

const SPEC_ROWS: [SpecRow; 10] = [
    SpecRow {
        key: "dimensions & mass",
        val: "43.40 × 69.20 × 13.78 mm // 38.2 g",
        note: "fully assembled chassis including lipo battery and 2.0\" display",
    },
    SpecRow {
        key: "chassis material",
        val: "HP Multi Jet Fusion (MJF) PA12",
        note: "glass bead vapor honed finish, 1.20 mm nominal wall thickness",
    },
    SpecRow {
        key: "cad kernel & drc",
        val: "build123d 0.7.0 + OpenCascade B-Rep",
        note: "0.00 mm³ volumetric collision check against COTS 3D STEP models",
    },
    SpecRow {
        key: "pcba stackup",
        val: "JLCPCB 4-Layer JLC04161H-7628 (1.0 mm)",
        note: "controlled impedance: 50Ω single-ended, 90Ω differential USB",
    },
    SpecRow {
        key: "silicon processor",
        val: "Espressif ESP32-C3 RISC-V 32-bit @ 160MHz",
        note: "ESP32-C3-WROOM-02-N4 module with integrated 4MB SPI flash",
    },
    SpecRow {
        key: "display & bus",
        val: "2.0\" ST7789T3 IPS (240×320 RGB)",
        note: "4-wire SPI2 @ 26.67MHz (80MHz/3) with GDMA zero-copy transfer",
    },
    SpecRow {
        key: "touch digitizer",
        val: "Hynitron CST816D Capacitive Single-Touch",
        note: "I2C 400kHz Fast-Mode with GPIO6 falling-edge low-jitter interrupt",
    },
    SpecRow {
        key: "battery & power",
        val: "TW301525 150mAh LiPo + DW01A/8205A PCM",
        note: "2.45V hardware cut-off, GPIO0 ADC telemetry, SW1 manual switch",
    },
    SpecRow {
        key: "firmware runtime",
        val: "Rust esp-hal 1.2.2 + embassy async framework",
        note: "3 concurrent async tasks, 153.6 KB SRAM backbuffer blitting @ 60Hz",
    },
    SpecRow {
        key: "factory dispatch",
        val: "JLCPCB PCBA & JLC3DP MJF Direct Dispatch",
        note: "zero-rework bring-up over remote Tailscale bench environment",
    },
];

/// Primary engineering specification table component rendered at `#specs`.
#[component]
pub fn SpecsMatrix() -> Element {
    rsx! {
        section {
            id: "specs",
            class: "bg-white py-16 sm:py-24 border-t border-zinc-200 font-sans",
            div {
                class: "max-w-[1600px] mx-auto px-4 sm:px-8",

                // Header
                div {
                    class: "flex flex-col md:flex-row md:items-end justify-between border-b border-zinc-200 pb-6 mb-8 gap-4",
                    div {
                        class: "flex flex-col",
                        span { class: "text-xs font-mono text-zinc-400 lowercase", "engineering reference" }
                        h2 { class: "text-2xl sm:text-3xl font-bold lowercase text-zinc-950 tracking-tight mt-1", "technical specifications" }
                    }
                    div {
                        class: "text-xs font-mono text-zinc-500 lowercase",
                        "win95 minesweeper touch keyring // verified 0.00 mm³"
                    }
                }

                // Swiss 2-Column Key-Value Grid
                div {
                    class: "border border-zinc-200 divide-y divide-zinc-200 text-xs sm:text-sm font-mono lowercase",
                    for row in SPEC_ROWS.iter() {
                        div {
                            class: "grid grid-cols-1 md:grid-cols-3 lg:grid-cols-4 p-4 sm:p-5 hover:bg-zinc-50/50 transition-colors gap-2",
                            div {
                                class: "text-zinc-500 font-medium",
                                "{row.key}"
                            }
                            div {
                                class: "md:col-span-2 lg:col-span-2 text-zinc-950 font-bold",
                                "{row.val}"
                            }
                            div {
                                class: "text-zinc-400 font-sans text-xs pt-0.5",
                                "{row.note}"
                            }
                        }
                    }
                }
            }
        }
    }
}
