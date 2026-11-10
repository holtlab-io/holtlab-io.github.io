use dioxus::prelude::*;

#[component]
pub fn Comparison() -> Element {
    rsx! {
        section {
            id: "why-turnkey",
            class: "py-20 bg-[#06080C] relative border-b border-white/[0.08]",
            div {
                class: "max-w-7xl mx-auto px-4 sm:px-6 lg:px-8",
                // Section Header
                div {
                    class: "max-w-3xl space-y-3 mb-12",
                    div {
                        class: "inline-flex items-center gap-2 px-2.5 py-1 rounded bg-white/[0.03] border border-white/10 text-zinc-400 text-xs font-mono tracking-widest uppercase",
                        "SYSTEM ARCHITECTURE AUDIT"
                    }
                    h2 {
                        class: "text-2xl sm:text-4xl font-bold text-white tracking-tight leading-tight",
                        "왜 3사 분리 외주 대신 "
                        br {}
                        span { class: "text-zinc-400", "단일 리드 아키텍트 턴키인가?" }
                    }
                    p {
                        class: "text-zinc-400 text-sm sm:text-base leading-relaxed",
                        "하드웨어 프로젝트가 지연되고 예산이 초과하는 대부분의 원인은 엔지니어링 실력이 아니라, "
                        span { class: "text-white font-medium", "외주자들 간의 책임 전가 핑퐁과 물리적 결합 오차" }
                        " 때문입니다."
                    }
                }

                // Precision Comparison Grid
                div {
                    class: "grid grid-cols-1 md:grid-cols-2 gap-6 items-stretch font-mono",
                    // Left Column: Traditional Split Outsourcing (Subtle dark graphite)
                    div {
                        class: "rounded border border-white/[0.08] bg-[#0A0D13] p-5 sm:p-7 flex flex-col justify-between space-y-6",
                        div {
                            class: "space-y-4",
                            div {
                                class: "flex items-center justify-between pb-3 border-b border-white/[0.06] text-xs text-zinc-400",
                                span { class: "text-zinc-400 font-semibold", "01 // LEGACY FRAGMENTED OUTSOURCING" }
                                span { class: "text-red-400 text-[11px]", "HIGH RISK / PING-PONG" }
                            }
                            h3 {
                                class: "text-lg font-bold text-white font-sans",
                                "기구 따로, 회로 따로, 펌웨어 따로 외주"
                            }
                            ul {
                                class: "space-y-4 text-xs font-sans text-zinc-300",
                                li {
                                    class: "flex items-start gap-3",
                                    span { class: "text-red-400 font-mono font-bold text-sm mt-0.5", "[-]" }
                                    div {
                                        strong { class: "text-white block font-mono text-xs", "물리적 조립 간섭 참사" }
                                        span { class: "text-zinc-400 text-xs", "기구 설계자가 회로 부품의 실측 높이와 커넥터 유격을 놓쳐 뚜껑이 안 닫히고 외함 재가공만 수회 반복." }
                                    }
                                }
                                li {
                                    class: "flex items-start gap-3",
                                    span { class: "text-red-400 font-mono font-bold text-sm mt-0.5", "[-]" }
                                    div {
                                        strong { class: "text-white block font-mono text-xs", "끝없는 책임 핑퐁" }
                                        span { class: "text-zinc-400 text-xs", "보드가 안 켜지면 펌웨어는 회로 탓, 회로는 기구 탓, 기구는 가공집 탓하며 의뢰자만 속이 타들어 감." }
                                    }
                                }
                                li {
                                    class: "flex items-start gap-3",
                                    span { class: "text-red-400 font-mono font-bold text-sm mt-0.5", "[-]" }
                                    div {
                                        strong { class: "text-white block font-mono text-xs", "중간 PM 및 영업 마진 거품" }
                                        span { class: "text-zinc-400 text-xs", "대형 에이전시의 불필요한 중간 관리 인력과 영업 이익이 얹혀 단순 PoC 견적이 수천만 원으로 폭증." }
                                    }
                                }
                                li {
                                    class: "flex items-start gap-3",
                                    span { class: "text-red-400 font-mono font-bold text-sm mt-0.5", "[-]" }
                                    div {
                                        strong { class: "text-white block font-mono text-xs", "납기 4~6개월 지연" }
                                        span { class: "text-zinc-400 text-xs", "3개 업체 간 도면 수정 핑퐁으로 인해 정부지원사업 마감 기한과 투자 IR 일정을 놓침." }
                                    }
                                }
                            }
                        }
                        div {
                            class: "p-3 rounded bg-white/[0.02] border border-white/[0.04] text-[11px] text-zinc-400 text-center font-mono",
                            "결과: 일정 지연, 예산 소진, 조립 안 되는 불량 시제품"
                        }
                    }

                    // Right Column: Holt Lab Turnkey (Precision Titanium & Safety Orange)
                    div {
                        class: "rounded border border-[#FF5722]/50 bg-[#0B0E17] p-5 sm:p-7 flex flex-col justify-between space-y-6 relative shadow-xl",
                        div {
                            class: "space-y-4",
                            div {
                                class: "flex items-center justify-between pb-3 border-b border-white/[0.08] text-xs font-mono",
                                span { class: "text-[#FF5722] font-semibold", "02 // HOLT LAB BOUTIQUE TURNKEY" }
                                span { class: "text-emerald-400 text-[11px]", "ZERO-PINGPONG INTEGRATED" }
                            }
                            h3 {
                                class: "text-lg font-bold text-white font-sans",
                                "리드 엔지니어 단일 책임 1:1 직통 완주"
                            }
                            ul {
                                class: "space-y-4 text-xs font-sans text-zinc-200",
                                li {
                                    class: "flex items-start gap-3",
                                    span { class: "text-emerald-400 font-mono font-bold text-sm mt-0.5", "[+]" }
                                    div {
                                        strong { class: "text-white block font-mono text-xs", "0.00 mm³ 간섭 제로 동시 설계" }
                                        span { class: "text-zinc-400 text-xs", "3D B-Rep CAD 모델과 4층 PCB 3D STEP을 단일 워크벤치에서 동기화하여 첫 공장 주문부터 완벽 조립 보증." }
                                    }
                                }
                                li {
                                    class: "flex items-start gap-3",
                                    span { class: "text-emerald-400 font-mono font-bold text-sm mt-0.5", "[+]" }
                                    div {
                                        strong { class: "text-white block font-mono text-xs", "핑퐁 없는 단일 책임제" }
                                        span { class: "text-zinc-400 text-xs", "회로와 펌웨어를 동시에 설계하는 엔지니어가 핀맵과 고속 신호 타이밍(SPI 26.67MHz)을 현장에서 즉시 해결." }
                                    }
                                }
                                li {
                                    class: "flex items-start gap-3",
                                    span { class: "text-emerald-400 font-mono font-bold text-sm mt-0.5", "[+]" }
                                    div {
                                        strong { class: "text-white block font-mono text-xs", "실 엔지니어링 공수 기반 합리적 견적" }
                                        span { class: "text-zinc-400 text-xs", "중간 PM, 영업 수수료, 불필요한 고정비 거품을 제거하고 순수 설계 및 공장 실비에만 집중." }
                                    }
                                }
                                li {
                                    class: "flex items-start gap-3",
                                    span { class: "text-emerald-400 font-mono font-bold text-sm mt-0.5", "[+]" }
                                    div {
                                        strong { class: "text-white block font-mono text-xs", "3~4주 초단기 동작 완제품 납품" }
                                        span { class: "text-zinc-400 text-xs", "JLCPCB SMT와 JLC3DP MJF 공장 직발주 자동화로 데모 날짜와 투자 일정에 맞춰 손에 쥐는 실물 완성." }
                                    }
                                }
                            }
                        }
                        div {
                            class: "p-3 rounded bg-[#FF5722]/10 border border-[#FF5722]/20 text-[11px] text-[#FF5722] text-center font-mono font-semibold",
                            "결과: 물리 오차 0.00mm 실물 동작 보증 + 회로도·CAD·펌웨어 전수 인수인계"
                        }
                    }
                }
            }
        }
    }
}
