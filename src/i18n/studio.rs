//! ## Overview
//! Localized strings for the studio turnkey value proposition and co-design pipeline.
//! Contrasts traditional fragmented multi-agency development against Holt Lab's closed-loop execution.
//!
//! ## Search Tags
//! #i18n, #studio, #turnkey, #pipeline, #co-design
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
    /// Step sequence indicator (e.g. 01 // CO-DESIGN LOCK).
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
            badge: "TRADITIONAL AGENCY FRAGMENTATION",
            metric: "6–12 MONTHS SLIPPAGE",
            heading: "the multi-vendor ping-pong trap",
            body: "mechanical teams design shells in a vacuum. electrical agencies route boards without COTS 3D models. firmware contractors receive physical prototypes with swapped pins, strapping bus lockups, and unrouted RC delays. weeks turn into months of finger-pointing and costly PCB respins.",
            bullet_1: "✕ MCAD / ECAD clearance mismatches discovered after tooling",
            bullet_2: "✕ Firmware blocked by unexpected internal module resistance",
            bullet_3: "✕ 3+ PCB revisions required before initial firmware bring-up",
        },
        Language::Ko => ComparisonCardStrings {
            badge: "기존 분절형 다중 외주 모델",
            metric: "6~12개월 일정 지연",
            heading: "다중 외주 업체의 핑퐁 딜레마",
            body: "기구 설계팀은 회로를 고려하지 않고 외함을 그립니다. 회로 설계사는 실제 부품 3D 모델 없이 기판을 배선합니다. 펌웨어 외주사는 핀이 바뀌고 스트래핑 버스가 꼬이고 RC 신호 지연이 고려되지 않은 불량 보드를 전달받습니다. 책임 공방 속에 수개월이 낭비되고 수백만 원의 PCB 재제작 비용이 발생합니다.",
            bullet_1: "✕ 금형/출력 후 뒤늦게 발견되는 기구·회로 간섭 오류",
            bullet_2: "✕ 모듈 내부 저항 및 핀 충돌로 펌웨어 브링업 전면 중단",
            bullet_3: "✕ 첫 펌웨어 동작까지 3회 이상의 PCB 재설계 및 재제작 반복",
        },
    }
}

fn turnkey_card(lang: Language) -> ComparisonCardStrings {
    match lang {
        Language::En => ComparisonCardStrings {
            badge: "HOLT LAB UNIFIED TURNKEY",
            metric: "ZERO-REWORK GUARANTEED",
            heading: "unified co-design loop",
            body: "every millimeter of chassis geometry, copper impedance trace, and real-time clock cycle is engineered within a single unified co-design loop. COTS 3D STEP models are locked before trace freeze. Volumetric collision checks and 0 ERC/DRC verification run deterministically before manufacturing orders.",
            bullet_1: "✓ Interference-free assembly: OpenCascade B-Rep Gate 1 check",
            bullet_2: "✓ Firmware-aware ECAD: GPIO strapping & RC delays solved in design",
            bullet_3: "✓ First-spin success: working hardware boots over remote Tailscale bench",
        },
        Language::Ko => ComparisonCardStrings {
            badge: "홀트 랩 통합 턴키 솔루션",
            metric: "단 1회 무결 브링업",
            heading: "단일 통합 엔지니어링 루프",
            body: "기구 외함의 0.1밀리미터 치수부터 회로의 임피던스 배선, 고신뢰성 펌웨어의 클록 사이클까지 기구·회로·펌웨어를 하나의 통합된 엔지니어링 아키텍처로 총괄합니다. 배선 확정 전 부품 3D STEP 모델을 100% 동기화하며, 발주 전 3D 체적 간섭 및 0 ERC/DRC를 결정론적으로 검증합니다.",
            bullet_1: "✓ 무간섭 일체형 설계: OpenCascade B-Rep 게이트 1 사전 검증",
            bullet_2: "✓ 펌웨어 연계 회로: 스트래핑 핀 및 RC 지연을 회로 설계 단에서 해결",
            bullet_3: "✓ 단 1회 생산 성공: Tailscale 원격 환경에서 즉시 펌웨어 구동 완주",
        },
    }
}

fn pipeline_phases_en() -> [PipelinePhaseStrings; 4] {
    [
        PipelinePhaseStrings {
            phase_tag: "01 // CO-DESIGN LOCK",
            title: "datasheets & 3d contract",
            desc: "pinouts, power envelopes, and vendor COTS 3D STEP models frozen before drafting lines.",
        },
        PipelinePhaseStrings {
            phase_tag: "02 // CAD & 4-LAYER ECAD",
            title: "interference-free solid modeling",
            desc: "build123d B-Rep solids paired with KiCad 9 controlled impedance routing and full DRC.",
        },
        PipelinePhaseStrings {
            phase_tag: "03 // FACTORY DISPATCH",
            title: "direct line fabrication",
            desc: "factory-direct precision SMT assembly & industrial MJF PA12 fabrication with live status tracking.",
        },
        PipelinePhaseStrings {
            phase_tag: "04 // REMOTE BENCH BOOT",
            title: "zero-rework firmware",
            desc: "real-time esp-hal runtime, zero-copy DMA graphics, and Tailscale bench flashing.",
        },
    ]
}

fn pipeline_phases_ko() -> [PipelinePhaseStrings; 4] {
    [
        PipelinePhaseStrings {
            phase_tag: "01 // CO-DESIGN LOCK",
            title: "데이터시트 및 3D 스펙 동결",
            desc: "선 하나 긋기 전 핀아웃, 소비 전력 한도, 부품 COTS 3D STEP 모델을 먼저 확정합니다.",
        },
        PipelinePhaseStrings {
            phase_tag: "02 // CAD & 4-LAYER ECAD",
            title: "무간섭 일체형 기구 및 회로 설계",
            desc: "build123d B-Rep 솔리드 모델링과 KiCad 9 제어 임피던스 라우팅 및 0 DRC 검증을 동시 진행합니다.",
        },
        PipelinePhaseStrings {
            phase_tag: "03 // FACTORY DISPATCH",
            title: "공장 직통 고속 발주",
            desc: "정밀 SMT 실장 및 산업용 MJF 나일론 시제품을 실시간 생산 추적과 함께 다이렉트 발주합니다.",
        },
        PipelinePhaseStrings {
            phase_tag: "04 // 임베디드 브링업",
            title: "재작업 없는 단번에 브링업",
            desc: "고신뢰성 esp-hal 및 embassy 런타임, 제로카피 DMA 그래픽스, Tailscale 원격 벤치 플래싱으로 완벽 구동합니다.",
        },
    ]
}

/// Returns the studio section string bundle for the requested language.
pub fn get_studio_strings(lang: Language) -> StudioStrings {
    let (eyebrow, title, subtitle, pipeline) = match lang {
        Language::En => (
            "studio turnkey model",
            "why unified turnkey?",
            "eliminating agency ping-pong // silicon to screen",
            pipeline_phases_en(),
        ),
        Language::Ko => (
            "스튜디오 턴키 모델",
            "왜 통합 턴키인가?",
            "외주 핑퐁 근절 // 실리콘 칩부터 화면까지",
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
