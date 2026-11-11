//! ## Overview
//! Localized strings for the studio hero section, showcase headers, navigation bars, and footer credits.
//! Repositions Holt Lab as a full-stack hardware, embedded, and IoT engineering studio with bilingual parity.
//!
//! ## Search Tags
//! #i18n, #hero, #navbar, #footer, #branding, #studio
//!
//! ## Submodules
//! None. This module provides atomic presentation text models.

use super::Language;

/// Localized string bundle for the hero studio section and showcase console.
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct HeroStrings {
    /// Top eyebrow badge text.
    pub eyebrow: &'static str,
    /// Monumental headline text.
    pub title: &'static str,
    /// Engineering subtitle paragraph for studio overview.
    pub subtitle: &'static str,
    /// 4 studio-wide discipline pills.
    pub pills: [&'static str; 4],
    /// Home hero preview status label.
    pub hero_preview_status: &'static str,
    /// Home hero preview badge tag.
    pub hero_preview_tag: &'static str,
    /// Home hero preview bottom bar items.
    pub hero_preview_notes: [&'static str; 3],
    /// Primary visual boundary width indicator for case study workbench.
    pub width_label: &'static str,
    /// Center visual material indicator for case study workbench.
    pub material_label: &'static str,
    /// Visual boundary height indicator for case study workbench.
    pub height_label: &'static str,
    /// Case study jump CTA button text.
    pub cta_casestudy: &'static str,
    /// Inquiry jump CTA button text.
    pub cta_inquire: &'static str,
    /// Console workbench status banner.
    pub console_status: &'static str,
    /// Case study headline caption.
    pub console_sub: &'static str,
    /// Dedicated case study subtitle on `/demonstrator`.
    pub case_study_subtitle: &'static str,
    /// Viewport inspection mode configurations (id, tab_label, telemetry, image_src).
    pub console_modes: [(&'static str, &'static str, &'static str, &'static str); 4],
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
    pub nav_items: [(&'static str, &'static str, &'static str); 4],
    /// Mobile drawer anchor items with subtitle descriptions.
    pub mobile_items: [(&'static str, &'static str, &'static str); 4],
}

/// Localized string bundle for footer credits and copyright.
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct FooterStrings {
    /// Studio subtitle descriptor in footer.
    pub studio_desc: &'static str,
    /// Location note.
    pub clearance_note: &'static str,
    /// Copyright statement.
    pub copyright: &'static str,
}

fn console_modes_en() -> [(&'static str, &'static str, &'static str, &'static str); 4] {
    [
        ("iso", "01 // 3D ISO", "PERSPECTIVE: 3D PROJECTION // SOLID ASSEMBLY", "/assets/showcase/mcad_iso.png"),
        ("front", "02 // FRONT BEZEL", "ELEVATION: FRONT CHASSIS // FLUSH 0.85mm BEZEL", "/assets/showcase/mcad_front.png"),
        ("section", "03 // INTERNAL SECTION", "ANATOMY: CROSS-SECTION // INTERNAL STACKUP FIT", "/assets/showcase/mcad_section.png"),
        ("drawing", "04 // CAD WIREFRAME", "ENGINEERING: 2D WIREFRAME // 3D STEP CLEARANCE CHECK", "/assets/showcase/mcad_drawing.png"),
    ]
}

fn console_modes_ko() -> [(&'static str, &'static str, &'static str, &'static str); 4] {
    [
        ("iso", "01 // 3D 아이소", "투영 모드: 3D 입체 조립도 // 외함 및 부품 배치", "/assets/showcase/mcad_iso.png"),
        ("front", "02 // 전면 베젤", "정면도: 전면 외함 // 0.85mm 단차 플러시 베젤", "/assets/showcase/mcad_front.png"),
        ("section", "03 // 내부 단면", "단면도: 내부 적층 구조 // 부품 간 유격 및 결합 확인", "/assets/showcase/mcad_section.png"),
        ("drawing", "04 // CAD 도면", "도면 모드: 기구 와이어프레임 // 3D STEP 치수 검증", "/assets/showcase/mcad_drawing.png"),
    ]
}

fn hero_en() -> HeroStrings {
    HeroStrings {
        eyebrow: "HARDWARE, EMBEDDED & IOT ENGINEERING STUDIO",
        title: "From Circuit & Enclosure to Cloud. We Engineer Physical Hardware.",
        subtitle: "Mechanical enclosures, custom multi-layer PCBA, embedded firmware, and connected backend servers engineered under one roof — reducing cross-vendor mismatches and delivering physically verified prototypes.",
        pills: [
            "01 // Mechanical & Enclosures",
            "02 // Custom Circuit & PCBA",
            "03 // Embedded Firmware",
            "04 // IoT Server & Web",
        ],
        hero_preview_status: "INTEGRATED R&D SHOWCASE // MECHANICAL · PCBA · FIRMWARE VERIFIED",
        hero_preview_tag: "CASE STUDY 01 // COMPACT TOUCH DEVICE",
        hero_preview_notes: [
            "3D Fit & Clearance Verified",
            "Controlled-Impedance PCBA · Custom Enclosure",
            "Real-Time Low-Power Firmware",
        ],
        width_label: "DIMENSION: 43.40 × 69.20 mm",
        material_label: "HP MJF PA12 // 4-LAYER PCBA",
        height_label: "STACKUP: 13.78 mm",
        cta_casestudy: "Explore Case Study",
        cta_inquire: "Inquire Project",
        console_status: "ARCHITECTURE STATUS: 3D FIT CHECK & PHYSICAL BRING-UP VERIFIED",
        console_sub: "Win95 Minesweeper Touch Keyring",
        case_study_subtitle: "An end-to-end engineering case study integrating a custom MJF nylon enclosure, 4-layer controlled-impedance PCBA, capacitive touch IPS display, and real-time embedded firmware into a pocket-sized form factor.",
        console_modes: console_modes_en(),
    }
}

fn hero_ko() -> HeroStrings {
    HeroStrings {
        eyebrow: "하드웨어 · 임베디드 · IoT 통합 엔지니어링 스튜디오",
        title: "회로와 기구 외함부터 서버 연동까지, 실물로 작동하는 하드웨어를 설계합니다.",
        subtitle: "기구 외함 설계, 맞춤형 회로 기판(PCBA), 임베디드 펌웨어, 그리고 기기와 통신하는 백엔드 서버·웹 대시보드까지 한곳에서 통합 개발합니다. 외주 분절로 인한 설계 불일치를 줄이고 실제 조립·구동 검증을 마친 시제품을 완성합니다.",
        pills: [
            "01 // 기구 및 외함 설계",
            "02 // 맞춤형 회로 · PCBA",
            "03 // 임베디드 펌웨어",
            "04 // IoT 서버 · 웹 연동",
        ],
        hero_preview_status: "통합 하드웨어 개발 실증 // 기구 · 회로 · 펌웨어 실물 구동 완료",
        hero_preview_tag: "CASE STUDY 01 // 소형 터치 디바이스",
        hero_preview_notes: [
            "3D 부품 간섭 사전 검증",
            "다층 제어 임피던스 PCBA · 맞춤형 외함",
            "실시간 저전력 펌웨어 구동",
        ],
        width_label: "외형 규격: 43.40 × 69.20 mm",
        material_label: "HP MJF PA12 // 4층 회로 기판",
        height_label: "두께 규격: 13.78 mm",
        cta_casestudy: "기술 쇼케이스 보기",
        cta_inquire: "프로젝트 문의하기",
        console_status: "설계 및 검증 상태: 3D 기구 간섭 검증 · 실물 보드 브링업 완료",
        console_sub: "Win95 마인스위퍼 터치 키링",
        case_study_subtitle: "소형 폼팩터 안에 맞춤형 MJF 나일론 외함, 4층 제어 임피던스 기판, 정전식 터치 IPS 디스플레이, 저전력 실시간 펌웨어를 통합 설계하고 실물 조립·브링업까지 검증한 기술 실증 프로젝트입니다.",
        console_modes: console_modes_ko(),
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
            descriptor: "Hardware, Embedded & IoT Engineering Studio",
            turnkey_badge: "UNIFIED HARDWARE · FIRMWARE · IOT",
            cta_inquire: "Inquire",
            nav_items: [
                ("/", "home", "01"),
                ("/demonstrator", "showcase", "02"),
                ("/studio", "studio", "03"),
                ("/contact", "inquire", "04"),
            ],
            mobile_items: [
                ("/", "Home", "Studio overview & 4 core capabilities"),
                ("/demonstrator", "Showcase", "Compact touch hardware case study"),
                ("/studio", "Studio", "Unified engineering workflow"),
                ("/contact", "Inquire", "Project consultation & RFQ"),
            ],
        },
        Language::Ko => NavStrings {
            descriptor: "하드웨어 · 임베디드 · IoT 엔지니어링 스튜디오",
            turnkey_badge: "기구 · 회로 · 펌웨어 · 서버 통합 개발",
            cta_inquire: "문의하기",
            nav_items: [
                ("/", "홈", "01"),
                ("/demonstrator", "쇼케이스", "02"),
                ("/studio", "개발 방식", "03"),
                ("/contact", "문의하기", "04"),
            ],
            mobile_items: [
                ("/", "홈", "스튜디오 소개 및 4대 핵심 역량"),
                ("/demonstrator", "쇼케이스", "소형 터치 디바이스 통합 설계 사례"),
                ("/studio", "개발 방식", "통합 하드웨어 개발 프로세스"),
                ("/contact", "문의하기", "프로젝트 상담 및 견적 문의"),
            ],
        },
    }
}

/// Returns footer strings for the requested language.
pub fn get_footer_strings(lang: Language) -> FooterStrings {
    match lang {
        Language::En => FooterStrings {
            studio_desc: "Hardware, Embedded & IoT Engineering Studio",
            clearance_note: "Seoul, Republic of Korea",
            copyright: "© 2026 Holt Lab. All rights reserved.",
        },
        Language::Ko => FooterStrings {
            studio_desc: "하드웨어 · 임베디드 · IoT 엔지니어링 스튜디오",
            clearance_note: "대한민국 서울",
            copyright: "© 2026 Holt Lab. All rights reserved.",
        },
    }
}
