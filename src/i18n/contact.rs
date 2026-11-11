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
        eyebrow: "COLLABORATION & DELIVERABLES",
        title: "Scope & Handover Guide",
        body: "From full turnkey prototypes to individual modules (enclosure CAD, PCBA layout, firmware, or IoT backend & web dashboard), all design sources — 3D STEP, schematics, Gerbers, BOM, and software repositories — are cleanly handed over upon milestone completion.",
        email_label: "DIRECT EMAIL",
        location_label: "STUDIO LOCATION",
        location_val: "Seoul, Republic of Korea (Worldwide Remote)",
        location_badge: "SEOUL, KR",
        sla_label: "RESPONSE TIME",
        sla_val: "Initial engineering review within 24 hours",
        sla_badge: "24H",
    }
}

fn selectors_en() -> ContactSelectorStrings {
    ContactSelectorStrings {
        discipline_label: "Development Scope",
        scopes: [
            "Turnkey Hardware Prototype",
            "Mechanical & Enclosure",
            "Custom Circuit & PCBA",
            "Embedded Firmware",
            "IoT Server & Web Integration",
        ],
    }
}

fn form_en() -> ContactFormStrings {
    ContactFormStrings {
        name_label: "Name / Organization",
        name_placeholder: "e.g. Horizon Labs / Alex Kim",
        contact_email_label: "Contact Email",
        contact_email_placeholder: "name@company.com",
        timeline_label: "Target Timeline",
        timeline_placeholder: "e.g. 6–8 weeks to prototype",
        budget_label: "Estimated Budget",
        budget_placeholder: "e.g. $8k–$20k USD / Flexible",
        brief_label: "Project Brief & Requirements",
        brief_placeholder: "Describe your product concept, core features, target size, power source (battery or wired), cloud/server connectivity needs, and expected deliverables...",
        submit_button: "Submit Inquiry",
        dispatch_note: "Instant web submission · 24h engineering review",
    }
}

fn buffer_en() -> ContactBufferStrings {
    ContactBufferStrings {
        buffer_badge: "INQUIRY RECEIVED",
        buffer_title: "Your Inquiry Has Been Received",
        buffer_desc: "Your project brief has been transmitted directly to Holt Lab. Our engineering team will reply to your email within 24 business hours. To share drawings or reference files, click below to attach via email.",
        btn_open_mail: "Attach Files via Email →",
        btn_edit_specs: "← Edit or Resend",
    }
}

fn info_ko() -> ContactInfoStrings {
    ContactInfoStrings {
        eyebrow: "협업 및 산출물 안내",
        title: "상담 안내 및 진행 방식",
        body: "전체 턴키 시제품 제작은 물론 기구 설계, 회로·PCBA, 펌웨어, IoT 백엔드 서버 및 웹 대시보드 등 필요한 영역만 선택하여 의뢰하실 수 있습니다. 완료 시 3D STEP, 회로도·거버·BOM, 소스코드 등 원본 산출물을 인도하며 표준 전자계약 및 기업 정산 프로세스를 지원합니다.",
        email_label: "이메일 문의",
        location_label: "스튜디오 소재지",
        location_val: "대한민국 서울 (전국 · 글로벌 원격 협업)",
        location_badge: "SEOUL, KR",
        sla_label: "검토 및 회신",
        sla_val: "영업일 기준 24시간 이내 1차 기술 검토 회신",
        sla_badge: "24H",
    }
}

fn selectors_ko() -> ContactSelectorStrings {
    ContactSelectorStrings {
        discipline_label: "의뢰 분야 선택",
        scopes: [
            "턴키 시제품 개발",
            "기구 · 외함 설계",
            "전자 회로 · PCBA",
            "임베디드 펌웨어",
            "IoT 서버 · 웹 연동",
        ],
    }
}

fn form_ko() -> ContactFormStrings {
    ContactFormStrings {
        name_label: "성함 / 소속 (기업·팀명)",
        name_placeholder: "예: (주)테크스타트업 / 홍길동",
        contact_email_label: "회신받을 이메일",
        contact_email_placeholder: "name@company.com",
        timeline_label: "희망 일정",
        timeline_placeholder: "예: 6~8주 내 1차 시제품 희망",
        budget_label: "예상 예산 범위",
        budget_placeholder: "예: 1,000만 원 내외 / 협의 가능",
        brief_label: "프로젝트 개요 및 요구사항",
        brief_placeholder: "구현하고자 하는 제품이나 시스템의 핵심 기능, 예상 크기, 전원 환경(배터리/상시전원), 서버·웹 연동 필요 여부, 희망 산출물 등을 편하게 적어주세요.",
        submit_button: "프로젝트 문의 접수하기",
        dispatch_note: "별도 메일 앱 없이 즉시 접수 · 24시간 내 회신",
    }
}

fn buffer_ko() -> ContactBufferStrings {
    ContactBufferStrings {
        buffer_badge: "문의 접수 완료",
        buffer_title: "프로젝트 문의가 정상적으로 접수되었습니다",
        buffer_desc: "입력하신 요구사항이 스튜디오로 즉시 전달되었습니다. 담당 엔지니어가 검토 후 영업일 기준 24시간 이내에 이메일로 회신드립니다. 도면·참고 사진 등 첨부파일이 있으시면 아래 버튼으로 추가 발송하실 수 있습니다.",
        btn_open_mail: "첨부파일 · 메일 추가 발송 →",
        btn_edit_specs: "← 내용 수정 · 재접수",
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
