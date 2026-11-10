//! ## Overview
//! Localized presentation strings for studio teasers, summary cards, and call-to-action banners.
//! Provides bilingual English and Korean copy for home page navigation previews.
//!
//! ## Search Tags
//! #i18n, #teasers, #summary, #cta, #banner, #branding
//!
//! ## Submodules
//! None. This module provides atomic presentation text models.

use super::Language;

/// Localized string bundle for the demonstrator teaser preview card.
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct DemonstratorTeaserStrings {
    /// Top eyebrow badge text.
    pub eyebrow: &'static str,
    /// Demonstrator headline.
    pub title: &'static str,
    /// Demonstrator description summary.
    pub subtitle: &'static str,
    /// 4 key engineering highlight badges.
    pub pills: [&'static str; 4],
    /// Call-to-action button label.
    pub cta: &'static str,
}

/// Localized string bundle for the studio turnkey methodology teaser.
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct StudioTeaserStrings {
    /// Top eyebrow badge text.
    pub eyebrow: &'static str,
    /// Value proposition headline.
    pub title: &'static str,
    /// Engineering methodology summary.
    pub subtitle: &'static str,
    /// Call-to-action button label.
    pub cta: &'static str,
}

/// Localized string bundle for the direct inquiry banner.
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct InquiryBannerStrings {
    /// Top eyebrow badge text.
    pub eyebrow: &'static str,
    /// Monumental inquiry headline.
    pub title: &'static str,
    /// Consultation dispatch guidance text.
    pub subtitle: &'static str,
    /// Call-to-action button label.
    pub cta: &'static str,
}

/// Complete bundle of home page teaser strings.
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct HomeTeaserBundle {
    /// Flagship demonstrator teaser.
    pub demonstrator: DemonstratorTeaserStrings,
    /// Turnkey methodology teaser.
    pub studio: StudioTeaserStrings,
    /// Direct inquiry dispatch banner.
    pub inquiry: InquiryBannerStrings,
}

fn teasers_en() -> HomeTeaserBundle {
    HomeTeaserBundle {
        demonstrator: DemonstratorTeaserStrings {
            eyebrow: "FLAGSHIP DEMONSTRATOR // OPUS 01",
            title: "Win95 Minesweeper Touch Keyring",
            subtitle: "Ultra-compact interactive pocket demonstrator integrating ESP32-C3, ST7789T3 2.0\" 60Hz IPS, CST816D capacitive touch, and real-time embedded firmware in a 43.4×69.2×13.78mm HP MJF PA12 enclosure.",
            pills: [
                "HP MJF PA12 Enclosure",
                "4-Layer Controlled-Z PCBA",
                "AHB GDMA 60Hz Zero-Copy",
                "CST816D Capacitive Touch",
            ],
            cta: "inspect demonstrator // 실증 머신 상세 분석 →",
        },
        studio: StudioTeaserStrings {
            eyebrow: "WHY UNIFIED TURNKEY?",
            title: "Zero agency ping-pong. Single deterministic loop.",
            subtitle: "Mechanical CAD, 4-Layer ECAD, and Embedded Firmware engineered concurrently to achieve first-spin physical bench success without intermediate agency friction.",
            cta: "explore studio methodology // 턴키 모델 보기 →",
        },
        inquiry: InquiryBannerStrings {
            eyebrow: "DIRECT ARCHITECT DISPATCH",
            title: "Ready to engineer physical hardware?",
            subtitle: "Submit your project requirements, target schedule, and budget range for rapid feasibility assessment and architectural review within 24 hours.",
            cta: "submit project brief // 의뢰서 작성하기 →",
        },
    }
}

fn teasers_ko() -> HomeTeaserBundle {
    HomeTeaserBundle {
        demonstrator: DemonstratorTeaserStrings {
            eyebrow: "플래그십 실증 머신 // OPUS 01",
            title: "Win95 마인스위퍼 터치 키링",
            subtitle: "ESP32-C3, ST7789T3 2.0인치 60Hz IPS, CST816D 정전식 터치, 초저지연 임베디드 펌웨어를 43.4×69.2×13.78mm HP MJF PA12 외함에 집약한 초소형 실증 머신.",
            pills: [
                "MJF PA12 정밀 외함",
                "4층 제어 임피던스 회로",
                "AHB GDMA 60Hz 전송",
                "CST816D 정전식 터치",
            ],
            cta: "실증 머신 상세 분석 // teardown →",
        },
        studio: StudioTeaserStrings {
            eyebrow: "왜 통합 턴키인가?",
            title: "외주 핑퐁 없는 단일 결정론적 코디자인 루프.",
            subtitle: "기구 CAD, 4층 임피던스 제어 회로, 임베디드 펌웨어를 단일 팀에서 동시 설계하여 첫 번째 시제품에서 즉시 구동을 검증합니다.",
            cta: "턴키 모델 보기 // studio →",
        },
        inquiry: InquiryBannerStrings {
            eyebrow: "리드 엔지니어 직통 의뢰",
            title: "실물로 작동하는 하드웨어를 제작할 준비가 되셨습니까?",
            subtitle: "프로젝트 요구 사양, 목표 일정, 예산 범위를 전달해주시면 24시간 이내에 아키텍처 검토 의견을 회신합니다.",
            cta: "의뢰서 작성하기 // inquire →",
        },
    }
}

/// Returns the home teaser string bundle for the requested language.
pub fn get_home_teaser_bundle(lang: Language) -> HomeTeaserBundle {
    match lang {
        Language::En => teasers_en(),
        Language::Ko => teasers_ko(),
    }
}
