//! ## Overview
//! Localized strings for the flagship studio hero section, navigation headers, and footer credits.
//! Repositions Holt Lab as a hardware engineering studio and provides complete bilingual parity.
//!
//! ## Search Tags
//! #i18n, #hero, #navbar, #footer, #branding, #studio
//!
//! ## Submodules
//! None. This module provides atomic presentation text models.

use super::Language;

/// Localized string bundle for the hero studio section.
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct HeroStrings {
    /// Top eyebrow badge text.
    pub eyebrow: &'static str,
    /// Monumental headline text.
    pub title: &'static str,
    /// Engineering subtitle paragraph.
    pub subtitle: &'static str,
    /// 4 discipline telemetry pills.
    pub pills: [&'static str; 4],
    /// Primary visual boundary width indicator.
    pub width_label: &'static str,
    /// Center visual material indicator.
    pub material_label: &'static str,
    /// Visual boundary height indicator.
    pub height_label: &'static str,
    /// Case study jump CTA button text.
    pub cta_casestudy: &'static str,
    /// Inquiry jump CTA button text.
    pub cta_inquire: &'static str,
    /// Console workbench status banner.
    pub console_status: &'static str,
    /// Console demonstrator caption.
    pub console_sub: &'static str,
}

/// Localized string bundle for navigation bar elements.
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct NavStrings {
    /// Studio subtitle below brand logo.
    pub descriptor: &'static str,
    /// Verified turnkey badge indicator.
    pub turnkey_badge: &'static str,
    /// Direct inquiry CTA button text.
    pub cta_inquire: &'static str,
    /// Desktop navigation anchor labels.
    pub nav_items: [(&'static str, &'static str, &'static str); 5],
    /// Mobile drawer anchor items with subtitle descriptions.
    pub mobile_items: [(&'static str, &'static str, &'static str); 5],
}

/// Localized string bundle for footer credits and copyright.
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct FooterStrings {
    /// Studio subtitle descriptor in footer.
    pub studio_desc: &'static str,
    /// Verification clearance note.
    pub clearance_note: &'static str,
    /// Copyright statement.
    pub copyright: &'static str,
}

fn hero_en() -> HeroStrings {
    HeroStrings {
        eyebrow: "HARDWARE ENGINEERING & RAPID PROTOTYPING STUDIO",
        title: "From Silicon to Screen. We Engineer Physical Hardware.",
        subtitle: "Unified Turnkey Hardware R&D — Mechanical CAD, 4-Layer ECAD, and Low-Latency Embedded Firmware co-designed without agency ping-pong.",
        pills: [
            "01 // MCAD Parametric B-Rep",
            "02 // ECAD 4-Layer Controlled-Z",
            "03 // Embedded 60Hz Async Actors",
            "04 // DFM Rapid SMT Dispatch",
        ],
        width_label: "DIMENSION: 43.40 × 69.20 mm",
        material_label: "MJF PA12 // 4-LAYER PCBA",
        height_label: "STACKUP: 13.78 mm",
        cta_casestudy: "explore case study // 실증 사례 보기",
        cta_inquire: "inquire project // 턴키 의뢰하기",
        console_status: "SYSTEM ARCHITECTURE: UNIFIED CO-DESIGN // TAILSCALE BENCH PASS",
        console_sub: "FLAGSHIP DEMONSTRATOR // OPUS 01 TOUCH GAMING MACHINE",
    }
}

fn hero_ko() -> HeroStrings {
    HeroStrings {
        eyebrow: "턴키 하드웨어 엔지니어링 & 프로토타이핑 랩",
        title: "실리콘 칩셋부터 외함까지, 실물로 작동하는 하드웨어를 설계합니다.",
        subtitle: "기구 CAD, 4층 임피던스 제어 회로, 초저지연 임베디드 펌웨어를 단일 루프로 코디자인하여 외주 핑퐁 없이 단 1회에 실물로 구동시킵니다.",
        pills: [
            "01 // MCAD 파라메트릭 B-Rep",
            "02 // ECAD 4층 제어 임피던스",
            "03 // 펌웨어 60Hz 비동기 액터",
            "04 // DFM 양산 SMT 직통 발주",
        ],
        width_label: "외형 규격: 43.40 × 69.20 mm",
        material_label: "MJF PA12 // 4층 회로 기판",
        height_label: "두께 규격: 13.78 mm",
        cta_casestudy: "실증 사례 보기 // case study",
        cta_inquire: "턴키 의뢰하기 // inquire",
        console_status: "시스템 아키텍처: 통합 코디자인 // 원격 벤치 검증 통과",
        console_sub: "플래그십 실증 머신 // OPUS 01 터치 게임 콘솔",
    }
}

/// Returns the studio hero string bundle for the requested language.
pub fn get_hero_strings(lang: Language) -> HeroStrings {
    match lang {
        Language::En => hero_en(),
        Language::Ko => hero_ko(),
    }
}

/// Returns navigation bar strings for the requested language.
pub fn get_nav_strings(lang: Language) -> NavStrings {
    match lang {
        Language::En => NavStrings {
            descriptor: "Hardware & Embedded Engineering Studio",
            turnkey_badge: "UNIFIED TURNKEY // VERIFIED",
            cta_inquire: "inquire // 의뢰하기",
            nav_items: [
                ("#capabilities", "capabilities", "01"),
                ("#case-study", "case study", "02"),
                ("#specs", "specs", "03"),
                ("#studio", "turnkey", "04"),
                ("#inquire", "inquire", "05"),
            ],
            mobile_items: [
                ("#capabilities", "capabilities", "4 core engineering disciplines"),
                ("#case-study", "case study", "flagship hardware demonstrator"),
                ("#specs", "specs", "monospaced swiss spec matrix"),
                ("#studio", "studio", "unified turnkey model"),
                ("#inquire", "inquire // 의뢰하기", "direct architect dispatch"),
            ],
        },
        Language::Ko => NavStrings {
            descriptor: "하드웨어 & 임베디드 엔지니어링 랩",
            turnkey_badge: "통합 턴키 // 검증 완료",
            cta_inquire: "inquire // 의뢰하기",
            nav_items: [
                ("#capabilities", "엔지니어링 역량", "01"),
                ("#case-study", "실증 사례", "02"),
                ("#specs", "기술 사양", "03"),
                ("#studio", "턴키 모델", "04"),
                ("#inquire", "의뢰하기", "05"),
            ],
            mobile_items: [
                ("#capabilities", "엔지니어링 역량", "4대 핵심 턴키 엔지니어링 역량"),
                ("#case-study", "실증 사례", "플래그십 하드웨어 실증 머신"),
                ("#specs", "기술 사양", "스위스 정밀 기술 사양 매트릭스"),
                ("#studio", "턴키 모델", "통합 턴키 코디자인 모델"),
                ("#inquire", "의뢰하기", "리드 아키텍트 직통 의뢰"),
            ],
        },
    }
}

/// Returns footer strings for the requested language.
pub fn get_footer_strings(lang: Language) -> FooterStrings {
    match lang {
        Language::En => FooterStrings {
            studio_desc: "hardware & embedded laboratory",
            clearance_note: "all systems verified interference-free physical assembly · seoul, korea",
            copyright: "© 2026 holt lab. all rights reserved.",
        },
        Language::Ko => FooterStrings {
            studio_desc: "하드웨어 및 임베디드 연구소",
            clearance_note: "전 시스템 무간섭 물리 결합 검증 완료 · 대한민국 서울",
            copyright: "© 2026 holt lab. 판권 소유.",
        },
    }
}
