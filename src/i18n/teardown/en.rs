//! ## Overview
//! English localized layer specifications and telemetry strings for the anatomical teardown section.
//! Defines exact specifications, materials, and engineering details in English.
//!
//! ## Search Tags
//! #i18n, #teardown, #english, #telemetry, #specifications
//!
//! ## Submodules
//! None. This module provides atomic localized English records.

use super::{TeardownLayerData, TeardownSectionStrings};

/// Returns English section-level headings and exploded banner strings.
pub fn get_section_strings() -> TeardownSectionStrings {
    TeardownSectionStrings {
        eyebrow: "SUBSYSTEM TEARDOWN · LAYER ANALYSIS",
        title: "4-Layer Hardware & Firmware Breakdown",
        subtitle: "Detailed architecture across enclosure, 4-layer PCBA, firmware, and interface.",
        exploded_eyebrow: "INTERNAL ARCHITECTURE",
        exploded_title: "Four Architectural Layers, Inside and Out.",
        callouts: [
            ("TOP", "PA12 Front Bezel & Glass"),
            ("MID-1", "2.0\" IPS & CST816D Touch"),
            ("MID-2", "4-Layer PCBA & ESP32-C3"),
            ("BASE", "LiPo, Motor & Rear Shell"),
        ],
        bottom_strip: [
            "PARAMETRIC 3D B-REP SOLID MODELING",
            "3D STEP COMPONENT CLEARANCE VERIFIED",
            "HP MJF PA12 NYLON & 4-LAYER SMT PCBA",
        ],
    }
}

fn layer_0_en() -> TeardownLayerData {
    TeardownLayerData {
        id: 0,
        number: "01",
        title: "MCAD Enclosure",
        subtitle: "HP Multi Jet Fusion PA12 Nylon & Stepped Bezel Assembly",
        image_src: "/assets/showcase/mcad_drawing.png",
        image_alt: "MCAD Enclosure Engineering Wireframe Drawing",
        verification_badge: "3D FIT & CLEARANCE VERIFIED",
        primary_spec: "43.40 × 69.20 × 13.78 mm Footprint",
        secondary_spec: "1.20 mm Nominal Wall · 0.85 mm Stepped Bezel",
        bus_or_material: "HP MJF PA12 + SLA Clear Light Pipe",
        deep_summary: "Modeled with parametric 3D B-Rep solid geometry referenced directly against vendor 3D STEP component models to verify internal clearances and assembly fit. Features a captive T-slider power switch cap with ±1.50 mm travel and an integrated lanyard lug.",
        bullet_1: "Monolithic rear shell with recessed corner terraces and M2 countersunk bores",
        bullet_2: "Display-hugging front bezel with 0.85 mm stepped relief for flush glass seating",
        bullet_3: "Dedicated internal cradles for the LiPo cell and 1027 coin haptic motor",
    }
}

fn layer_1_en() -> TeardownLayerData {
    TeardownLayerData {
        id: 1,
        number: "02",
        title: "ECAD 4-Layer PCBA",
        subtitle: "4-Layer Controlled-Impedance Stackup & High-Speed Signal Integrity",
        image_src: "/assets/showcase/ecad_pcb.png",
        image_alt: "ECAD 4-Layer Routed PCB Assembly",
        verification_badge: "0 ERC · 0 DRC VERIFIED",
        primary_spec: "40.00 × 64.60 × 1.00 mm 4-Layer Board",
        secondary_spec: "50Ω Single-Ended · 90Ω Differential USB",
        bus_or_material: "F.Cu / In1.Cu GND / In2.Cu +3V3 / B.Cu",
        deep_summary: "Custom 4-layer controlled-impedance PCBA integrating an ESP32-C3 wireless MCU module (4MB flash), TP4054 100mA LiPo charger, ME6211 3.3V LDO, and DW01A/8205A 2.45V hardware cut-off protection — with SPI pin routing optimized around internal 499Ω module damping characteristics.",
        bullet_1: "Solid inner ground and power reference planes for low-noise return paths",
        bullet_2: "USB-C 2.0 Full-Speed native D+/D- with dedicated TVS electrostatic protection",
        bullet_3: "Active-low reset supervisor with RC debounce and accessible bench test pads",
    }
}

fn layer_2_en() -> TeardownLayerData {
    TeardownLayerData {
        id: 2,
        number: "03",
        title: "Embedded Firmware",
        subtitle: "3-Task Bare-Metal Async Architecture & AHB GDMA Graphics",
        image_src: "/assets/showcase/firmware_logic.png",
        image_alt: "Embedded Firmware Architecture Map",
        verification_badge: "ZERO-COPY 60Hz DMA VERIFIED",
        primary_spec: "AHB GDMA CH0 Zero-Copy Pipeline",
        secondary_spec: "0.79 ms Tile Blit · 47.04 ms Full Frame",
        bus_or_material: "Hardware SPI2 @ 26.67 MHz (80MHz/3)",
        deep_summary: "Engineered on a deterministic bare-metal async multi-tasking architecture with zero dynamic heap allocations for long-term runtime stability. Pairs a static 153.6 KB SRAM framebuffer with AHB GDMA Channel 0 to concurrently handle touch interrupts, state transitions, and 60Hz display blitting.",
        bullet_1: "Zero heap allocations: static 153.6 KB SRAM backbuffer mapped to display geometry",
        bullet_2: "Falling-edge touch interrupt (GPIO6 IRQ) wakes the core from 0Hz WFI sleep",
        bullet_3: "20-second automatic LEDC PWM display dimming and 60-second deep SLPIN standby",
    }
}

fn layer_3_en() -> TeardownLayerData {
    TeardownLayerData {
        id: 3,
        number: "04",
        title: "Interface & Haptics",
        subtitle: "CST816D Capacitive Touch & 3.0V Regulated Resonant Feedback",
        image_src: "/assets/showcase/mcad_iso.png",
        image_alt: "Human Interface and Haptic Subsystem Isometric View",
        verification_badge: "FEED-FORWARD PWM VERIFIED",
        primary_spec: "2.0\" IPS 240×320 + CST816D Touch",
        secondary_spec: "1027 Coin Vibration Motor + AO3400A NMOS",
        bus_or_material: "I2C Fast-Mode 400kHz + 24kHz LEDC Backlight",
        deep_summary: "Flush glass front combining responsive capacitive touch with crisp tactile feedback. The touch controller samples over 400kHz I2C with low-jitter interrupts, paired with an AO3400A-driven coin vibration motor using feed-forward PWM scaling for consistent 3.0V RMS click feel across battery levels.",
        bullet_1: "Dual-aperture trackpad UI with 9×9 clamping bounds and 14px deadzone filtering",
        bullet_2: "Feed-forward PWM voltage scaling: D_max = min(1.0, 3.0 / V_sys) prevents overload",
        bullet_3: "High-frequency 24kHz LEDC PWM backlight dimming eliminates audible coil whine",
    }
}

/// Returns the 4 English teardown layers.
pub fn get_layers() -> [TeardownLayerData; 4] {
    [
        layer_0_en(),
        layer_1_en(),
        layer_2_en(),
        layer_3_en(),
    ]
}
