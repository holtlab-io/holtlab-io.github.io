use dioxus::prelude::*;

#[component]
pub fn Process() -> Element {
    rsx! {
        section {
            id: "process",
            class: "py-20 bg-[#07090E] relative border-b border-white/[0.08] font-mono",
            div {
                class: "max-w-7xl mx-auto px-4 sm:px-6 lg:px-8",
                // Header
                div {
                    class: "max-w-3xl space-y-3 mb-12",
                    div {
                        class: "inline-flex items-center gap-2 px-2.5 py-1 rounded bg-white/[0.03] border border-white/10 text-zinc-400 text-xs tracking-widest uppercase",
                        "DETERMINISTIC VERIFICATION PROTOCOL"
                    }
                    h2 {
                        class: "text-2xl sm:text-4xl font-bold text-white tracking-tight font-sans",
                        "실패 없는 하드웨어 제작 "
                        br {}
                        span { class: "text-zinc-400", "4단계 엔지니어링 검증 게이트" }
                    }
                    p {
                        class: "text-zinc-400 text-sm sm:text-base leading-relaxed font-sans",
                        "주먹구구식 빵판 회로가 아닙니다. 공장 제조 규격과 3D 물리 정합성을 사전에 검증하여 "
                        span { class: "text-white font-medium", "첫 번째 공장 발주부터 바로 동작하는 시제품" }
                        "을 만듭니다."
                    }
                }

                // 4-Step Process Pipeline
                div {
                    class: "grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-4",
                    // Gate 0
                    div {
                        class: "p-5 rounded border border-white/[0.08] bg-[#0A0D13] flex flex-col justify-between space-y-4 hover:border-white/20 transition-all",
                        div {
                            class: "space-y-3",
                            div {
                                class: "flex items-center justify-between text-xs text-zinc-400 pb-2 border-b border-white/[0.06]",
                                span { class: "text-[#FF5722] font-bold", "STAGE 00" },
                                span { class: "text-[10px] text-zinc-500", "GATE 0 LOCK" }
                            }
                            h3 { class: "text-base font-bold text-white font-sans", "COTS 부품 & 스펙 동결" }
                            p {
                                class: "text-xs text-zinc-300 font-sans leading-relaxed",
                                "MCU, 디스플레이, 배터리, 커넥터 등 핵심 부품의 데이터시트와 3D STEP 모델을 먼저 확보하고 수급 안정성을 검증합니다."
                            }
                        }
                        div {
                            class: "pt-3 border-t border-white/[0.06] text-[10px] text-zinc-400",
                            "산출물: 부품 사양서 & 3D STEP 락"
                        }
                    }

                    // Gate 1
                    div {
                        class: "p-5 rounded border border-white/[0.08] bg-[#0A0D13] flex flex-col justify-between space-y-4 hover:border-white/20 transition-all",
                        div {
                            class: "space-y-3",
                            div {
                                class: "flex items-center justify-between text-xs text-zinc-400 pb-2 border-b border-white/[0.06]",
                                span { class: "text-[#FF5722] font-bold", "STAGE 01" },
                                span { class: "text-[10px] text-zinc-500", "GATE 1 AUDIT" }
                            }
                            h3 { class: "text-base font-bold text-white font-sans", "회로 제도 & 3D 간섭 검증" }
                            p {
                                class: "text-xs text-zinc-300 font-sans leading-relaxed",
                                "KiCad 9 스케메틱 배선과 3D B-Rep 외함을 동시 설계합니다. 부품 간격과 하우징 간섭을 0.00 mm³로 자동 연산 검증합니다."
                            }
                        }
                        div {
                            class: "pt-3 border-t border-white/[0.06] text-[10px] text-zinc-400",
                            "산출물: 4층 거버 & 3D 조립 어셈블리"
                        }
                    }

                    // Gate 2
                    div {
                        class: "p-5 rounded border border-white/[0.08] bg-[#0A0D13] flex flex-col justify-between space-y-4 hover:border-white/20 transition-all",
                        div {
                            class: "space-y-3",
                            div {
                                class: "flex items-center justify-between text-xs text-zinc-400 pb-2 border-b border-white/[0.06]",
                                span { class: "text-[#FF5722] font-bold", "STAGE 02" },
                                span { class: "text-[10px] text-zinc-500", "GATE 2 DFM" }
                            }
                            h3 { class: "text-base font-bold text-white font-sans", "공장 DFM & 원스톱 발주" }
                            p {
                                class: "text-xs text-zinc-300 font-sans leading-relaxed",
                                "JLCPCB PCBA 제조 거버와 CPL, JLC3DP 3D 프린팅 공정 규격을 통과시킨 후 공장 주문을 자동으로 실행합니다."
                            }
                        }
                        div {
                            class: "pt-3 border-t border-white/[0.06] text-[10px] text-zinc-400",
                            "산출물: 거버·CPL·BOM & 공장 실물 생산"
                        }
                    }

                    // Gate 3
                    div {
                        class: "p-5 rounded border border-white/[0.08] bg-[#0A0D13] flex flex-col justify-between space-y-4 hover:border-white/20 transition-all",
                        div {
                            class: "space-y-3",
                            div {
                                class: "flex items-center justify-between text-xs text-zinc-400 pb-2 border-b border-white/[0.06]",
                                span { class: "text-[#FF5722] font-bold", "STAGE 03" },
                                span { class: "text-[10px] text-zinc-500", "BRING-UP" }
                            }
                            h3 { class: "text-base font-bold text-white font-sans", "실물 조립 & 펌웨어 브링업" }
                            p {
                                class: "text-xs text-zinc-300 font-sans leading-relaxed",
                                "도착한 기판과 외함을 직접 조립하고, 전원단 계측 및 펌웨어 플래싱을 마친 후 손에서 완벽히 동작하는 상태로 납품합니다."
                            }
                        }
                        div {
                            class: "pt-3 border-t border-white/[0.06] text-[10px] text-emerald-400 font-bold",
                            "산출물: 손에 쥐는 동작 실물 완제품"
                        }
                    }
                }
            }
        }
    }
}
