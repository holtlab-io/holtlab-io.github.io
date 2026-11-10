//! ## Overview
//! Localized strings for the flagship hero section, navigation headers, and footer credits.
//! Provides complete bilingual parity between English and Korean presentation.
//!
//! ## Search Tags
//! #i18n, #hero, #navbar, #footer, #branding
//!
//! ## Submodules
//! None. This module provides atomic presentation text models.

use super::Language;

/// Localized string bundle for the hero demonstrator section.
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct HeroStrings {
    /// Top eyebrow badge text.
    pub eyebrow: &'static str,
    /// Monumental headline text.
    pub title: &'static str,
    /// Engineering subtitle paragraph.
    pub subtitle: &'static str,
    /// 5 quick specification pills.
    pub pills: [&'static str; 5],
    /// Primary visual boundary width indicator.
    pub width_label: &'static str,
    /// Center visual material indicator.
    pub material_label: &'static str,
    /// Visual boundary height indicator.
    pub height_label: &'static str,
    /// Teardown jump CTA button text.
    pub cta_teardown: &'static str,
    /// Specs jump CTA button text.
    pub cta_specs: &'static str,
}

/// Localized string bundle for navigation bar elements.
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct NavStrings {
    /// Studio subtitle below brand logo.
    pub descriptor: &'static str,
    /// Verified tolerance badge indicator.
    pub tolerance_badge: &'static str,
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

/// Returns the hero string bundle for the requested language.
pub fn get_hero_strings(lang: Language) -> HeroStrings {
    match lang {
        Language::En => HeroStrings {
            eyebrow: "OPUS 01 // HARDWARE DEMONSTRATOR",
            title: "win95 minesweeper touch keyring",
            subtitle: "0.00 mm³ collision-free mechanical CAD, 4-layer impedance-controlled PCBA, and bare-metal Rust GDMA 60Hz graphics engine. A living proof of 1-person turnkey hardware execution from silicon to screen.",
            pills: [
                "43.4×69.2×13.78mm",
                "ESP32-C3 160MHz",
                "2.0\" 60Hz IPS",
                "HP MJF PA12",
                "Bare-metal Rust",
            ],
            width_label: "WIDTH: 43.40 mm",
            material_label: "MJF PA12 // BEZEL-LESS",
            height_label: "HEIGHT: 69.20 mm",
            cta_teardown: "explore teardown",
            cta_specs: "technical specs",
        },
        Language::Ko => HeroStrings {
            eyebrow: "OPUS 01 // 하드웨어 실증 머신",
            title: "win95 minesweeper touch keyring",
            subtitle: "0.00 mm³ 무간섭 기구 CAD, 4층 임피던스 제어 PCBA, 베어메탈 Rust GDMA 60Hz 그래픽스 엔진. 실리콘 칩셋부터 외함과 화면까지 1인 턴키 하드웨어 개발의 정점을 증명합니다.",
            pills: [
                "43.4×69.2×13.78mm",
                "ESP32-C3 160MHz",
                "2.0\" 60Hz IPS",
                "HP MJF PA12",
                "베어메탈 Rust",
            ],
            width_label: "가로: 43.40 mm",
            material_label: "MJF PA12 // 초슬림 베젤",
            height_label: "세로: 69.20 mm",
            cta_teardown: "하드웨어 분해 보기",
            cta_specs: "상세 기술 사양",
        },
    }
}

/// Returns navigation bar strings for the requested language.
pub fn get_nav_strings(lang: Language) -> NavStrings {
    match lang {
        Language::En => NavStrings {
            descriptor: "Boutique Hardware Engineering Studio",
            tolerance_badge: "0.00 mm³ TOLERANCE // VERIFIED",
            cta_inquire: "inquire // 의뢰하기",
            nav_items: [
                ("#demonstrator", "demonstrator", "01"),
                ("#teardown", "teardown", "02"),
                ("#specs", "specs", "03"),
                ("#studio", "studio", "04"),
                ("#inquire", "inquire", "05"),
            ],
            mobile_items: [
                ("#demonstrator", "demonstrator", "flagship hardware machine"),
                ("#teardown", "teardown", "4-layer engineering teardown"),
                ("#specs", "specs", "monospaced swiss spec matrix"),
                ("#studio", "studio", "1-person turnkey model"),
                ("#inquire", "inquire // 의뢰하기", "direct architect dispatch"),
            ],
        },
        Language::Ko => NavStrings {
            descriptor: "부티크 하드웨어 엔지니어링 스튜디오",
            tolerance_badge: "0.00 mm³ 공차 // 검증 완료",
            cta_inquire: "inquire // 의뢰하기",
            nav_items: [
                ("#demonstrator", "실증 기기", "01"),
                ("#teardown", "하드웨어 분해", "02"),
                ("#specs", "기술 사양", "03"),
                ("#studio", "턴키 모델", "04"),
                ("#inquire", "의뢰하기", "05"),
            ],
            mobile_items: [
                ("#demonstrator", "실증 기기", "플래그십 하드웨어 실증 머신"),
                ("#teardown", "하드웨어 분해", "4계층 엔지니어링 분해 분석"),
                ("#specs", "기술 사양", "스위스 정밀 기술 사양 매트릭스"),
                ("#studio", "턴키 모델", "1인 턴키 코디자인 모델"),
                ("#inquire", "의뢰하기", "리드 아키텍트 직통 의뢰"),
            ],
        },
    }
}

/// Returns footer strings for the requested language.
pub fn get_footer_strings(lang: Language) -> FooterStrings {
    match lang {
        Language::En => FooterStrings {
            studio_desc: "boutique hardware & embedded laboratory",
            clearance_note: "all systems verified 0.00 mm³ physical clearance · seoul, korea",
            copyright: "© 2026 holt lab. all rights reserved.",
        },
        Language::Ko => FooterStrings {
            studio_desc: "부티크 하드웨어 및 임베디드 연구소",
            clearance_note: "전 시스템 0.00 mm³ 물리적 간섭 검증 완료 · 대한민국 서울",
            copyright: "© 2026 holt lab. 판권 소유.",
        },
    }
}
