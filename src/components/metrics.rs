use dioxus::prelude::*;

#[component]
pub fn Metrics() -> Element {
    rsx! {
        section {
            class: "py-10 border-y border-white/[0.08] bg-[#07090F] relative font-mono",
            div {
                class: "max-w-7xl mx-auto px-4 sm:px-6 lg:px-8",
                div {
                    class: "grid grid-cols-2 lg:grid-cols-4 gap-4 sm:gap-6",
                    // Stat 1: 0.00 mm³
                    div {
                        class: "p-4 sm:p-5 rounded bg-white/[0.01] border border-white/[0.06] hover:border-white/20 transition-colors space-y-1.5",
                        div {
                            class: "text-2xl sm:text-3xl font-bold tracking-tight text-white",
                            "0.00 mm³"
                        }
                        div {
                            class: "text-[11px] font-semibold text-[#FF5722] uppercase tracking-wider",
                            "CAD COLLISION VOLUME"
                        }
                        p {
                            class: "text-xs text-zinc-400 font-sans leading-relaxed pt-1",
                            "오픈캐스케이드 B-Rep 솔리드와 4층 회로 3D STEP을 0.01mm 오차 없이 정합하여 기구 간섭을 원천 차단합니다."
                        }
                    }

                    // Stat 2: 1:1 Direct Turnkey
                    div {
                        class: "p-4 sm:p-5 rounded bg-white/[0.01] border border-white/[0.06] hover:border-white/20 transition-colors space-y-1.5",
                        div {
                            class: "text-2xl sm:text-3xl font-bold tracking-tight text-white",
                            "1:1 DIRECT"
                        }
                        div {
                            class: "text-[11px] font-semibold text-zinc-300 uppercase tracking-wider",
                            "LEAD ARCHITECT TURNKEY"
                        }
                        p {
                            class: "text-xs text-zinc-400 font-sans leading-relaxed pt-1",
                            "기구·회로·펌웨어 분리 외주의 책임 전가 핑퐁을 없애고, 단일 엔지니어가 전체 품질을 1:1로 책임집니다."
                        }
                    }

                    // Stat 3: 3~4 Weeks Speed
                    div {
                        class: "p-4 sm:p-5 rounded bg-white/[0.01] border border-white/[0.06] hover:border-white/20 transition-colors space-y-1.5",
                        div {
                            class: "text-2xl sm:text-3xl font-bold tracking-tight text-white",
                            "3~4 WEEKS"
                        }
                        div {
                            class: "text-[11px] font-semibold text-zinc-300 uppercase tracking-wider",
                            "RAPID WORKING PROTOTYPE"
                        }
                        p {
                            class: "text-xs text-zinc-400 font-sans leading-relaxed pt-1",
                            "자동화된 공장 DFM 검증과 원스톱 직발주(JLCPCB/JLC3DP)로 투자 유치 및 과제 마감 일정에 맞춰 완제품을 납품합니다."
                        }
                    }

                    // Stat 4: 100% Grant Evidence
                    div {
                        class: "p-4 sm:p-5 rounded bg-white/[0.01] border border-white/[0.06] hover:border-white/20 transition-colors space-y-1.5",
                        div {
                            class: "text-2xl sm:text-3xl font-bold tracking-tight text-white",
                            "100% AUDIT"
                        }
                        div {
                            class: "text-[11px] font-semibold text-emerald-400 uppercase tracking-wider",
                            "GOVERNMENT GRANT COMPLIANT"
                        }
                        p {
                            class: "text-xs text-zinc-400 font-sans leading-relaxed pt-1",
                            "예창패, 초창패, 청창사 등 정부지원사업 맞춤 전자계약, 세금계산서, 회로도/도면/펌웨어 전수 검수보고서를 지원합니다."
                        }
                    }
                }
            }
        }
    }
}
