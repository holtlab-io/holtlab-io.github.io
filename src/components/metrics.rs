use dioxus::prelude::*;

#[component]
pub fn Metrics() -> Element {
    rsx! {
        section {
            class: "py-12 border-y border-white/[0.08] bg-[#0E131C]/60 backdrop-blur-sm relative",
            div {
                class: "max-w-7xl mx-auto px-4 sm:px-6 lg:px-8",
                div {
                    class: "grid grid-cols-2 lg:grid-cols-4 gap-6 sm:gap-8",
                    // Stat 1: 0.00 mm³
                    div {
                        class: "p-6 rounded-2xl bg-white/[0.02] border border-white/[0.06] hover:border-orange-500/30 transition-colors space-y-2",
                        div {
                            class: "text-3xl sm:text-4xl font-extrabold font-mono tracking-tight text-transparent bg-clip-text bg-gradient-to-r from-orange-400 to-amber-300",
                            "0.00 mm³"
                        }
                        div {
                            class: "text-sm font-semibold text-white",
                            "기구·회로 간섭 제로"
                        }
                        p {
                            class: "text-xs text-slate-400 leading-relaxed",
                            "3D B-Rep 커널과 4층 PCB 3D 부품을 0.01mm 단위로 동시 정합하여 뚜껑이 안 닫히는 참사를 원천 차단합니다."
                        }
                    }

                    // Stat 2: 1-Stop Lead Architect
                    div {
                        class: "p-6 rounded-2xl bg-white/[0.02] border border-white/[0.06] hover:border-orange-500/30 transition-colors space-y-2",
                        div {
                            class: "text-3xl sm:text-4xl font-extrabold font-mono tracking-tight text-transparent bg-clip-text bg-gradient-to-r from-sky-400 to-indigo-300",
                            "1-Stop"
                        }
                        div {
                            class: "text-sm font-semibold text-white",
                            "단일 책임 턴키 직통"
                        }
                        p {
                            class: "text-xs text-slate-400 leading-relaxed",
                            "기구, 회로, 펌웨어 담당자가 서로 남 탓하는 핑퐁을 없애고, 리드 엔지니어가 전체 품질을 1:1로 직접 책임집니다."
                        }
                    }

                    // Stat 3: 3~4 Weeks Speed
                    div {
                        class: "p-6 rounded-2xl bg-white/[0.02] border border-white/[0.06] hover:border-orange-500/30 transition-colors space-y-2",
                        div {
                            class: "text-3xl sm:text-4xl font-extrabold font-mono tracking-tight text-transparent bg-clip-text bg-gradient-to-r from-emerald-400 to-teal-300",
                            "3~4 Weeks"
                        }
                        div {
                            class: "text-sm font-semibold text-white",
                            "신속 동작 PoC 완주"
                        }
                        p {
                            class: "text-xs text-slate-400 leading-relaxed",
                            "자동화된 DFM 검증과 원스톱 공장 발주(JLCPCB/JLC3DP)로 데모 기한과 투자 유치 일정에 맞춥니다."
                        }
                    }

                    // Stat 4: 100% Grant Evidence
                    div {
                        class: "p-6 rounded-2xl bg-white/[0.02] border border-white/[0.06] hover:border-orange-500/30 transition-colors space-y-2",
                        div {
                            class: "text-3xl sm:text-4xl font-extrabold font-mono tracking-tight text-transparent bg-clip-text bg-gradient-to-r from-purple-400 to-pink-300",
                            "100%"
                        }
                        div {
                            class: "text-sm font-semibold text-white",
                            "정부지원사업 지출증빙"
                        }
                        p {
                            class: "text-xs text-slate-400 leading-relaxed",
                            "예창패, 초창패, 청창사 등 정부과제 전용 전자계약, 세금계산서, 중간·최종 검수보고서를 철저히 지원합니다."
                        }
                    }
                }
            }
        }
    }
}
