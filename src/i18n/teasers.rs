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
    /// Visual card status badge.
    pub status_badge: &'static str,
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
    /// Showcase case study teaser.
    pub demonstrator: DemonstratorTeaserStrings,
    /// Turnkey methodology teaser.
    pub studio: StudioTeaserStrings,
    /// Direct inquiry dispatch banner.
    pub inquiry: InquiryBannerStrings,
}

fn teasers_en() -> HomeTeaserBundle {
    HomeTeaserBundle {
        demonstrator: DemonstratorTeaserStrings {
            eyebrow: "ENGINEERING SHOWCASE // CASE STUDY 01",
            title: "Win95 Minesweeper Touch Keyring",
            subtitle: "A compact device case study integrating an ESP32-C3 wireless MCU, 2.0\" IPS display, capacitive touch panel, and LiPo power protection into a 43.4×69.2×13.78mm MJF PA12 enclosure — verified through physical assembly and firmware bring-up.",
            pills: [
                "HP MJF PA12 Enclosure",
                "4-Layer Controlled-Z PCBA",
                "60Hz DMA Display Pipeline",
                "Capacitive Touch & Sleep",
            ],
            status_badge: "ASSEMBLED & VERIFIED",
            cta: "Explore Case Study →",
        },
        studio: StudioTeaserStrings {
            eyebrow: "WHY UNIFIED ENGINEERING?",
            title: "Hardware, firmware, and cloud engineered without vendor silos.",
            subtitle: "Splitting enclosures, PCBs, firmware, and backend servers across separate contractors causes interface mismatches and costly rework. We align mechanical fit, circuit design, and software protocols from day one through hands-on prototype validation.",
            cta: "Explore Engineering Process →",
        },
        inquiry: InquiryBannerStrings {
            eyebrow: "TECHNICAL CONSULTATION",
            title: "Ready to build your custom hardware or IoT product?",
            subtitle: "Share your concept, required scope (turnkey prototype, enclosure, PCBA, firmware, or IoT server integration), and target schedule for a direct engineering feasibility review within 24 hours.",
            cta: "Start Project Inquiry →",
        },
    }
}

fn teasers_ko() -> HomeTeaserBundle {
    HomeTeaserBundle {
        demonstrator: DemonstratorTeaserStrings {
            eyebrow: "기술 쇼케이스 // CASE STUDY 01",
            title: "Win95 마인스위퍼 터치 키링",
            subtitle: "ESP32-C3 무선 MCU, 2.0인치 IPS 디스플레이, 정전식 터치 패널, 리튬폴리머 충전·보호 회로를 43.4×69.2×13.78mm MJF 나일론 외함에 통합 설계하고 실물 조립과 펌웨어 브링업까지 완료한 소형 디바이스 제작 사례입니다.",
            pills: [
                "MJF PA12 기구 외함",
                "4층 제어 임피던스 기판",
                "60Hz DMA 그래픽 처리",
                "정전식 터치 & 저전력 슬립",
            ],
            status_badge: "실물 조립 · 구동 검증",
            cta: "케이스 스터디 자세히 보기 →",
        },
        studio: StudioTeaserStrings {
            eyebrow: "왜 통합 개발인가?",
            title: "기구·회로·펌웨어·서버의 분절 없는 통합 엔지니어링",
            subtitle: "기구, 기판, 펌웨어, 서버를 각기 다른 외주사에 맡기며 발생하는 설계 불일치와 책임 공방을 줄입니다. 설계 초기부터 부품 규격과 통신 구조를 함께 맞추고, 실물 시제품 조립·검증을 거쳐 완성도를 높입니다.",
            cta: "개발 프로세스 보기 →",
        },
        inquiry: InquiryBannerStrings {
            eyebrow: "프로젝트 기술 상담",
            title: "아이디어를 실물 하드웨어로 구현할 준비가 되셨나요?",
            subtitle: "제품 컨셉, 필요한 개발 범위(턴키 시제품·기구·회로·펌웨어·IoT 서버), 희망 일정을 남겨주시면 담당 엔지니어가 기술 검토 후 영업일 기준 24시간 이내에 회신드립니다.",
            cta: "프로젝트 문의하기 →",
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
