use dioxus::prelude::*;

#[component]
pub fn Comparison() -> Element {
    rsx! {
        section {
            id: "why-turnkey",
            class: "py-24 bg-[#090D14] relative overflow-hidden",
            div {
                class: "max-w-7xl mx-auto px-4 sm:px-6 lg:px-8",
                // Section Header
                div {
                    class: "text-center max-w-3xl mx-auto space-y-4 mb-16",
                    div {
                        class: "inline-flex items-center gap-2 px-3 py-1 rounded-full bg-orange-500/10 border border-orange-500/20 text-orange-400 text-xs font-mono font-medium",
                        "THE ZERO-PINGPONG ADVANTAGE"
                    }
                    h2 {
                        class: "text-3xl sm:text-4xl lg:text-5xl font-extrabold text-white tracking-tight",
                        "왜 3사 분리 외주 대신"
                        br {}
                        span {
                            class: "text-transparent bg-clip-text bg-gradient-to-r from-orange-400 to-amber-300",
                            "Holt Lab 턴키인가?"
                        }
                    }
                    p {
                        class: "text-slate-300 text-base sm:text-lg leading-relaxed",
                        "하드웨어 프로젝트가 지연되고 예산이 터지는 90%의 원인은 엔지니어링 실력이 아니라 "
                        strong { class: "text-white font-semibold", "외주자들 간의 '책임 전가(핑퐁)'와 '조립 불일치'" }
                        " 때문입니다."
                    }
                }

                // Comparison Table Grid
                div {
                    class: "grid grid-cols-1 md:grid-cols-2 gap-8 items-stretch",
                    // Left Column: Traditional Split Outsourcing (Red / Gray Danger)
                    div {
                        class: "rounded-2xl border border-red-500/20 bg-gradient-to-b from-red-950/10 via-[#121620] to-[#121620] p-6 sm:p-8 flex flex-col justify-between space-y-6 shadow-xl",
                        div {
                            class: "space-y-4",
                            div {
                                class: "flex items-center justify-between pb-4 border-b border-white/[0.08]",
                                span {
                                    class: "px-3 py-1 rounded-md bg-red-500/20 border border-red-500/30 text-red-300 text-xs font-mono font-semibold",
                                    "전통적 3사 분리 외주"
                                }
                                span { class: "text-xs font-mono text-red-400", "High Risk / Ping-pong" }
                            }
                            h3 {
                                class: "text-xl font-bold text-white",
                                "기구 따로, 회로 따로, 펌웨어 따로"
                            }
                            ul {
                                class: "space-y-4 text-sm text-slate-300",
                                li {
                                    class: "flex items-start gap-3",
                                    span { class: "text-red-400 font-bold text-base mt-0.5", "✕" }
                                    div {
                                        strong { class: "text-white block", "물리적 조립 간섭 참사" }
                                        span { class: "text-slate-400 text-xs", "기구 설계자가 회로 부품의 높이와 커넥터 위치를 놓쳐 뚜껑이 안 닫히고 외함 재가공만 2~3회 반복." }
                                    }
                                }
                                li {
                                    class: "flex items-start gap-3",
                                    span { class: "text-red-400 font-bold text-base mt-0.5", "✕" }
                                    div {
                                        strong { class: "text-white block", "끝없는 책임 핑퐁" }
                                        span { class: "text-slate-400 text-xs", "보드가 안 켜지면 펌웨어는 회로 탓, 회로는 기구 탓, 기구는 외주 탓하며 발주자만 속이 터짐." }
                                    }
                                }
                                li {
                                    class: "flex items-start gap-3",
                                    span { class: "text-red-400 font-bold text-base mt-0.5", "✕" }
                                    div {
                                        strong { class: "text-white block", "중간 PM 및 영업 마진 거품" }
                                        span { class: "text-slate-400 text-xs", "대형 에이전시의 불필요한 관리 인력과 영업 이익이 붙어 단순 PoC 견적이 2,500만~4,000만 원으로 폭증." }
                                    }
                                }
                                li {
                                    class: "flex items-start gap-3",
                                    span { class: "text-red-400 font-bold text-base mt-0.5", "✕" }
                                    div {
                                        strong { class: "text-white block", "납기 4~6개월 지연" }
                                        span { class: "text-slate-400 text-xs", "3사 간 일정 조율과 수정 핑퐁으로 인해 정부지원사업 마감 기한과 데모 일정을 놓침." }
                                    }
                                }
                            }
                        }
                        div {
                            class: "p-4 rounded-xl bg-red-500/10 border border-red-500/20 text-xs text-red-200 text-center font-mono",
                            "결과: 일정 지연, 예산 소진, 동작하지 않는 깡통 시제품"
                        }
                    }

                    // Right Column: Holt Lab Turnkey (Orange / Emerald Success)
                    div {
                        class: "rounded-2xl border border-orange-500/40 bg-gradient-to-b from-orange-950/20 via-[#131926] to-[#111724] p-6 sm:p-8 flex flex-col justify-between space-y-6 shadow-2xl relative",
                        // Highlight Pill Badge
                        div {
                            class: "absolute -top-3.5 right-6 px-3 py-1 rounded-full bg-orange-500 text-white font-mono text-xs font-bold shadow-lg shadow-orange-500/30",
                            "RECOMMENDED // 턴키 직통"
                        }

                        div {
                            class: "space-y-4",
                            div {
                                class: "flex items-center justify-between pb-4 border-b border-white/[0.08]",
                                span {
                                    class: "px-3 py-1 rounded-md bg-orange-500/20 border border-orange-500/40 text-orange-300 text-xs font-mono font-semibold",
                                    "Holt Lab 부티크 턴키"
                                }
                                span { class: "text-xs font-mono text-emerald-400 font-semibold", "✓ Zero-Pingpong Architecture" }
                            }
                            h3 {
                                class: "text-xl font-bold text-white",
                                "리드 엔지니어 1:1 직통 완주"
                            }
                            ul {
                                class: "space-y-4 text-sm text-slate-200",
                                li {
                                    class: "flex items-start gap-3",
                                    span { class: "text-emerald-400 font-bold text-base mt-0.5", "✓" }
                                    div {
                                        strong { class: "text-white block", "0.00 mm³ 간섭 제로 동시 설계" }
                                        span { class: "text-slate-300 text-xs", "3D B-Rep CAD 모델과 4층 PCB 3D STEP을 단일 작업대에서 동기화하여 첫 공장 주문부터 완벽 조립 보증." }
                                    }
                                }
                                li {
                                    class: "flex items-start gap-3",
                                    span { class: "text-emerald-400 font-bold text-base mt-0.5", "✓" }
                                    div {
                                        strong { class: "text-white block", "핑퐁 없는 단일 책임제" }
                                        span { class: "text-slate-300 text-xs", "회로와 펌웨어를 동시에 아는 설계자가 핀맵과 고속 신호 타이밍(SPI 26.67MHz)을 현장에서 즉각 해결." }
                                    }
                                }
                                li {
                                    class: "flex items-start gap-3",
                                    span { class: "text-emerald-400 font-bold text-base mt-0.5", "✓" }
                                    div {
                                        strong { class: "text-white block", "중간 거품 없는 합리적 비용" }
                                        span { class: "text-slate-300 text-xs", "중간 PM, 영업 사원, 화려한 사무실 임대료 거품을 빼고, 실제 엔지니어링 공수에만 집중하여 70% 이상 합리적." }
                                    }
                                }
                                li {
                                    class: "flex items-start gap-3",
                                    span { class: "text-emerald-400 font-bold text-base mt-0.5", "✓" }
                                    div {
                                        strong { class: "text-white block", "3~4주 초단기 실물 납품" }
                                        span { class: "text-slate-300 text-xs", "JLCPCB SMT와 JLC3DP MJF 공장 직발주 자동화로 데모 날짜와 투자 IR 기한에 맞춰 손에 쥐는 실물 납품." }
                                    }
                                }
                            }
                        }
                        div {
                            class: "p-4 rounded-xl bg-emerald-500/10 border border-emerald-500/30 text-xs text-emerald-300 text-center font-mono font-medium",
                            "결과: 오차 0.00mm 실물 동작 검증 완료 + 완벽한 소스코드 및 도면 인수인계"
                        }
                    }
                }
            }
        }
    }
}
