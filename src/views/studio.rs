//! ## Overview
//! Studio turnkey engineering architecture and methodology page view.
//! Explains the unified turnkey co-design philosophy, compares fragmented agencies against Holt Lab, and details the 4-phase pipeline.
//!
//! ## Search Tags
//! #studio, #methodology, #turnkey, #pipeline, #co-design
//!
//! ## Submodules
//! None. This module provides atomic view composition.

use dioxus::prelude::*;
use crate::components::{
    studio::StudioTurnkey,
    teasers::InquiryCtaBanner,
};
use crate::i18n::Language;

/// Dedicated studio turnkey architecture and methodology page view.
#[component]
pub fn StudioPage() -> Element {
    let lang = use_context::<Signal<Language>>();
    let (hero_title, hero_sub) = match lang() {
        Language::En => (
            "Unified Hardware & IoT Development Process",
            "How we take hardware concepts from system specification and 3D CAD/PCBA design through physical prototype assembly, firmware bring-up, and server integration.",
        ),
        Language::Ko => (
            "통합 하드웨어 · IoT 개발 프로세스",
            "요구사항 정의와 부품 선정부터 기구·다층 기판 통합 설계, 시제품 실물 조립 점검, 펌웨어 브링업 및 백엔드 서버 연동까지 이어지는 엔지니어링 절차를 소개합니다.",
        ),
    };

    rsx! {
        div {
            class: "w-full",
            section {
                id: "studio-hero",
                class: "pt-8 sm:pt-16 pb-6 sm:pb-8 bg-[#FAFAFA] border-b border-zinc-200 text-center font-sans relative overflow-hidden",
                div { class: "absolute inset-0 bg-tech-dots opacity-35 pointer-events-none" }
                div {
                    class: "max-w-4xl mx-auto px-4 sm:px-8 relative z-10 space-y-4 reveal-on-scroll",
                    div {
                        class: "inline-flex items-center gap-2 px-3 py-1 rounded-full bg-white border border-zinc-200 text-zinc-700 font-mono text-[11px] font-semibold tracking-wider uppercase shadow-2xs",
                        span { class: "w-2 h-2 rounded-full bg-[#E64A19] animate-pulse" }
                        match lang() {
                            Language::En => "STUDIO ARCHITECTURE // METHODOLOGY",
                            Language::Ko => "스튜디오 아키텍처 // 엔지니어링 방법론",
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

            StudioTurnkey {}
            InquiryCtaBanner {}
        }
    }
}
