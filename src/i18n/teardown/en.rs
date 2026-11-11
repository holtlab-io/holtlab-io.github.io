//! ## Overview
//! English localized layer specifications and telemetry strings for the anatomical teardown section.
//! Defines exact specifications, materials, and engineering achievements in English.
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
        eyebrow: "ENGINEERING SHOWCASE // CASE STUDY 01",
        title: "win95 minesweeper touch keyring",
        subtitle: "How we engineered a 60Hz touch game console into a 43mm pocket footprint.",
        exploded_eyebrow: "ANATOMICAL ARCHITECTURE",
        exploded_title: "pulled apart to the bare copper.",
        callouts: [
            ("TOP", "PA12 Front Bezel & Glass"),
            ("MID-1", "2.0\" IPS & CST816D Touch"),
            ("MID-2", "4-Layer PCBA & ESP32-C3"),
            ("BASE", "LiPo, Motor & Rear Enclosure"),
        ],
        bottom_strip: [
            "PARAMETRIC CAD // OPENCASCADE B-REP",
            "23 PAIRWISE CHECKS // ZERO-INTERFERENCE",
            "INDUSTRIAL MJF NYLON & FACTORY SMT PCBA",
        ],
    }
}

fn layer_0_en() -> TeardownLayerData {
    TeardownLayerData {
        id: 0,
        number: "01",
        title: "MCAD Enclosure",
        subtitle: "HP Multi Jet Fusion PA12 Nylon & Precision Interference-Free Fit",
        image_src: "assets/showcase/mcad_drawing.png",
        image_alt: "MCAD Enclosure Engineering Wireframe Drawing",
        verification_badge: "GATE 1 // ZERO-INTERFERENCE PASS",
        primary_spec: "43.40 × 69.20 × 13.78 mm Footprint",
        secondary_spec: "1.20 mm Nominal Wall // 0.85 mm Stepped Bezel",
        bus_or_material: "HP MJF PA12 + SLA 8001 Clear Light Pipe",
        deep_summary: "Parametrically generated via build123d OpenCascade B-Rep kernel. 23 pairwise volumetric collision checks verified against vendor COTS 3D STEP models with complete zero interference. Features a captive T-slider switch cap with ±1.50 mm stroke corridor and integrated lanyard lug.",
        bullet_1: "Monolithic rear shell with recessed corner terraces and M2 countersunk bores",
        bullet_2: "Display-hugging front bezel with 0.85 mm stepped relief for flush glass seating",
        bullet_3: "Integrated 1.50 mm LiPo battery and 1027 coin haptic vibration motor cradles",
    }
}

fn layer_1_en() -> TeardownLayerData {
    TeardownLayerData {
        id: 1,
        number: "02",
        title: "ECAD 4-Layer PCBA",
        subtitle: "Industrial 4-Layer Controlled-Impedance Stackup & 499Ω Damping RC Delay Compensation",
        image_src: "assets/showcase/ecad_pcb.png",
        image_alt: "ECAD 4-Layer Routed PCB Assembly",
        verification_badge: "GATE 2 // 0 ERC · 0 DRC VERIFIED",
        primary_spec: "40.00 × 64.60 × 1.51 mm 4-Layer Board",
        secondary_spec: "50Ω Single-Ended // 90Ω Differential USB",
        bus_or_material: "F.Cu / In1.Cu GND / In2.Cu +3V3 / B.Cu",
        deep_summary: "Designed in KiCad 9 with controlled impedance routing. Integrates ESP32-C3-WROOM-02-N4 with 4MB flash, TP4054 100mA safe LiPo charger, ME6211 3.3V 500mA LDO, and DW01A/8205A 2.45V hardware cut-off protection. Engineered with 499Ω damping RC delay compensation on high-speed lines.",
        bullet_1: "Full solid inner reference planes for continuous return paths and ultra-low noise",
        bullet_2: "Hardware USB-C 2.0 Full-Speed native D+/D- with dedicated TVS electrostatic protection",
        bullet_3: "Shared active-low reset supervisor with RC debounce and surface-mount test pads",
    }
}

fn layer_2_en() -> TeardownLayerData {
    TeardownLayerData {
        id: 2,
        number: "03",
        title: "High-Reliability Embedded Firmware Engine",
        subtitle: "esp-hal 1.2.2 + embassy 3-Task Async Actor Architecture",
        image_src: "assets/showcase/firmware_logic.png",
        image_alt: "High-Reliability Embedded Firmware Architecture Map",
        verification_badge: "BENCH // ZERO-COPY 60Hz DMA PASS",
        primary_spec: "AHB GDMA CH0 Zero-Copy Pipeline",
        secondary_spec: "0.79 ms Tile Blit // 47.04 ms Full Frame",
        bus_or_material: "Hardware SPI2 @ 26.67 MHz (80MHz/3)",
        deep_summary: "Pure #![no_std] embedded firmware engineered on esp-hal and the embassy async runtime. Eliminates heap allocations during runtime blitting with a 153.6 KB SRAM backbuffer for zero-crash stability. Concurrently manages touch input polling, deterministic Windows 95 state transitions, and asynchronous DMA blitting.",
        bullet_1: "Zero heap allocations: 153.6 KB SRAM backbuffer directly mapped to display geometry",
        bullet_2: "Falling-edge touch interrupt (GPIO6 IRQ) awakens core from 0Hz WFI power sleep",
        bullet_3: "20-second automatic LEDC PWM display dimming and 60-second deep SLPIN standby",
    }
}

fn layer_3_en() -> TeardownLayerData {
    TeardownLayerData {
        id: 3,
        number: "04",
        title: "Human Interface & Haptics",
        subtitle: "CST816D Capacitive Touch & 3.0V Regulated Resonant Feedback",
        image_src: "assets/showcase/mcad_iso.png",
        image_alt: "Human Interface and Haptic Subsystem Isometric View",
        verification_badge: "TELEMETRY // FEED-FORWARD PWM PASS",
        primary_spec: "2.0\" IPS 240×320 + CST816D Touch",
        secondary_spec: "1027 Coin Vibration Motor with AO3400A",
        bus_or_material: "I2C Fast-Mode 400kHz + 24kHz LEDC Backlight",
        deep_summary: "Seamless glass front with responsive capacitive touch and crisp mechanical feel. Touch digitizer samples at 400kHz I2C with low-jitter falling edge interrupts. Coupled with a 1027 flat coin vibration motor driven by an AO3400A NMOS with feed-forward PWM scaling, providing identical 3.0V RMS click feel on battery or USB.",
        bullet_1: "Dual aperture trackpad UI with 9×9 clamping bounds and 14px deadzone filtering",
        bullet_2: "Feed-forward PWM voltage scaling: D_max = min(1.0, 3.0 / V_sys) prevents motor overload",
        bullet_3: "High-frequency 24kHz LEDC PWM backlight dimming completely eliminates audible coil whine",
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
