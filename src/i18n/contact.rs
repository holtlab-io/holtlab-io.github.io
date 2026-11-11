//! ## Overview
//! Localized strings for the inquiry consultation and RFQ intake module.
//! Covers discipline selectors, form inputs, placeholders, and the submitted buffer view.
//!
//! ## Search Tags
//! #i18n, #contact, #rfq, #selectors, #buffer
//!
//! ## Submodules
//! None. This module provides atomic inquiry localization structures.

use super::Language;

/// Localized left column coordinates and SLA assurances.
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct ContactInfoStrings {
    /// Left column eyebrow text.
    pub eyebrow: &'static str,
    /// Left column title text.
    pub title: &'static str,
    /// Left column narrative body paragraph.
    pub body: &'static str,
    /// Email card label.
    pub email_label: &'static str,
    /// Studio location card label.
    pub location_label: &'static str,
    /// Studio location value.
    pub location_val: &'static str,
    /// Studio location badge.
    pub location_badge: &'static str,
    /// Response SLA card label.
    pub sla_label: &'static str,
    /// Response SLA value.
    pub sla_val: &'static str,
    /// Response SLA badge.
    pub sla_badge: &'static str,
}

/// Localized pill selector strings for engineering disciplines.
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct ContactSelectorStrings {
    /// Engineering discipline selector label.
    pub discipline_label: &'static str,
    /// 5 discipline scopes.
    pub scopes: [&'static str; 5],
}

/// Localized form field labels and placeholders.
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct ContactFormStrings {
    /// Name input label.
    pub name_label: &'static str,
    /// Name input placeholder.
    pub name_placeholder: &'static str,
    /// Email input label.
    pub contact_email_label: &'static str,
    /// Email input placeholder.
    pub contact_email_placeholder: &'static str,
    /// Timeline input label.
    pub timeline_label: &'static str,
    /// Timeline input placeholder.
    pub timeline_placeholder: &'static str,
    /// Budget envelope input label.
    pub budget_label: &'static str,
    /// Budget envelope input placeholder.
    pub budget_placeholder: &'static str,
    /// Project brief textarea label.
    pub brief_label: &'static str,
    /// Project brief textarea placeholder.
    pub brief_placeholder: &'static str,
    /// Submit button CTA text.
    pub submit_button: &'static str,
    /// Direct dispatch subtext.
    pub dispatch_note: &'static str,
}

/// Localized submitted buffer message and action buttons.
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct ContactBufferStrings {
    /// Submitted buffer badge.
    pub buffer_badge: &'static str,
    /// Submitted buffer title.
    pub buffer_title: &'static str,
    /// Submitted buffer description.
    pub buffer_desc: &'static str,
    /// Open mail client button text.
    pub btn_open_mail: &'static str,
    /// Edit specifications button text.
    pub btn_edit_specs: &'static str,
}

/// Localized aggregate string bundle for the contact and RFQ intake section.
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct ContactStrings {
    /// Left column info block.
    pub info: ContactInfoStrings,
    /// Selection buttons.
    pub selectors: ContactSelectorStrings,
    /// Form fields and inputs.
    pub form: ContactFormStrings,
    /// Buffer confirmation view.
    pub buffer: ContactBufferStrings,
}

fn info_en() -> ContactInfoStrings {
    ContactInfoStrings {
        eyebrow: "direct inquiry",
        title: "studio inquiry.",
        body: "we accept selected turnkey hardware commissions, multi-layer pcba layout, high-reliability embedded firmware, and end-to-end iot cloud integration. every inquiry is evaluated directly by our lead architect within 24 hours.",
        email_label: "direct contact // email",
        location_label: "studio base",
        location_val: "seoul, korea (remote worldwide)",
        location_badge: "REMOTE",
        sla_label: "response commitment",
        sla_val: "24h initial architectural review",
        sla_badge: "24H",
    }
}

fn selectors_en() -> ContactSelectorStrings {
    ContactSelectorStrings {
        discipline_label: "development discipline",
        scopes: [
            "turnkey hardware development",
            "enclosure & mechanical design",
            "electronic circuit & pcba",
            "embedded firmware",
            "iot & cloud / web integration",
        ],
    }
}

fn form_en() -> ContactFormStrings {
    ContactFormStrings {
        name_label: "name / organization",
        name_placeholder: "e.g. Acme Robotics / Jane Doe",
        contact_email_label: "contact email",
        contact_email_placeholder: "name@domain.com",
        timeline_label: "target timeline",
        timeline_placeholder: "e.g. 4-6 weeks to working prototype",
        budget_label: "target budget",
        budget_placeholder: "e.g. $5k-$15k USD / KRW 10M",
        brief_label: "project brief & requirements",
        brief_placeholder: "describe your product concept, target dimensions, operating environment, core features, battery/power requirements, and key deliverables...",
        submit_button: "dispatch inquiry",
        dispatch_note: "direct architect dispatch · 24h review",
    }
}

fn buffer_en() -> ContactBufferStrings {
    ContactBufferStrings {
        buffer_badge: "DISPATCH BUFFER READY",
        buffer_title: "inquiry ready for dispatch",
        buffer_desc: "your project specifications have been formatted. click below to open your mail client and dispatch directly to holt lab engineering.",
        btn_open_mail: "open mail client →",
        btn_edit_specs: "← edit specifications",
    }
}

fn info_ko() -> ContactInfoStrings {
    ContactInfoStrings {
        eyebrow: "직통 프로젝트 의뢰",
        title: "프로젝트 의뢰 및 기술 상담.",
        body: "엄선된 하드웨어 턴키 프로젝트, PCBA 회로 설계, 고신뢰성 임베디드 펌웨어, 기기-서버 IoT 클라우드 연동 의뢰를 진행합니다. 모든 의뢰서는 리드 하드웨어 아키텍트가 24시간 이내에 직접 기술 검토를 수행합니다.",
        email_label: "직통 연락처 // 이메일",
        location_label: "스튜디오 거점",
        location_val: "대한민국 서울 (원격 기술 협업 지원)",
        location_badge: "원격 지원",
        sla_label: "응답 보장 SLA",
        sla_val: "24시간 이내 1차 아키텍처 기술 검토",
        sla_badge: "24H",
    }
}

fn selectors_ko() -> ContactSelectorStrings {
    ContactSelectorStrings {
        discipline_label: "개발 분야 선택",
        scopes: [
            "턴키 하드웨어 개발",
            "외함 기구 설계",
            "전자 회로 기판",
            "임베디드 펌웨어",
            "IoT & 클라우드/웹 연동",
        ],
    }
}

fn form_ko() -> ContactFormStrings {
    ContactFormStrings {
        name_label: "성함 / 기업명",
        name_placeholder: "예: 에크미 로보틱스 / 홍길동",
        contact_email_label: "연락처 이메일",
        contact_email_placeholder: "name@domain.com",
        timeline_label: "희망 일정",
        timeline_placeholder: "예: 4~6주 내 시제품",
        budget_label: "희망 예산",
        budget_placeholder: "예: 1,000만 원 내외 / $10k",
        brief_label: "프로젝트 내용 및 요구사항",
        brief_placeholder: "제품 컨셉, 목표 치수, 주요 기능, 배터리 구동 환경, 최종 납품 희망 품목 등 프로젝트 요구사항을 자유롭게 설명해 주세요...",
        submit_button: "의뢰서 발송하기",
        dispatch_note: "리드 아키텍트 직통 전달 · 24시간 내 검토",
    }
}

fn buffer_ko() -> ContactBufferStrings {
    ContactBufferStrings {
        buffer_badge: "발송 버퍼 준비 완료",
        buffer_title: "의뢰서 발송 준비 완료",
        buffer_desc: "입력하신 프로젝트 요구사항이 정리되었습니다. 아래 버튼을 눌러 메일 프로그램을 열고 홀트 랩 엔지니어링 팀으로 직접 발송해주세요.",
        btn_open_mail: "메일 프로그램 열기 →",
        btn_edit_specs: "← 요구사항 수정하기",
    }
}

/// Returns the inquiry and RFQ strings for the requested language.
pub fn get_contact_strings(lang: Language) -> ContactStrings {
    match lang {
        Language::En => ContactStrings {
            info: info_en(),
            selectors: selectors_en(),
            form: form_en(),
            buffer: buffer_en(),
        },
        Language::Ko => ContactStrings {
            info: info_ko(),
            selectors: selectors_ko(),
            form: form_ko(),
            buffer: buffer_ko(),
        },
    }
}
