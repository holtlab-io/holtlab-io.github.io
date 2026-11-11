//! ## Overview
//! Dedicated client RFQ intake and engineering diagnostic consultation page view.
//! Connects prospective clients directly with Holt Lab's lead hardware architect.
//!
//! ## Search Tags
//! #contact, #rfq, #intake, #consultation, #brief
//!
//! ## Submodules
//! None. This module provides atomic view composition.

use dioxus::prelude::*;
use crate::components::contact::Contact;
use crate::i18n::Language;

/// Dedicated client consultation and project brief submission page view.
#[component]
pub fn ContactPage() -> Element {
    let lang = use_context::<Signal<Language>>();
    let (hero_title, hero_sub) = match lang() {
        Language::En => (
            "Project Consultation & RFQ",
            "Share your hardware or IoT product concept, required engineering scope, and target schedule. Our engineering team reviews every brief directly within 24 hours.",
        ),
        Language::Ko => (
            "프로젝트 의뢰 및 기술 상담",
            "구상 중인 하드웨어 또는 IoT 시스템의 핵심 기능, 필요한 개발 범위, 희망 일정을 남겨주세요. 담당 엔지니어가 기술 타당성을 검토해 영업일 기준 24시간 이내에 회신드립니다.",
        ),
    };

    rsx! {
        div {
            class: "w-full",
            section {
                id: "contact-hero",
                class: "pt-12 sm:pt-16 pb-8 bg-[#FAFAFA] border-b border-zinc-200 text-center font-sans relative overflow-hidden",
                div { class: "absolute inset-0 bg-tech-grid opacity-35 pointer-events-none" }
                div {
                    class: "max-w-4xl mx-auto px-4 sm:px-8 relative z-10 space-y-4 reveal-on-scroll",
                    div {
                        class: "inline-flex items-center gap-2 px-3 py-1 rounded-full bg-white border border-zinc-200 text-zinc-700 font-mono text-[11px] font-semibold tracking-wider uppercase shadow-2xs",
                        span { class: "w-2 h-2 rounded-full bg-[#E64A19] animate-pulse" }
                        match lang() {
                            Language::En => "CLIENT RFQ INTAKE // CONSULTATION",
                            Language::Ko => "프로젝트 문의 // 기술 상담",
                        }
                    }
                    h1 {
                        class: "text-3xl sm:text-5xl font-bold font-sans tracking-tight text-zinc-950 leading-tight break-keep",
                        "{hero_title}"
                    }
                    p {
                        class: "text-sm sm:text-base text-zinc-600 font-normal leading-relaxed max-w-2xl mx-auto break-keep",
                        "{hero_sub}"
                    }
                }
            }

            Contact {}
        }
    }
}
