//! ## Overview
//! Localized data models for the 10-row Swiss engineering specification matrix table.
//! Provides bilingual parity for exact physical dimensions, silicon parameters, and fabrication references.
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
            eyebrow: "engineering reference // benchmark metrics",
            title: "technical specifications & verification metrics",
            subtitle: "win95 minesweeper touch keyring // verified benchmark telemetry",
        },
        Language::Ko => SpecsSectionStrings {
            eyebrow: "엔지니어링 제원 // 검증 메트릭",
            title: "상세 기술 제원 및 검증 메트릭",
            subtitle: "윈도우 95 터치 지뢰찾기 키링 // 검증 벤치 텔레메트리",
        },
    }
}

fn rows_part1_en() -> [SpecRowData; 5] {
    [
        SpecRowData {
            key: "dimensions & mass",
            val: "43.40 × 69.20 × 13.78 mm // 38.2 g",
            note: "fully assembled chassis including lipo battery and 2.0\" display",
        },
        SpecRowData {
            key: "chassis material",
            val: "HP Multi Jet Fusion (MJF) PA12",
            note: "glass bead vapor honed finish, 1.20 mm nominal wall thickness",
        },
        SpecRowData {
            key: "cad kernel & drc",
            val: "build123d 0.7.0 + OpenCascade B-Rep",
            note: "interference-free volumetric collision check against COTS 3D STEP models",
        },
        SpecRowData {
            key: "pcba stackup",
            val: "Industrial 4-Layer Controlled-Impedance Stackup (1.0 mm)",
            note: "controlled impedance: 50Ω single-ended, 90Ω differential USB",
        },
        SpecRowData {
            key: "silicon processor",
            val: "Espressif ESP32-C3 RISC-V 32-bit @ 160MHz",
            note: "ESP32-C3-WROOM-02-N4 module with integrated 4MB SPI flash",
        },
    ]
}

fn rows_part2_en() -> [SpecRowData; 5] {
    [
        SpecRowData {
            key: "display & bus",
            val: "2.0\" ST7789T3 IPS (240×320 RGB)",
            note: "4-wire SPI2 @ 26.67MHz (80MHz/3) with GDMA zero-copy transfer",
        },
        SpecRowData {
            key: "touch digitizer",
            val: "Hynitron CST816D Capacitive Single-Touch",
            note: "I2C 400kHz Fast-Mode with GPIO6 falling-edge low-jitter interrupt",
        },
        SpecRowData {
            key: "battery & power",
            val: "TW301525 150mAh LiPo + DW01A/8205A PCM",
            note: "2.45V hardware cut-off, GPIO0 ADC telemetry, SW1 manual switch",
        },
        SpecRowData {
            key: "firmware runtime",
            val: "Rust esp-hal 1.2.2 + embassy async framework",
            note: "3 concurrent async tasks, 153.6 KB SRAM backbuffer blitting @ 60Hz",
        },
        SpecRowData {
            key: "factory dispatch",
            val: "Factory-Direct SMT Assembly & Industrial MJF Prototyping",
            note: "zero-rework bring-up over remote Tailscale bench environment",
        },
    ]
}

fn rows_part1_ko() -> [SpecRowData; 5] {
    [
        SpecRowData {
            key: "외형 치수 및 무게",
            val: "43.40 × 69.20 × 13.78 mm // 38.2 g",
            note: "LiPo 배터리 및 2.0\" 디스플레이를 포함한 완전 조립 상태 기준",
        },
        SpecRowData {
            key: "외함 기구 소재",
            val: "HP Multi Jet Fusion (MJF) PA12",
            note: "유리 비드 증기 연마 마감, 1.20 mm 공칭 벽두께",
        },
        SpecRowData {
            key: "CAD 커널 및 DRC",
            val: "build123d 0.7.0 + OpenCascade B-Rep",
            note: "COTS 3D STEP 모델 기반 3D 체적 무간섭 일체형 검증",
        },
        SpecRowData {
            key: "회로 기판 적층",
            val: "산업용 4층 임피던스 제어 기판 (1.0 mm)",
            note: "제어 임피던스: 50Ω 싱글엔디드, 90Ω 차동 USB",
        },
        SpecRowData {
            key: "프로세서 칩셋",
            val: "Espressif ESP32-C3 RISC-V 32-bit @ 160MHz",
            note: "4MB SPI 플래시 내장 ESP32-C3-WROOM-02-N4 모듈",
        },
    ]
}

fn rows_part2_ko() -> [SpecRowData; 5] {
    [
        SpecRowData {
            key: "디스플레이 및 버스",
            val: "2.0\" ST7789T3 IPS (240×320 RGB)",
            note: "GDMA 제로카피 전송 기반 4선식 SPI2 @ 26.67MHz (80MHz/3)",
        },
        SpecRowData {
            key: "정전식 터치 센서",
            val: "Hynitron CST816D Capacitive Single-Touch",
            note: "GPIO6 하강 에지 저지터 인터럽트 및 I2C 400kHz 패스트모드",
        },
        SpecRowData {
            key: "배터리 및 전원 관리",
            val: "TW301525 150mAh LiPo + DW01A/8205A PCM",
            note: "2.45V 하드웨어 차단, GPIO0 ADC 배터리 모니터링, SW1 수동 스위치",
        },
        SpecRowData {
            key: "임베디드 펌웨어 런타임",
            val: "Rust esp-hal 1.2.2 + embassy async framework",
            note: "3개 동시 비동기 태스크, 153.6 KB SRAM 백버퍼 60Hz 고속 렌더링",
        },
        SpecRowData {
            key: "공장 직통 생산 및 배포",
            val: "정밀 SMT 실장 및 공장 직통 양산 시제품 납품",
            note: "Tailscale 원격 벤치 환경을 통한 리워크 없는 단번에 구동 성공",
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
