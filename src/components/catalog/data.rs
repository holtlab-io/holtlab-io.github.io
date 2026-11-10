//! ## Overview
//! Hardware catalog data definitions and technical specifications catalog for Holt Lab reference units.
//! Stores dimensions, silicon specs, layer stackups, CAD tolerances, and static render asset paths.
//!
//! ## Search Tags
//! #data, #hardware, #units, #specs, #records

/// Complete technical specification record for an individual hardware unit.
#[derive(Clone, PartialEq, Debug)]
pub struct HardwareUnit {
    /// Unique identifier for filtering and anchors.
    pub id: &'static str,
    /// Product name in lowercase sans-serif.
    pub title: &'static str,
    /// Short technical summary displayed directly beneath the card.
    pub subtitle: &'static str,
    /// Functional discipline category for filtering.
    pub category: &'static str,
    /// Static image path for the hardware render.
    pub image: &'static str,
    /// Physical dimensions and envelope.
    pub dimensions: &'static str,
    /// Primary silicon processor and compute bus.
    pub silicon: &'static str,
    /// Display and digitizer interface.
    pub display: &'static str,
    /// Enclosure fabrication process and finish.
    pub material: &'static str,
    /// PCBA layer count, controlled impedance, and surface finish.
    pub stackup: &'static str,
    /// Bare-metal firmware architecture.
    pub firmware: &'static str,
    /// Quantitative CAD DRC tolerance volume.
    pub tolerance: &'static str,
}

/// Static array of all verified hardware reference products in the Holt Lab catalog.
pub const HARDWARE_UNITS: [HardwareUnit; 8] = [
    HardwareUnit {
        id: "minesweeper-keyring",
        title: "win95 minesweeper touch keyring",
        subtitle: "esp32-c3 · 2.0\" touch ips · hp mjf pa12 · 4-layer pcba",
        category: "keyrings",
        image: "assets/showcase/mcad_exploded.png",
        dimensions: "43.40 × 69.20 × 13.78 mm // 38.2 g",
        silicon: "ESP32-C3 RISC-V 32-bit Single-Core @ 160MHz",
        display: "2.0\" ST7789T3 IPS (240×320) + CST816D Touch @ I2C 400kHz",
        material: "HP Multi Jet Fusion PA12, Glass Bead Finish",
        stackup: "JLCPCB 4-Layer 7628 Controlled Impedance, ENIG Gold",
        firmware: "Rust esp-hal 1.2.2 + embassy async, DMA_CH0 60Hz",
        tolerance: "0.00 mm³ collision volume verified",
    },
    HardwareUnit {
        id: "pcba-rev-a",
        title: "esp32-c3 4-layer pcba (rev a)",
        subtitle: "controlled impedance 7628 · jigless pogo bringup · enig gold",
        category: "pcba",
        image: "assets/showcase/pcb_top.png",
        dimensions: "40.00 × 64.60 × 1.00 mm // 4-layer",
        silicon: "ESP32-C3-WROOM-02-N4 + CST816D + ST7789T3",
        display: "4-wire 8-bit SPI2 @ 26.67MHz (80MHz/3) GDMA",
        material: "FR-4 High TG 150-160°C, Matte Black Soldermask",
        stackup: "JLC04161H-7628: Top Cu / Prepreg / In1 Cu / Core / In2 / Bot",
        firmware: "AHB Zero-Copy GDMA 2D partial rect blitting",
        tolerance: "0.20 mm track / 0.20 mm clearance DRC pass",
    },
    HardwareUnit {
        id: "mjf-chassis",
        title: "mjf pa12 monolithic enclosure & bezel",
        subtitle: "43.40 × 69.20 × 13.78 mm · snap-fit latch · vapor honed",
        category: "enclosures",
        image: "assets/showcase/mcad_iso_front.png",
        dimensions: "43.40 × 69.20 × 13.78 mm",
        silicon: "Internal battery compartment for TW301525 LiPo",
        display: "Flush 0.20 mm step bezel with bonded glass display",
        material: "HP 3D High Reusability PA12 Polyamide",
        stackup: "1.20 mm nominal wall thickness, 0.15 mm snap deflection",
        firmware: "M1 vibration motor bed with silicone damping",
        tolerance: "0.00 mm³ B-Rep CAD collision volume",
    },
    HardwareUnit {
        id: "cad-section",
        title: "gate 2 cross-section clearance audit",
        subtitle: "0.00 mm³ collision volume · 0.20 mm display shim · step verified",
        category: "enclosures",
        image: "assets/showcase/mcad_section.png",
        dimensions: "B-Rep solid boolean intersection analysis",
        silicon: "COTS 3D STEP models integrated before CAD freeze",
        display: "Exact LCD flex cable bend radius clearance",
        material: "build123d 0.7.0 OpenCascade Python B-Rep kernel",
        stackup: "Integrated PCB mounting standoffs and snap tabs",
        firmware: "Power switch SW1 actuator sliding tolerance 0.10 mm",
        tolerance: "0.00 mm³ volumetric collision check Gate 1 & 2 PASS",
    },
    HardwareUnit {
        id: "firmware-runtime",
        title: "bare-metal rust esp-hal & embassy runtime",
        subtitle: "26.67 mhz gdma spi · 60 hz zero-copy ahb · 153.6 kb sram",
        category: "firmware",
        image: "assets/showcase/pcb_iso.png",
        dimensions: "3 concurrent async tasks, 0 OS latency overhead",
        silicon: "Direct ESP32-C3 register & GPIO Matrix routing",
        display: "ST7789T3 RDDST=0x80532400 hardware readback verified",
        material: "Target binary: riscv32imc-unknown-none-elf",
        stackup: "153.6 KB SRAM backbuffer allocation for frame diffing",
        firmware: "GPIO6 touch falling-edge IRQ wake + 20s backlight dim",
        tolerance: "0 dropped frames during 60Hz trackpad drag",
    },
    HardwareUnit {
        id: "mechanical-drawing",
        title: "iso 2768-m mechanical drafting & gd&t",
        subtitle: "jlc3dp / jlcpcb automated direct dispatch · 0.05 mm precision",
        category: "enclosures",
        image: "assets/showcase/mcad_drawing.png",
        dimensions: "Standard engineering drafting with section views A-A & B-B",
        silicon: "Designed for HP MJF PA12 industrial sintering",
        display: "Clear aperture: 31.00 × 41.50 mm viewing area",
        material: "ISO 2768-m general linear and angular tolerances",
        stackup: "Direct dispatch to JLC3DP automated production line",
        firmware: "Keyring loop reinforcement rib 2.0 mm diameter",
        tolerance: "±0.05 mm nominal fabrication tolerance",
    },
    HardwareUnit {
        id: "pcba-bottom",
        title: "4-layer pcba bottom routing & test pads",
        subtitle: "jigless pogo-pin programming pads · unbroken ground reference",
        category: "pcba",
        image: "assets/showcase/pcb_bottom.png",
        dimensions: "40.00 × 64.60 mm continuous copper reference plane",
        silicon: "Pogo pads: GPIO0, U0TXD, U0RXD, EN, GND, 3V3",
        display: "Battery solder pads with thermal relief ties",
        material: "ENIG Electroless Nickel Immersion Gold (2u\" gold)",
        stackup: "Layer 4 copper pour stitched with ground via grid",
        firmware: "Zero-rework boot strapping and flash verification",
        tolerance: "100% netlist electrical DRC clearance pass",
    },
    HardwareUnit {
        id: "touch-assembly",
        title: "cst816d capacitive touch & 2.0\" st7789 ips",
        subtitle: "custom win95 ui · 9x9 clamping · 14px deadzone · haptic click",
        category: "keyrings",
        image: "assets/branding/banner.png",
        dimensions: "Dual-aperture trackpad coordinate transformation",
        silicon: "Hynitron CST816D self-capacitive single-touch chip",
        display: "Full-color IPS 178° viewing angle, 240×320 pixels",
        material: "Cover glass with anti-fingerprint oleophobic coating",
        stackup: "FPC ZIF connector with zero insertion force latch",
        firmware: "ADC calibrated battery telemetry & sleep manager",
        tolerance: "Sub-millimeter cursor position accuracy",
    },
];
