use dioxus::prelude::*;

#[component]
pub fn Hero() -> Element {
    rsx! {
        section {
            class: "relative pt-32 pb-20 md:pt-40 md:pb-28 overflow-hidden bg-blueprint-grid",
            // Ambient Glow in Background
            div {
                class: "absolute top-1/4 left-1/2 -translate-x-1/2 -translate-y-1/2 w-[600px] h-[350px] bg-orange-500/10 blur-[130px] rounded-full pointer-events-none"
            }
            div {
                class: "absolute top-1/3 right-10 w-[400px] h-[300px] bg-sky-500/10 blur-[120px] rounded-full pointer-events-none"
            }

            div {
                class: "relative max-w-7xl mx-auto px-4 sm:px-6 lg:px-8",
                div {
                    class: "text-center max-w-4xl mx-auto space-y-6",
                    // Eyebrow Status Pill
                    div {
                        class: "inline-flex items-center gap-2.5 px-3.5 py-1.5 rounded-full bg-white/[0.04] border border-white/[0.12] backdrop-blur-sm shadow-inner",
                        span {
                            class: "w-2 h-2 rounded-full bg-emerald-400 animate-pulse"
                        }
                        span {
                            class: "text-xs font-mono font-medium text-slate-300 tracking-wide",
                            "BOUTIQUE HARDWARE & EMBEDDED STUDIO"
                        }
                        span {
                            class: "text-slate-500 text-xs",
                            "|"
                        }
                        span {
                            class: "text-xs font-mono text-orange-400 font-medium",
                            "PoC / MVP Turnkey"
                        }
                    }

                    // Main Headline
                    h1 {
                        class: "text-4xl sm:text-6xl lg:text-7xl font-extrabold tracking-tight text-white leading-[1.12]",
                        "From Silicon to Screen."
                        br {}
                        span {
                            class: "text-transparent bg-clip-text bg-gradient-to-r from-orange-400 via-amber-300 to-orange-500",
                            "외주 핑퐁 없는 1인 턴키 하드웨어 랩"
                        }
                    }

                    // Subheading
                    p {
                        class: "text-base sm:text-xl text-slate-300 font-normal leading-relaxed max-w-3xl mx-auto",
                        "기구 따로, 회로 따로, 펌웨어 따로 맡기느라 시간과 예산을 날리셨습니까?"
                        br {}
                        "Holt Lab은 "
                        strong { class: "text-white font-semibold", "3D 기구(MCAD) · 4층 회로(ECAD) · 초저전력 펌웨어" }
                        "를 단일 리드 아키텍트가 물리적 오차 0.00mm로 직통 설계하여, 실패 없이 손에 쥐는 동작 실물을 완성합니다."
                    }

                    // CTA Buttons & Trust Indicators
                    div {
                        class: "pt-4 flex flex-col sm:flex-row items-center justify-center gap-4",
                        a {
                            href: "#contact",
                            class: "w-full sm:w-auto inline-flex items-center justify-center gap-2.5 px-8 py-4 rounded-xl bg-orange-500 hover:bg-orange-600 text-white font-semibold text-base shadow-xl shadow-orange-500/25 hover:shadow-orange-500/40 transition-all active:scale-[0.98]",
                            "30분 무료 타당성 진단 신청"
                            span { class: "font-mono text-lg", "→" }
                        }
                        a {
                            href: "#showcase",
                            class: "w-full sm:w-auto inline-flex items-center justify-center gap-2 px-8 py-4 rounded-xl bg-white/[0.04] hover:bg-white/[0.08] text-slate-200 border border-white/10 hover:border-white/20 font-medium text-base backdrop-blur-sm transition-all",
                            "1호 실물 쇼케이스 보기"
                            span { class: "font-mono text-slate-400", "↓" }
                        }
                    }

                    // Trust Taglines
                    div {
                        class: "pt-2 flex flex-wrap items-center justify-center gap-x-6 gap-y-2 text-xs font-mono text-slate-400",
                        span { class: "flex items-center gap-1.5", "✓ 정부지원사업(예창패·청창사) 정식 지출증빙 100%" }
                        span { class: "flex items-center gap-1.5", "✓ JLCPCB / JLC3DP 공장 발주 완주" }
                        span { class: "flex items-center gap-1.5", "✓ C / Arduino / Rust 맞춤 인수인계" }
                    }
                }

                // Interactive Visual Hero Deck
                div {
                    class: "mt-14 max-w-5xl mx-auto relative",
                    div {
                        class: "relative rounded-2xl overflow-hidden border border-white/[0.14] bg-[#121722]/90 shadow-2xl shadow-black/80 backdrop-blur-xl p-2 sm:p-4",
                        // Top Telemetry Header
                        div {
                            class: "flex items-center justify-between pb-3 px-2 border-b border-white/[0.06] text-xs font-mono text-slate-400",
                            div {
                                class: "flex items-center gap-2",
                                span { class: "w-3 h-3 rounded-full bg-red-500/80 inline-block" }
                                span { class: "w-3 h-3 rounded-full bg-yellow-500/80 inline-block" }
                                span { class: "w-3 h-3 rounded-full bg-green-500/80 inline-block" }
                                span { class: "ml-2 text-slate-400 hidden sm:inline", "HOLT_LAB_CORE_PIPELINE // V2_CONVERGED_TRUE_PASS" }
                            }
                            div {
                                class: "flex items-center gap-3",
                                span { class: "px-2 py-0.5 rounded bg-emerald-500/10 text-emerald-400 border border-emerald-500/30 text-[11px]", "GATE 1 & 2 VERIFIED" }
                                span { class: "text-slate-400", "COLLISION: 0.00 mm³" }
                            }
                        }

                        // Hero Visual Content: Banner Showcase
                        div {
                            class: "mt-3 relative rounded-xl overflow-hidden aspect-[16/9] sm:aspect-[21/9] bg-[#0E121A] flex items-center justify-center",
                            img {
                                src: "assets/branding/banner.png",
                                alt: "Holt Lab Turnkey Engineering Showcase",
                                class: "w-full h-full object-cover object-center transform hover:scale-[1.02] transition-transform duration-700"
                            }
                            // Bottom Floating Hardware Specs Overlay
                            div {
                                class: "absolute bottom-3 left-3 right-3 sm:bottom-4 sm:left-4 sm:right-4 p-3 sm:p-4 rounded-xl bg-[#090D14]/85 backdrop-blur-md border border-white/10 flex flex-wrap items-center justify-between gap-3 text-xs font-mono",
                                div {
                                    class: "flex flex-wrap items-center gap-2",
                                    span { class: "px-2 py-1 rounded bg-orange-500/20 text-orange-300 border border-orange-500/40 font-semibold", "FLAGSHIP CASE" }
                                    span { class: "text-white font-medium", "Win95 Minesweeper Touch Keyring" }
                                }
                                div {
                                    class: "flex flex-wrap items-center gap-3 text-slate-300",
                                    span { class: "text-slate-400", "• 4-Layer PCB (40.0×64.6mm)" }
                                    span { class: "text-slate-400", "• MJF Enclosure (0.00mm clearance)" }
                                    span { class: "text-slate-400", "• 26.67MHz SPI DMA" }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
