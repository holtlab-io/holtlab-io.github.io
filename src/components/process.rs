//! ## Overview
//! Deterministic 4-stage engineering verification protocol for hardware execution.
//! Details Gate 0 COTS Lock, Gate 1 3D Collision Audit, Gate 2 Factory DFM, and Gate 3 Physical Bring-up.
//!
//! ## Search Tags
//! #process, #gates, #verification, #dfm, #bringup

use dioxus::prelude::*;

/// 4-stage deterministic engineering verification gate section.
#[component]
pub fn Process() -> Element {
    rsx! {
        section {
            id: "process",
            class: "py-20 bg-[#F4F4F5] relative border-b border-zinc-200 font-mono",
            div {
                class: "max-w-7xl mx-auto px-4 sm:px-6 lg:px-8",
                // Header
                div {
                    class: "max-w-3xl space-y-3 mb-12",
                    div {
                        class: "inline-flex items-center gap-2 px-2.5 py-1 rounded-full bg-zinc-200/60 border border-zinc-300/80 text-zinc-700 text-xs tracking-wider uppercase",
                        "DETERMINISTIC VERIFICATION PROTOCOL"
                    }
                    h2 {
                        class: "text-2xl sm:text-4xl font-bold text-zinc-900 tracking-tight font-sans",
                        "실패 없는 하드웨어 제작 "
                        br {}
                        span { class: "text-zinc-500", "4단계 엔지니어링 검증 게이트" }
                    }
                    p {
                        class: "text-zinc-600 text-sm sm:text-base leading-relaxed font-sans",
                        "주먹구구식 빵판 회로가 아닙니다. 공장 제조 규격과 3D 물리 정합성을 사전에 검증하여 "
                        span { class: "text-zinc-900 font-medium", "첫 번째 공장 발주부터 바로 동작하는 시제품" }
                        "을 만듭니다."
                    }
                }

                // 4-Step Process Pipeline
                div {
                    class: "grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-4",
                    GateCard {
                        stage: "STAGE 00",
                        gate: "GATE 0 LOCK",
                        title: "COTS 부품 & 스펙 동결",
                        desc: "MCU, 디스플레이, 배터리, 커넥터 등 핵심 부품의 데이터시트와 3D STEP 모델을 먼저 확보하고 수급 안정성을 검증합니다.",
                        deliverable: "산출물: 부품 사양서 & 3D STEP 락"
                    }
                    GateCard {
                        stage: "STAGE 01",
                        gate: "GATE 1 AUDIT",
                        title: "회로 제도 & 3D 간섭 검증",
                        desc: "KiCad 9 스케메틱 배선과 3D B-Rep 외함을 동시 설계합니다. 부품 간격과 하우징 간섭을 0.00 mm³로 자동 연산 검증합니다.",
                        deliverable: "산출물: 4층 거버 & 3D 조립 어셈블리"
                    }
                    GateCard {
                        stage: "STAGE 02",
                        gate: "GATE 2 DFM",
                        title: "공장 DFM & 원스톱 발주",
                        desc: "JLCPCB PCBA 제조 거버와 CPL, JLC3DP 3D 프린팅 공정 규격을 통과시킨 후 공장 주문을 자동으로 실행합니다.",
                        deliverable: "산출물: 거버·CPL·BOM & 공장 실물 생산"
                    }
                    GateCard {
                        stage: "STAGE 03",
                        gate: "GATE 3 BRING-UP",
                        title: "신호 브링업 & 펌웨어 탑재",
                        desc: "입고된 보드에 테스트 펌웨어를 플래싱하고 오실로스코프 신호와 전압을 검측한 후 의뢰자에게 동작 상태를 1:1 시연합니다.",
                        deliverable: "산출물: 실물 완제품 & 펌웨어 소스코드"
                    }
                }
            }
        }
    }
}

/// Single verification gate card.
#[component]
fn GateCard(
    stage: &'static str,
    gate: &'static str,
    title: &'static str,
    desc: &'static str,
    deliverable: &'static str,
) -> Element {
    rsx! {
        div {
            class: "p-5 rounded-xl border border-zinc-200 bg-white flex flex-col justify-between space-y-4 hover:border-zinc-300 shadow-2xs transition-all",
            div {
                class: "space-y-3",
                div {
                    class: "flex items-center justify-between text-xs pb-2 border-b border-zinc-100",
                    span { class: "text-zinc-900 font-bold", "{stage}" }
                    span { class: "text-[10px] text-zinc-500", "{gate}" }
                }
                h3 { class: "text-base font-bold text-zinc-900 font-sans", "{title}" }
                p {
                    class: "text-xs text-zinc-600 font-sans leading-relaxed",
                    "{desc}"
                }
            }
            div {
                class: "pt-3 border-t border-zinc-100 text-[10px] text-zinc-500",
                "{deliverable}"
            }
        }
    }
}
