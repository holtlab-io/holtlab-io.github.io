//! ## Overview
//! Localized strings for the studio engineering methodology and 4-phase workflow.
//! Contrasts fragmented multi-agency outsourcing against Holt Lab's cohesive hardware, firmware, and IoT integration.
//!
//! ## Search Tags
//! #i18n, #studio, #turnkey, #pipeline, #co-design, #iot
//!
//! ## Submodules
//! None. This module provides atomic studio methodology localization models.

use super::Language;

/// Localized string bundle for the studio comparison card.
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct ComparisonCardStrings {
    /// Top badge text.
    pub badge: &'static str,
    /// Metric or timeline annotation.
    pub metric: &'static str,
    /// Card heading text.
    pub heading: &'static str,
    /// Narrative description paragraph.
    pub body: &'static str,
    /// First verification bullet item.
    pub bullet_1: &'static str,
    /// Second verification bullet item.
    pub bullet_2: &'static str,
    /// Third verification bullet item.
    pub bullet_3: &'static str,
}

/// Localized single phase step in the turnkey pipeline.
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct PipelinePhaseStrings {
    /// Step sequence indicator.
    pub phase_tag: &'static str,
    /// Phase headline title.
    pub title: &'static str,
    /// Phase description paragraph.
    pub desc: &'static str,
}

/// Localized string bundle for the complete studio section.
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct StudioStrings {
    /// Section eyebrow badge text.
    pub eyebrow: &'static str,
    /// Section main title.
    pub title: &'static str,
    /// Subtitle telemetry description.
    pub subtitle: &'static str,
    /// Traditional agency comparison card data.
    pub traditional: ComparisonCardStrings,
    /// Holt Lab turnkey comparison card data.
    pub turnkey: ComparisonCardStrings,
    /// 4-phase turnkey development pipeline steps.
    pub pipeline: [PipelinePhaseStrings; 4],
}

fn traditional_card(lang: Language) -> ComparisonCardStrings {
    match lang {
        Language::En => ComparisonCardStrings {
            badge: "FRAGMENTED MULTI-AGENCY OUTSOURCING",
            metric: "FREQUENT REDESIGNS & DELAYS",
            heading: "The Cost of Disconnected Contractors",
            body: "Mechanical designers model shells without exact PCB component heights. Electronics contractors route boards without firmware pin constraints. Firmware and backend teams receive hardware with connector mismatches and signal bottlenecks — leading to finger-pointing and repeated rework cycles.",
            bullet_1: "✕ Enclosure, connector, and battery clearance clashes found after fabrication",
            bullet_2: "✕ Firmware and cloud integration stalled by hardware interface oversights",
            bullet_3: "✕ Months lost coordinating revisions across three or four separate vendors",
        },
        Language::Ko => ComparisonCardStrings {
            badge: "기존 분절형 다중 외주 방식",
            metric: "잦은 재설계 및 일정 지연",
            heading: "기구·회로·소프트웨어 외주 분리로 인한 병목",
            body: "기구 설계실은 내부 기판 높이와 커넥터 유격을 모른 채 외함을 설계하고, 회로 외주사는 펌웨어 부트 핀 제약이나 케이스 간섭을 고려하지 않고 기판을 넘깁니다. 시제품 부품을 모아 조립하는 단계에서야 규격 불일치가 드러나 책임 공방과 반복적인 재발주 비용이 발생합니다.",
            bullet_1: "✕ 외함 출력·가공 후 뒤늦게 발견되는 기판·배터리·커넥터 간섭",
            bullet_2: "✕ 회로와 펌웨어·서버 간 인터페이스 불일치로 인한 브링업 지연",
            bullet_3: "✕ 업체 간 커뮤니케이션 비용 증가 및 불필요한 기판 재제작 반복",
        },
    }
}

fn turnkey_card(lang: Language) -> ComparisonCardStrings {
    match lang {
        Language::En => ComparisonCardStrings {
            badge: "HOLT LAB UNIFIED ENGINEERING",
            metric: "ALIGNED DESIGN & VALIDATION",
            heading: "Cohesive Co-Design Through Physical Validation",
            body: "Mechanical enclosures, multi-layer PCBs, embedded firmware, and optional IoT backend services are architected together from day one. We run 3D component clearance and electrical rule checks before ordering parts, followed by hands-on physical assembly, bench bring-up, and iterative refinement.",
            bullet_1: "✓ Upfront 3D STEP fit checks prevent major mechanical-electrical clashes",
            bullet_2: "✓ Firmware pin assignments, power budgets, and RF needs built into ECAD",
            bullet_3: "✓ Hands-on prototype assembly, hardware bring-up, and system verification",
        },
        Language::Ko => ComparisonCardStrings {
            badge: "HOLT LAB 통합 엔지니어링",
            metric: "설계 일관성 및 시행착오 최소화",
            heading: "설계부터 실물 조립·연동까지 이어지는 단일 루프",
            body: "기구 외함, 전자 회로 기판, 임베디드 펌웨어, 그리고 필요한 경우 IoT 백엔드 서버까지 하나의 아키텍처 안에서 유기적으로 설계합니다. 발주 전 부품 3D 모델 간섭과 회로 규칙을 교차 검증하고, 시제품 수령 후 실제 가조립·계측·보완을 거쳐 검증된 결과물을 전달합니다.",
            bullet_1: "✓ 3D 부품·외함 사전 정합성 검증으로 치명적인 기구 설계 오류 예방",
            bullet_2: "✓ 회로 설계 단계부터 펌웨어 핀맵·전력 소모·무선 통신 규격 동시 반영",
            bullet_3: "✓ 시제품 실물 조립, 보드 브링업 테스트 및 서버 연동 확인 후 인도",
        },
    }
}

fn pipeline_phases_en() -> [PipelinePhaseStrings; 4] {
    [
        PipelinePhaseStrings {
            phase_tag: "01 // ARCHITECTURE & BOM LOCK",
            title: "System Spec & Component Selection",
            desc: "Define functional goals, power budgets, and connectivity while locking procurable components and verified 3D STEP models.",
        },
        PipelinePhaseStrings {
            phase_tag: "02 // UNIFIED MCAD & ECAD",
            title: "Concurrent Enclosure & PCB Design",
            desc: "Co-design multi-layer PCB layouts and mechanical housings with cross-domain 3D clearance and ERC/DRC verification.",
        },
        PipelinePhaseStrings {
            phase_tag: "03 // PROTOTYPE & FIT CHECK",
            title: "Fabrication & Physical Assembly",
            desc: "Produce SMT-assembled boards and prototype enclosures to inspect physical fit, tolerances, power rails, and thermal behavior.",
        },
        PipelinePhaseStrings {
            phase_tag: "04 // FIRMWARE & IOT INTEGRATION",
            title: "Board Bring-Up & Cloud Link",
            desc: "Validate real-time firmware on physical hardware and integrate custom backend APIs, web dashboards, or OTA pipelines as required.",
        },
    ]
}

fn pipeline_phases_ko() -> [PipelinePhaseStrings; 4] {
    [
        PipelinePhaseStrings {
            phase_tag: "01 // 요구사항 및 부품 확정",
            title: "아키텍처 정의 및 핵심 부품 선정",
            desc: "요구 기능, 목표 크기, 소비 전력, 통신 방식을 정의하고 실제 조달 가능한 부품 데이터시트와 3D 모델을 확정합니다.",
        },
        PipelinePhaseStrings {
            phase_tag: "02 // 기구 · 회로 통합 설계",
            title: "기판 레이아웃 및 외함 동시 설계",
            desc: "다층 회로 기판(PCBA) 배선과 기구 외함 모델링을 병행하며 커넥터 위치, 조립 유격, 전기적 설계 규칙(ERC/DRC)을 교차 검증합니다.",
        },
        PipelinePhaseStrings {
            phase_tag: "03 // 시제품 제작 및 조립 검증",
            title: "시제품 발주 · 실물 가조립 점검",
            desc: "SMT 실장 기판과 3D 프린팅·가공 외함을 제작하여 실제 물리적 결합 상태, 조작감, 발열 및 전원 안정성을 면밀히 점검하고 보완합니다.",
        },
        PipelinePhaseStrings {
            phase_tag: "04 // 펌웨어 · 서버 통합 완주",
            title: "실기 브링업 및 IoT 서버 연동",
            desc: "실물 보드 위에서 임베디드 펌웨어를 최적화하고, 프로젝트 범위에 따라 백엔드 API 서버 및 웹 관제 대시보드까지 연동하여 인도합니다.",
        },
    ]
}

/// Returns the studio section string bundle for the requested language.
pub fn get_studio_strings(lang: Language) -> StudioStrings {
    let (eyebrow, title, subtitle, pipeline) = match lang {
        Language::En => (
            "Engineering Methodology",
            "Why Unified Hardware Engineering?",
            "Eliminating Multi-Vendor Silos // Co-Design to Validated Prototype",
            pipeline_phases_en(),
        ),
        Language::Ko => (
            "개발 방식 및 프로세스",
            "왜 통합 엔지니어링인가?",
            "외주 간 소통 단절 해소 // 설계 검증부터 실물 시제품 조립·연동까지",
            pipeline_phases_ko(),
        ),
    };

    StudioStrings {
        eyebrow,
        title,
        subtitle,
        traditional: traditional_card(lang),
        turnkey: turnkey_card(lang),
        pipeline,
    }
}
