//! ## Overview
//! Localized data models and telemetry structures for the 4 core studio turnkey engineering capabilities.
//! Details mechanical CAD, electrical PCBA, embedded firmware, and rapid prototyping capabilities.
//!
//! ## Search Tags
//! #i18n, #capabilities, #disciplines, #mcad, #ecad, #firmware, #dfm
//!
//! ## Submodules
//! None. This module provides atomic presentation text models.

use super::Language;

/// Single discipline capability card data model.
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct CapabilityCard {
    /// Discipline numerical indicator (e.g. 01).
    pub number: &'static str,
    /// Technical domain badge tag.
    pub tag: &'static str,
    /// Discipline title heading.
    pub title: &'static str,
    /// Detailed capability description.
    pub desc: &'static str,
    /// Key engineering metric and verification standard.
    pub metric: &'static str,
}

/// Localized string bundle for the studio engineering capabilities section.
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct CapabilitiesStrings {
    /// Top eyebrow badge text.
    pub eyebrow: &'static str,
    /// Section main title.
    pub title: &'static str,
    /// Engineering subtitle paragraph.
    pub subtitle: &'static str,
    /// 4 core engineering discipline cards.
    pub cards: [CapabilityCard; 4],
}

fn cap_en_01_02() -> [CapabilityCard; 2] {
    [
        CapabilityCard {
            number: "01",
            tag: "PARAMETRIC B-REP // SOLID KERNEL",
            title: "Mechanical CAD & Enclosures",
            desc: "Interference-free solid modeling via build123d OpenCascade B-Rep. Verified against vendor COTS 3D STEP models with volumetric collision analysis before tooling. Optimized for HP MJF PA12 nylon, CNC aluminum, and custom silicone sealing.",
            metric: "interlocking fit · zero-collision solid check",
        },
        CapabilityCard {
            number: "02",
            tag: "CONTROLLED-Z // KICAD 9",
            title: "Electrical PCBA & High-Speed Routing",
            desc: "Controlled-impedance 4-layer stackup routing with solid inner ground planes for continuous return paths and ultra-low noise. Integrated safe LiPo battery charging, hardware cut-off protection, TVS ESD arrays, and strapping bus isolation.",
            metric: "50Ω / 90Ω diff · 0 ERC · 0 DRC verified",
        },
    ]
}

fn cap_en_03_04() -> [CapabilityCard; 2] {
    [
        CapabilityCard {
            number: "03",
            tag: "REAL-TIME ASYNC // EMBEDDED RUST",
            title: "Embedded Firmware & DMA Engine",
            desc: "Pure #![no_std] firmware built on esp-hal and embassy async actors. Zero-copy AHB GDMA display pipelines blitting at 60Hz with zero runtime heap allocations. Microsecond interrupt latency, low-jitter touch, and microamp deep sleep standby.",
            metric: "60Hz GDMA pipeline · 0 alloc runtime",
        },
        CapabilityCard {
            number: "04",
            tag: "SMT FAB & BENCH // TAILSCALE",
            title: "DFM & Rapid Bench Verification",
            desc: "Direct SMT assembly dispatch and industrial additive manufacturing. Automated bill of materials reconciliation and remote Tailscale hardware testbed bring-up with zero reset drops, guaranteeing working physical prototypes on the first iteration.",
            metric: "first-spin bring-up · bench-verified prototype",
        },
    ]
}

fn cap_ko_01_02() -> [CapabilityCard; 2] {
    [
        CapabilityCard {
            number: "01",
            tag: "파라메트릭 B-REP // 솔리드 커널",
            title: "정밀 기구 CAD 및 산업용 외함 설계",
            desc: "build123d OpenCascade B-Rep 커널 기반 파라메트릭 기구 모델링. 부품 3D STEP 모델과의 체적 간섭 분석을 거쳐 금형 및 가공 전 완벽한 결합을 보장합니다. 산업용 HP MJF PA12 나일론, CNC 알루미늄, 정밀 실리콘 가스켓에 최적화 설계합니다.",
            metric: "정밀 결합 · 3D 무간섭 체적 검증",
        },
        CapabilityCard {
            number: "02",
            tag: "임피던스 제어 // KICAD 9",
            title: "4층 제어 임피던스 고신뢰성 PCBA",
            desc: "연속 신호 귀환 경로와 극저노이즈를 보장하는 솔리드 접지/전원 내층 기반 고밀도 4층 배선. 안전 LiPo 충전 회로, 하드웨어 배터리 차단 보호, TVS 정전기 보호, 펌웨어 연계 스트래핑 핀 설계를 일괄 처리합니다.",
            metric: "50Ω / 90Ω 차동 임피던스 · 0 ERC/DRC 검증",
        },
    ]
}

fn cap_ko_03_04() -> [CapabilityCard; 2] {
    [
        CapabilityCard {
            number: "03",
            tag: "실시간 비동기 // 임베디드 펌웨어",
            title: "고신뢰성 실시간 임베디드 펌웨어",
            desc: "esp-hal 및 embassy 비동기 액터 기반의 순수 #![no_std] 펌웨어. 런타임 힙 할당 없는 153.6 KB SRAM 직접 매핑과 60Hz AHB GDMA 제로카피 디스플레이 파이프라인. 마이크로초 단위 저지터 터치 인터럽트와 초절전 대기 모드를 구현합니다.",
            metric: "60Hz GDMA 제로카피 · 0 alloc 런타임",
        },
        CapabilityCard {
            number: "04",
            tag: "프로토타입 발주 // 벤치 검증",
            title: "공장 직통 프로토타입 발주 및 실기 벤치 검증",
            desc: "정밀 SMT 실장 및 산업용 적층 제조를 통한 신속한 시제품 발주. 자동화된 BOM 대조와 원격 하드웨어 벤치를 통해, 도면 전달에 그치지 않고 실물 가조립부터 실제 펌웨어 구동까지 끝까지 검증합니다.",
            metric: "실물 가조립 검증 · 벤치 브링업 완주",
        },
    ]
}

/// Returns the studio engineering capabilities string bundle for the requested language.
pub fn get_capabilities_strings(lang: Language) -> CapabilitiesStrings {
    match lang {
        Language::En => {
            let p1 = cap_en_01_02();
            let p2 = cap_en_03_04();
            CapabilitiesStrings {
                eyebrow: "capabilities // core disciplines",
                title: "4 pillars of unified turnkey hardware",
                subtitle: "eliminating multi-vendor friction through closed-loop mechanical, electrical, firmware, and production integration.",
                cards: [p1[0], p1[1], p2[0], p2[1]],
            }
        }
        Language::Ko => {
            let p1 = cap_ko_01_02();
            let p2 = cap_ko_03_04();
            CapabilitiesStrings {
                eyebrow: "엔지니어링 역량 // 핵심 도메인",
                title: "4대 턴키 하드웨어 엔지니어링 역량",
                subtitle: "외주 간 책임 공방과 일정 지연을 원천 차단하는 기구·회로·펌웨어·생산 일원화 체계.",
                cards: [p1[0], p1[1], p2[0], p2[1]],
            }
        }
    }
}
