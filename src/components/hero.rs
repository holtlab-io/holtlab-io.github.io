use dioxus::prelude::*;

#[component]
pub fn Hero() -> Element {
    rsx! {
        section {
            class: "relative pt-28 pb-16 md:pt-36 md:pb-24 overflow-hidden bg-cad-grid",
            div {
                class: "relative max-w-7xl mx-auto px-4 sm:px-6 lg:px-8",
                div {
                    class: "max-w-4xl mx-auto text-center space-y-6",
                    // Eyebrow Telemetry Tag
                    div {
                        class: "inline-flex items-center gap-2.5 px-3 py-1 rounded bg-white/[0.03] border border-white/[0.1] text-xs font-mono tracking-widest uppercase text-zinc-400",
                        span { class: "w-2 h-2 rounded-full bg-emerald-400 animate-pulse" }
                        span { "BOUTIQUE HARDWARE & EMBEDDED ENGINEERING" }
                        span { class: "text-zinc-600", "//" }
                        span { class: "text-[#FF5722]", "0.00 mm³ TOLERANCE" }
                    }

                    // Main Headline (Stark, Imposing, High-Contrast Typography)
                    h1 {
                        class: "text-4xl sm:text-6xl lg:text-7xl font-bold tracking-tight text-white leading-[1.08]",
                        "From Silicon to Screen."
                        br {}
                        span {
                            class: "text-zinc-400 font-medium text-3xl sm:text-5xl lg:text-6xl block mt-2",
                            "외주 핑퐁 없는 1인 턴키 하드웨어 랩"
                        }
                    }

                    // Subheading: Technical & Direct
                    p {
                        class: "text-base sm:text-lg text-zinc-400 font-normal leading-relaxed max-w-2xl mx-auto pt-2",
                        "기구 따로, 회로 따로, 펌웨어 따로 맡기느라 일정과 예산이 터지셨습니까? "
                        "Holt Lab은 "
                        span { class: "text-white font-medium", "3D 기구(MCAD) · 4층 회로(ECAD) · 베어메탈 펌웨어" }
                        "를 단일 리드 아키텍트가 물리 오차 0.00mm로 직통 설계하여, 실제 동작하는 완성품을 손에 쥐어드립니다."
                    }

                    // Tactile Industrial CTAs
                    div {
                        class: "pt-4 flex flex-col sm:flex-row items-center justify-center gap-3 font-mono text-xs",
                        a {
                            href: "#contact",
                            class: "w-full sm:w-auto inline-flex items-center justify-center gap-2 px-6 py-3.5 rounded-md bg-[#FF5722] hover:bg-[#F4511E] text-white font-bold tracking-wider uppercase transition-all shadow-lg active:scale-[0.98]",
                            "30분 기술 타당성 진단 신청"
                            span { "→" }
                        }
                        a {
                            href: "#showcase",
                            class: "w-full sm:w-auto inline-flex items-center justify-center gap-2 px-6 py-3.5 rounded-md bg-white/[0.04] hover:bg-white/[0.08] text-zinc-300 border border-white/10 hover:border-white/20 font-medium tracking-wider uppercase transition-all",
                            "1호 실물 쇼케이스 분석"
                            span { "↓" }
                        }
                    }

                    // Engineering Verification Standards (No emojis, crisp technical tags)
                    div {
                        class: "pt-3 flex flex-wrap items-center justify-center gap-x-6 gap-y-2 text-[11px] font-mono text-zinc-500",
                        span { class: "flex items-center gap-1.5", "[+] 정부지원사업(예창패·청창사) 정식 지출증빙 100%" }
                        span { class: "flex items-center gap-1.5", "[+] JLCPCB PCBA & JLC3DP MJF 공장 직발주" }
                        span { class: "flex items-center gap-1.5", "[+] C / Arduino / Rust(esp-hal) 풀스택 인수인계" }
                    }
                }

                // Precision Hardware Blueprint Inspection Bench
                div {
                    class: "mt-12 max-w-5xl mx-auto hardware-panel rounded-lg overflow-hidden p-3 sm:p-5 shadow-2xl bg-[#090C12]",
                    // Top Dossier Header
                    div {
                        class: "flex flex-wrap items-center justify-between pb-3 px-1 border-b border-white/[0.06] text-xs font-mono text-zinc-400 gap-2",
                        div {
                            class: "flex items-center gap-3",
                            span { class: "px-2 py-0.5 rounded bg-white/[0.05] border border-white/10 text-white font-bold text-[10px]", "REF // REV_A" }
                            span { class: "text-zinc-300 font-semibold", "WIN95 MINESWEEPER TOUCH KEYRING" }
                        }
                        div {
                            class: "flex items-center gap-4 text-[11px]",
                            span { class: "text-emerald-400 font-mono", "● COLLISION: 0.00 mm³" }
                            span { class: "text-zinc-500 hidden sm:inline", "CAD/ECAD INTEGRATED 1:1" }
                        }
                    }

                    // Product Imagery Bench with Technical Caliper Readouts
                    div {
                        class: "mt-3 relative rounded overflow-hidden aspect-[16/9] sm:aspect-[21/9] bg-[#07090E] border border-white/[0.04] flex items-center justify-center",
                        img {
                            src: "assets/branding/banner.png",
                            alt: "Holt Lab Turnkey Engineering Showcase",
                            class: "w-full h-full object-cover object-center"
                        }
                        // Bottom Engineering Telemetry Callout Strip
                        div {
                            class: "absolute bottom-2 left-2 right-2 sm:bottom-3 sm:left-3 sm:right-3 p-2.5 sm:p-3 rounded bg-[#06080C]/90 backdrop-blur-md border border-white/10 flex flex-wrap items-center justify-between gap-2 text-[11px] font-mono",
                            div {
                                class: "flex flex-wrap items-center gap-2",
                                span { class: "px-1.5 py-0.5 rounded bg-[#FF5722]/20 text-[#FF5722] border border-[#FF5722]/40 font-bold text-[10px]", "ACTUAL SILICON" }
                                span { class: "text-white font-medium", "ESP32-C3 RISC-V + 2.0\" ST7789T3 IPS Touch" }
                            }
                            div {
                                class: "flex flex-wrap items-center gap-3 text-zinc-400",
                                span { "• 4-Layer 7628 Prepreg" }
                                span { "• HP MJF Nylon PA12" }
                                span { "• 26.67MHz SPI Zero-Copy GDMA" }
                            }
                        }
                    }
                }
            }
        }
    }
}
