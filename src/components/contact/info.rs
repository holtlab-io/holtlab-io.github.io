//! ## Overview
//! Contact info and value assurance column presenting Holt Lab direct communication channels and SLAs.
//!
//! ## Search Tags
//! #contact, #info, #sla, #guarantee, #email

use dioxus::prelude::*;

/// Left column displaying direct contact information and institutional assurances.
#[component]
pub fn ContactInfo() -> Element {
    rsx! {
        div {
            class: "lg:col-span-5 space-y-6",
            div {
                class: "space-y-3",
                div {
                    class: "inline-flex items-center gap-2 px-2.5 py-1 rounded-full bg-zinc-100 border border-zinc-200 text-zinc-600 text-xs tracking-wider uppercase",
                    "PROJECT RFQ // DIRECT INQUIRY"
                }
                h2 {
                    class: "text-2xl sm:text-4xl font-bold text-zinc-900 tracking-tight font-sans",
                    "아이디어만 들고 오셔도 "
                    br {}
                    span { class: "text-zinc-500", "실물로 완성해 드립니다." }
                }
                p {
                    class: "text-zinc-600 text-xs sm:text-sm font-sans leading-relaxed",
                    "기획서나 도면이 완벽하지 않아도 괜찮습니다. 요구사항을 남겨주시면 "
                    span { class: "text-zinc-900 font-medium", "24시간 이내에 리드 엔지니어가 직접" }
                    " 물리적 실현 가능성, 부품 수급성, 예상 일정과 예산을 검토해 회신드립니다."
                }
            }

            // Direct Info Technical Cards
            div {
                class: "space-y-3 text-xs",
                div {
                    class: "p-4 rounded-xl border border-zinc-200 bg-white flex items-center justify-between shadow-2xs",
                    div {
                        div { class: "text-zinc-500 text-[10px] uppercase font-semibold", "DIRECT CONTACT // EMAIL" }
                        a {
                            href: "mailto:contact@holtlab.io",
                            class: "text-sm font-bold text-zinc-900 hover:text-zinc-950 hover:underline transition-colors mt-0.5 block",
                            "contact@holtlab.io"
                        }
                    }
                    span { class: "text-zinc-400 text-xs font-mono", "→" }
                }

                div {
                    class: "p-4 rounded-xl border border-zinc-200 bg-white flex items-center justify-between shadow-2xs",
                    div {
                        div { class: "text-zinc-500 text-[10px] uppercase font-semibold", "GOVERNMENT GRANT // TAX COMPLIANCE" }
                        div { class: "text-xs font-semibold text-zinc-800 mt-0.5", "전자계약 및 전자세금계산서 100% 발행 지원" }
                    }
                    span { class: "text-zinc-900 text-xs font-bold font-mono", "PASS" }
                }

                div {
                    class: "p-4 rounded-xl border border-zinc-200 bg-white flex items-center justify-between shadow-2xs",
                    div {
                        div { class: "text-zinc-500 text-[10px] uppercase font-semibold", "SLA // RESPONSE GUARANTEE" }
                        div { class: "text-xs font-semibold text-zinc-800 mt-0.5", "영업일 기준 24시간 이내 1차 기술 검토 완료" }
                    }
                    span { class: "text-zinc-600 text-xs font-mono font-medium", "24H" }
                }
            }
        }
    }
}
