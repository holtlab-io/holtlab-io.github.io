use dioxus::prelude::*;

#[component]
pub fn Process() -> Element {
    rsx! {
        section {
            id: "process",
            class: "py-24 bg-[#0B0F17] relative border-t border-white/[0.08]",
            div {
                class: "max-w-7xl mx-auto px-4 sm:px-6 lg:px-8",
                // Header
                div {
                    class: "max-w-3xl mx-auto text-center space-y-4 mb-16",
                    div {
                        class: "inline-flex items-center gap-2 px-3 py-1 rounded-full bg-purple-500/10 border border-purple-500/20 text-purple-400 text-xs font-mono font-medium",
                        "DETERMINISTIC VERIFICATION GATES"
                    }
                    h2 {
                        class: "text-3xl sm:text-4xl lg:text-5xl font-extrabold text-white tracking-tight",
                        "실패 없는 하드웨어 제작"
                        br {}
                        span {
                            class: "text-transparent bg-clip-text bg-gradient-to-r from-purple-400 via-pink-300 to-orange-400",
                            "4단계 엔지니어링 검증 게이트"
                        }
                    }
                    p {
                        class: "text-slate-300 text-base sm:text-lg leading-relaxed",
                        "주먹구구식 빵판 회로가 아닙니다. 공장 제조 규격과 3D 물리 정합성을 사전에 검증하여 "
                        strong { class: "text-white font-semibold", "첫 번째 발주부터 바로 동작하는 시제품" }
                        "을 만듭니다."
                    }
                }

                // 4-Step Process Timeline
                div {
                    class: "grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-6",
                    // Step 1: Gate 0
                    div {
                        class: "p-6 rounded-2xl bg-[#121622] border border-white/[0.08] hover:border-purple-500/30 transition-colors space-y-4 relative flex flex-col justify-between",
                        div {
                            class: "space-y-3",
                            div {
                                class: "flex items-center justify-between text-xs font-mono text-slate-400 pb-2 border-b border-white/[0.06]",
                                span { class: "text-purple-400 font-bold", "STAGE 00" },
                                span { "GATE 0 LOCK" }
                            }
                            h3 { class: "text-lg font-bold text-white", "COTS 부품 & 스펙 동결" }
                            p {
                                class: "text-xs text-slate-300 leading-relaxed",
                                "MCU, 디스플레이, 배터리, 커넥터 등 핵심 부품의 데이터시트와 3D STEP 모델을 먼저 확보하고 수급 안정성을 검증합니다."
                            }
                        }
                        div {
                            class: "pt-4 border-t border-white/[0.06] text-[11px] font-mono text-purple-300",
                            "산출물: 부품 규격서 & 3D COTS 락"
                        }
                    }

                    // Step 2: Gate 1
                    div {
                        class: "p-6 rounded-2xl bg-[#121622] border border-white/[0.08] hover:border-sky-500/30 transition-colors space-y-4 relative flex flex-col justify-between",
                        div {
                            class: "space-y-3",
                            div {
                                class: "flex items-center justify-between text-xs font-mono text-slate-400 pb-2 border-b border-white/[0.06]",
                                span { class: "text-sky-400 font-bold", "STAGE 01" },
                                span { "GATE 1 AUDIT" }
                            }
                            h3 { class: "text-lg font-bold text-white", "회로 제도 & 3D 간섭 검증" }
                            p {
                                class: "text-xs text-slate-300 leading-relaxed",
                                "KiCad 9 스케메틱 배선과 3D B-Rep 외함을 동시 설계합니다. 부품 간격과 하우징 간섭을 0.00 mm³로 자동 연산 검증합니다."
                            }
                        }
                        div {
                            class: "pt-4 border-t border-white/[0.06] text-[11px] font-mono text-sky-300",
                            "산출물: 회로도 & 기구 조립 모델"
                        }
                    }

                    // Step 3: Gate 2
                    div {
                        class: "p-6 rounded-2xl bg-[#121622] border border-white/[0.08] hover:border-emerald-500/30 transition-colors space-y-4 relative flex flex-col justify-between",
                        div {
                            class: "space-y-3",
                            div {
                                class: "flex items-center justify-between text-xs font-mono text-slate-400 pb-2 border-b border-white/[0.06]",
                                span { class: "text-emerald-400 font-bold", "STAGE 02" },
                                span { "GATE 2 DFM" }
                            }
                            h3 { class: "text-lg font-bold text-white", "공장 DFM & 원스톱 발주" }
                            p {
                                class: "text-xs text-slate-300 leading-relaxed",
                                "JLCPCB PCBA 제조 거버와 CPL, JLC3DP 3D 프린팅 공정 규격을 통과시킨 후 공장 주문을 자동으로 실행합니다."
                            }
                        }
                        div {
                            class: "pt-4 border-t border-white/[0.06] text-[11px] font-mono text-emerald-300",
                            "산출물: 거버·BOM 및 공장 실물 생산"
                        }
                    }

                    // Step 4: Gate 3
                    div {
                        class: "p-6 rounded-2xl bg-[#121622] border border-white/[0.08] hover:border-orange-500/30 transition-colors space-y-4 relative flex flex-col justify-between",
                        div {
                            class: "space-y-3",
                            div {
                                class: "flex items-center justify-between text-xs font-mono text-slate-400 pb-2 border-b border-white/[0.06]",
                                span { class: "text-orange-400 font-bold", "STAGE 03" },
                                span { "BRING-UP" }
                            }
                            h3 { class: "text-lg font-bold text-white", "실물 조립 & 펌웨어 브링업" }
                            p {
                                class: "text-xs text-slate-300 leading-relaxed",
                                "도착한 기판과 외함을 직접 조립하고, 전원단 계측 및 펌웨어 플래싱을 마친 후 손에서 완벽히 동작하는 상태로 납품합니다."
                            }
                        }
                        div {
                            class: "pt-4 border-t border-white/[0.06] text-[11px] font-mono text-orange-300",
                            "산출물: 손에 쥐는 동작 실물 완제품"
                        }
                    }
                }
            }
        }
    }
}
