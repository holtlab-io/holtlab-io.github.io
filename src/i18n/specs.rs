//! ## Overview
//! Localized data models for the 10-row Swiss engineering specification matrix table.
//! Provides bilingual parity for exact physical dimensions, silicon parameters, and verification details.
//!
//! ## Search Tags
//! #i18n, #specs, #matrix, #specifications, #stackup
//!
//! ## Submodules
//! None. This module provides atomic specification data models.

use super::Language;

/// Localized string bundle for the specification section headings.
#[derive(Clone, Copy)]
pub struct SpecsSectionStrings {
    /// Section eyebrow badge.
    pub eyebrow: &'static str,
    /// Main section title.
    pub title: &'static str,
    /// Subtitle telemetry description.
    pub subtitle: &'static str,
}

/// Single row item in the Swiss specification matrix table.
#[derive(Clone, Copy)]
pub struct SpecRowData {
    /// Specification metric category label.
    pub key: &'static str,
    /// Numeric value and engineering dimension.
    pub val: &'static str,
    /// Technical annotation or verification constraint.
    pub note: &'static str,
}

/// Returns the specification section heading strings.
pub fn get_specs_section_strings(lang: Language) -> SpecsSectionStrings {
    match lang {
        Language::En => SpecsSectionStrings {
            eyebrow: "ENGINEERING REFERENCE · CASE STUDY METRICS",
            title: "Technical Specifications & Verification Metrics",
            subtitle: "Win95 Minesweeper Touch Keyring · Assembled Hardware Parameters",
        },
        Language::Ko => SpecsSectionStrings {
            eyebrow: "엔지니어링 제원 · 케이스 스터디 사양",
            title: "상세 기술 제원 및 검증 사양",
            subtitle: "Win95 마인스위퍼 터치 키링 · 실물 시제품 측정 및 설계 기준",
        },
    }
}

fn rows_part1_en() -> [SpecRowData; 5] {
    [
        SpecRowData {
            key: "Dimensions & Mass",
            val: "43.40 × 69.20 × 13.78 mm · 38.2 g",
            note: "Fully assembled chassis including LiPo battery and 2.0\" IPS display",
        },
        SpecRowData {
            key: "Enclosure Material",
            val: "HP Multi Jet Fusion (MJF) PA12 Nylon",
            note: "Bead-blasted matte finish, 1.20 mm nominal wall thickness",
        },
        SpecRowData {
            key: "Mechanical CAD",
            val: "Parametric 3D B-Rep Solid Modeling",
            note: "Assembly fit and clearance verified against vendor 3D STEP models",
        },
        SpecRowData {
            key: "PCBA Stackup",
            val: "4-Layer Controlled-Impedance PCB (1.0 mm)",
            note: "Controlled impedance: 50Ω single-ended, 90Ω differential USB",
        },
        SpecRowData {
            key: "Wireless MCU",
            val: "Espressif ESP32-C3 RISC-V 32-bit @ 160 MHz",
            note: "ESP32-C3-WROOM-02-N4 module with integrated 4MB SPI flash",
        },
    ]
}

fn rows_part2_en() -> [SpecRowData; 5] {
    [
        SpecRowData {
            key: "Display & Bus",
            val: "2.0\" ST7789T3 IPS (240×320 RGB)",
            note: "4-wire SPI2 @ 26.67 MHz (80MHz/3) with AHB GDMA zero-copy transfer",
        },
        SpecRowData {
            key: "Touch Controller",
            val: "Hynitron CST816D Capacitive Touch",
            note: "I2C 400 kHz Fast-Mode with GPIO6 falling-edge low-jitter interrupt",
        },
        SpecRowData {
            key: "Battery & Protection",
            val: "TW301525 150mAh LiPo + DW01A/8205A PCM",
            note: "2.45V hardware cut-off, GPIO0 ADC voltage sensing, manual power switch",
        },
        SpecRowData {
            key: "Firmware Runtime",
            val: "Bare-Metal Async Multi-Tasking (Zero-Heap)",
            note: "3 concurrent async tasks, 153.6 KB static SRAM backbuffer @ 60Hz",
        },
        SpecRowData {
            key: "Prototype Validation",
            val: "SMT PCBA Assembly & MJF Enclosure Bring-Up",
            note: "Physical assembly, power/ADC calibration, and real-hardware testing",
        },
    ]
}

fn rows_part1_ko() -> [SpecRowData; 5] {
    [
        SpecRowData {
            key: "외형 치수 및 무게",
            val: "43.40 × 69.20 × 13.78 mm · 38.2 g",
            note: "LiPo 배터리 및 2.0\" IPS 디스플레이를 포함한 완전 조립 기준",
        },
        SpecRowData {
            key: "외함 기구 소재",
            val: "HP Multi Jet Fusion (MJF) PA12 나일론",
            note: "비드 블라스트 매트 마감, 1.20 mm 공칭 벽두께",
        },
        SpecRowData {
            key: "기구 설계 방식",
            val: "파라메트릭 3D B-Rep 솔리드 모델링",
            note: "실제 부품 3D STEP 모델 기반 조립 유격 및 간섭 사전 검증",
        },
        SpecRowData {
            key: "회로 기판 적층",
            val: "4층 제어 임피던스 기판 (두께 1.0 mm)",
            note: "제어 임피던스: 50Ω 싱글엔디드, 90Ω 차동 USB",
        },
        SpecRowData {
            key: "무선 마이크로컨트롤러",
            val: "Espressif ESP32-C3 RISC-V 32-bit @ 160 MHz",
            note: "4MB SPI 플래시 내장 ESP32-C3-WROOM-02-N4 모듈",
        },
    ]
}

fn rows_part2_ko() -> [SpecRowData; 5] {
    [
        SpecRowData {
            key: "디스플레이 및 버스",
            val: "2.0\" ST7789T3 IPS (240×320 RGB)",
            note: "AHB GDMA 제로카피 전송 기반 4선식 SPI2 @ 26.67 MHz",
        },
        SpecRowData {
            key: "정전식 터치 센서",
            val: "Hynitron CST816D Capacitive Touch",
            note: "GPIO6 하강 에지 인터럽트 및 I2C 400 kHz 패스트모드",
        },
        SpecRowData {
            key: "배터리 및 보호 회로",
            val: "TW301525 150mAh LiPo + DW01A/8205A PCM",
            note: "2.45V 하드웨어 과방전 차단, GPIO0 전압 계측, 슬라이드 전원 스위치",
        },
        SpecRowData {
            key: "임베디드 펌웨어",
            val: "베어메탈 비동기 멀티태스킹 (Zero-Heap)",
            note: "3개 비동기 태스크 동시 제어, 153.6 KB 정적 SRAM 백버퍼 60Hz 렌더링",
        },
        SpecRowData {
            key: "시제품 제작 및 검증",
            val: "정밀 SMT 실장 기판 및 MJF 외함 실물 브링업",
            note: "실물 가조립, 전원·ADC 캘리브레이션 및 실제 구동 검증 완료",
        },
    ]
}

/// Returns the 10 engineering specification rows for the selected language.
pub fn get_spec_rows(lang: Language) -> [SpecRowData; 10] {
    let (p1, p2) = match lang {
        Language::En => (rows_part1_en(), rows_part2_en()),
        Language::Ko => (rows_part1_ko(), rows_part2_ko()),
    };
    [
        p1[0], p1[1], p1[2], p1[3], p1[4],
        p2[0], p2[1], p2[2], p2[3], p2[4],
    ]
}
