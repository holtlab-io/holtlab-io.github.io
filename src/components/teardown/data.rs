//! ## Overview
//! Structured data definitions for the 4 anatomical teardown layers.
//! Contains engineering specifications, image references, tolerance telemetry, and subsystem descriptions.
//!
//! ## Search Tags
//! #teardown, #layers, #subsystems, #specs, #data
//!
//! ## Submodules
//! None. This module provides pure structured data models.

/// Subsystem teardown layer record describing physical and electrical metrics.
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct TeardownLayer {
    /// Zero-indexed layer identifier.
    pub id: usize,
    /// Numerical display string for the layer sequence.
    pub number: &'static str,
    /// Human-readable subsystem title.
    pub title: &'static str,
    /// High-level engineering subtitle describing core technologies.
    pub subtitle: &'static str,
    /// Asset path to the corresponding visual or engineering drawing.
    pub image_src: &'static str,
    /// Descriptive alt text for screen readers and accessibility.
    pub image_alt: &'static str,
    /// Deterministic gate verification status badge text.
    pub verification_badge: &'static str,
    /// Primary physical or electrical dimension metric.
    pub primary_spec: &'static str,
    /// Secondary tolerance or stackup clearance specification.
    pub secondary_spec: &'static str,
    /// Bus interface or industrial material composition.
    pub bus_or_material: &'static str,
    /// Deep paragraph detailing the engineering architecture of the layer.
    pub deep_summary: &'static str,
    /// First technical achievement bullet point.
    pub bullet_1: &'static str,
    /// Second technical achievement bullet point.
    pub bullet_2: &'static str,
    /// Third technical achievement bullet point.
    pub bullet_3: &'static str,
}

/// Catalog of all 4 architectural teardown layers comprising the flagship keyring machine.
pub const TEARDOWN_LAYERS: [TeardownLayer; 4] = [
    TeardownLayer {
        id: 0,
        number: "01",
        title: "MCAD Enclosure",
        subtitle: "HP Multi Jet Fusion PA12 Nylon & 0.00 mm³ Interference Fit",
        image_src: "assets/showcase/mcad_drawing.png",
        image_alt: "MCAD Enclosure Engineering Wireframe Drawing",
        verification_badge: "GATE 1 // 0.000 mm³ COLLISION PASS",
        primary_spec: "43.40 × 69.20 × 13.78 mm Footprint",
        secondary_spec: "1.20 mm Nominal Wall // 0.85 mm Stepped Bezel",
        bus_or_material: "HP MJF PA12 + SLA 8001 Clear Light Pipe",
        deep_summary: "Parametrically generated via build123d OpenCascade B-Rep kernel. 23 pairwise volumetric collision checks verified against vendor COTS 3D STEP models with 0.00 mm³ interference. Features a captive T-slider switch cap with ±1.50 mm stroke corridor and integrated lanyard lug.",
        bullet_1: "Monolithic rear shell with recessed corner terraces and M2 countersunk bores",
        bullet_2: "Display-hugging front bezel with 0.85 mm stepped relief for flush glass seating",
        bullet_3: "Integrated 1.50 mm LiPo battery and 1027 coin haptic vibration motor cradles",
    },
    TeardownLayer {
        id: 1,
        number: "02",
        title: "ECAD 4-Layer PCBA",
        subtitle: "JLC04161H Stackup & 499Ω Damping RC Delay Compensation",
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
    },
    TeardownLayer {
        id: 2,
        number: "03",
        title: "Bare-Metal Rust Engine",
        subtitle: "esp-hal 1.2.2 + embassy 3-Task Async Actor Architecture",
        image_src: "assets/showcase/firmware_logic.png",
        image_alt: "Bare-Metal Rust GDMA Graphics Engine Architecture Map",
        verification_badge: "BENCH // ZERO-COPY 60Hz DMA PASS",
        primary_spec: "AHB GDMA CH0 Zero-Copy Pipeline",
        secondary_spec: "0.79 ms Tile Blit // 47.04 ms Full Frame",
        bus_or_material: "Hardware SPI2 @ 26.67 MHz (80MHz/3)",
        deep_summary: "Pure #![no_std] bare-metal firmware written in Rust on esp-hal 1.2.2 and the embassy async runtime. Eliminates heap allocations during runtime blitting with a 153.6 KB SRAM backbuffer. Concurrently manages touch input polling, deterministic Windows 95 state transitions, and asynchronous DMA blitting.",
        bullet_1: "Zero heap allocations: 153.6 KB SRAM backbuffer directly mapped to display geometry",
        bullet_2: "Falling-edge touch interrupt (GPIO6 IRQ) awakens core from 0Hz WFI power sleep",
        bullet_3: "20-second automatic LEDC PWM display dimming and 60-second deep SLPIN standby",
    },
    TeardownLayer {
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
    },
];
