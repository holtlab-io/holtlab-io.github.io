//! ## Overview
//! Quantitative engineering metrics bar for the Holt Lab landing page.
//! Displays key validation metrics: zero CAD collision, 1:1 direct architecture, rapid prototype cycles, and grant compliance.
//!
//! ## Search Tags
//! #metrics, #accuracy, #tolerance, #turnkey, #standards

use dioxus::prelude::*;

/// Section displaying core engineering performance metrics.
#[component]
pub fn Metrics() -> Element {
    rsx! {
        section {
            class: "py-10 border-y border-zinc-200 bg-[#F4F4F5] relative font-mono",
            div {
                class: "max-w-7xl mx-auto px-4 sm:px-6 lg:px-8",
                div {
                    class: "grid grid-cols-2 lg:grid-cols-4 gap-4 sm:gap-6",
                    MetricCard {
                        value: "0.00 mm³",
                        tag: "CAD COLLISION VOLUME",
                        desc: "오픈캐스케이드 B-Rep 솔리드와 4층 회로 3D STEP을 0.01mm 오차 없이 정합하여 기구 간섭을 원천 차단합니다."
                    }
                    MetricCard {
                        value: "1:1 DIRECT",
                        tag: "LEAD ARCHITECT TURNKEY",
                        desc: "기구·회로·펌웨어 분리 외주의 책임 전가 핑퐁을 없애고, 단일 엔지니어가 전체 품질을 1:1로 책임집니다."
                    }
                    MetricCard {
                        value: "3~4 WEEKS",
                        tag: "RAPID WORKING PROTOTYPE",
                        desc: "자동화된 공장 DFM 검증과 원스톱 직발주(JLCPCB/JLC3DP)로 투자 유치 및 과제 마감 일정에 맞춰 완제품을 납품합니다."
                    }
                    MetricCard {
                        value: "100% AUDIT",
                        tag: "GOVERNMENT GRANT COMPLIANT",
                        desc: "예창패, 초창패, 청창사 등 정부지원사업 맞춤 전자계약, 세금계산서, 회로도/도면/펌웨어 전수 검수보고서를 지원합니다."
                    }
                }
            }
        }
    }
}

/// Single metric item card.
#[component]
fn MetricCard(value: &'static str, tag: &'static str, desc: &'static str) -> Element {
    rsx! {
        div {
            class: "p-4 sm:p-5 rounded-lg bg-white border border-zinc-200 hover:border-zinc-300 shadow-2xs transition-colors space-y-1.5",
            div {
                class: "text-2xl sm:text-3xl font-bold tracking-tight text-zinc-900",
                "{value}"
            }
            div {
                class: "text-[11px] font-bold text-zinc-500 uppercase tracking-wider",
                "{tag}"
            }
            p {
                class: "text-xs text-zinc-600 font-sans leading-relaxed pt-1",
                "{desc}"
            }
        }
    }
}
