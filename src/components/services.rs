use dioxus::prelude::*;

#[component]
pub fn Services() -> Element {
    rsx! {
        section {
            id: "capabilities",
            class: "py-24 bg-[#090D14] relative border-t border-white/[0.08]",
            div {
                class: "max-w-7xl mx-auto px-4 sm:px-6 lg:px-8",
                // Header
                div {
                    class: "max-w-3xl mx-auto text-center space-y-4 mb-16",
                    div {
                        class: "inline-flex items-center gap-2 px-3 py-1 rounded-full bg-emerald-500/10 border border-emerald-500/20 text-emerald-400 text-xs font-mono font-medium",
                        "UNIFIED HARDWARE CAPABILITIES"
                    }
                    h2 {
                        class: "text-3xl sm:text-4xl lg:text-5xl font-extrabold text-white tracking-tight",
                        "아이디어 스케치부터 실물까지"
                        br {}
                        span {
                            class: "text-transparent bg-clip-text bg-gradient-to-r from-emerald-400 via-teal-300 to-sky-400",
                            "4대 하드웨어 전문 영역"
                        }
                    }
                    p {
                        class: "text-slate-300 text-base sm:text-lg leading-relaxed",
                        "각 분야를 쪼개서 발주할 필요 없이, 단일 리드 엔지니어가 전체 공정을 통일된 정밀도로 완주합니다."
                    }
                }

                // 4 Capability Cards Grid
                div {
                    class: "grid grid-cols-1 md:grid-cols-2 gap-8",
                    // Card 1: ECAD
                    div {
                        class: "p-8 rounded-3xl bg-[#121622] border border-white/[0.08] hover:border-sky-500/40 transition-all duration-300 space-y-4 group",
                        div {
                            class: "w-12 h-12 rounded-2xl bg-sky-500/10 border border-sky-500/20 flex items-center justify-center text-sky-400 font-mono font-bold text-lg group-hover:scale-110 transition-transform",
                            "01"
                        }
                        h3 {
                            class: "text-2xl font-bold text-white group-hover:text-sky-300 transition-colors",
                            "전자 회로 & 4/6층 PCB 설계 (ECAD)"
                        }
                        p {
                            class: "text-slate-300 text-sm sm:text-base leading-relaxed",
                            "KiCad 9 기반으로 4~6층 고밀도 PCB를 제도합니다. ESP32, Nordic nRF, STM32, RP2040 등 주요 SoC와 전원단(Buck/Boost, LDO), 배터리 충전·PCM 보호 회로, 고속 SPI/RF 임피던스 제어 배선을 완벽히 설계합니다."
                        }
                        div {
                            class: "pt-2 flex flex-wrap gap-2 text-xs font-mono text-slate-400",
                            span { class: "px-2.5 py-1 rounded-md bg-white/[0.04] border border-white/10", "KiCad 9" }
                            span { class: "px-2.5 py-1 rounded-md bg-white/[0.04] border border-white/10", "4/6-Layer" }
                            span { class: "px-2.5 py-1 rounded-md bg-white/[0.04] border border-white/10", "RF / High-Speed SPI" }
                            span { class: "px-2.5 py-1 rounded-md bg-white/[0.04] border border-white/10", "Battery PCM" }
                            span { class: "px-2.5 py-1 rounded-md bg-white/[0.04] border border-white/10", "JLCPCB SMT" }
                        }
                    }

                    // Card 2: MCAD
                    div {
                        class: "p-8 rounded-3xl bg-[#121622] border border-white/[0.08] hover:border-orange-500/40 transition-all duration-300 space-y-4 group",
                        div {
                            class: "w-12 h-12 rounded-2xl bg-orange-500/10 border border-orange-500/20 flex items-center justify-center text-orange-400 font-mono font-bold text-lg group-hover:scale-110 transition-transform",
                            "02"
                        }
                        h3 {
                            class: "text-2xl font-bold text-white group-hover:text-orange-300 transition-colors",
                            "3D 기구 & 맞춤 외함 설계 (MCAD)"
                        }
                        p {
                            class: "text-slate-300 text-sm sm:text-base leading-relaxed",
                            "오픈캐스케이드 B-Rep 솔리드 커널 기반으로 회로 부품의 3D STEP 모델과 0.00mm 단위로 결합되는 외함을 설계합니다. HP MJF 나일론 3D 프린팅, SLA 투명 레진, 또는 정밀 CNC 알루미늄 가공을 고려한 DFM 공차를 완벽 준수합니다."
                        }
                        div {
                            class: "pt-2 flex flex-wrap gap-2 text-xs font-mono text-slate-400",
                            span { class: "px-2.5 py-1 rounded-md bg-white/[0.04] border border-white/10", "B-Rep 3D CAD" }
                            span { class: "px-2.5 py-1 rounded-md bg-white/[0.04] border border-white/10", "0.00 mm³ 간섭 검증" }
                            span { class: "px-2.5 py-1 rounded-md bg-white/[0.04] border border-white/10", "HP MJF PA12" }
                            span { class: "px-2.5 py-1 rounded-md bg-white/[0.04] border border-white/10", "CNC 알루미늄" }
                            span { class: "px-2.5 py-1 rounded-md bg-white/[0.04] border border-white/10", "JLC3DP DFM" }
                        }
                    }

                    // Card 3: Firmware
                    div {
                        class: "p-8 rounded-3xl bg-[#121622] border border-white/[0.08] hover:border-emerald-500/40 transition-all duration-300 space-y-4 group",
                        div {
                            class: "w-12 h-12 rounded-2xl bg-emerald-500/10 border border-emerald-500/20 flex items-center justify-center text-emerald-400 font-mono font-bold text-lg group-hover:scale-110 transition-transform",
                            "03"
                        }
                        h3 {
                            class: "text-2xl font-bold text-white group-hover:text-emerald-300 transition-colors",
                            "임베디드 펌웨어 & 초저전력 (Firmware)"
                        }
                        p {
                            class: "text-slate-300 text-sm sm:text-base leading-relaxed",
                            "고객사의 향후 유지보수와 인수인계 편의성에 맞춰 C / C++, Arduino, ESP-IDF부터 고성능 Rust (esp-hal / embassy)까지 최적의 언어로 개발합니다. Zero-Copy DMA 디스플레이 구동과 3.3V 배터리 딥슬립을 구현합니다."
                        }
                        div {
                            class: "pt-2 flex flex-wrap gap-2 text-xs font-mono text-slate-400",
                            span { class: "px-2.5 py-1 rounded-md bg-white/[0.04] border border-white/10", "C / C++ / Arduino" }
                            span { class: "px-2.5 py-1 rounded-md bg-white/[0.04] border border-white/10", "ESP-IDF / Zephyr" }
                            span { class: "px-2.5 py-1 rounded-md bg-white/[0.04] border border-white/10", "Rust (esp-hal)" }
                            span { class: "px-2.5 py-1 rounded-md bg-white/[0.04] border border-white/10", "Zero-Copy GDMA" }
                            span { class: "px-2.5 py-1 rounded-md bg-white/[0.04] border border-white/10", "초저전력 슬립" }
                        }
                    }

                    // Card 4: Turnkey Assembly
                    div {
                        class: "p-8 rounded-3xl bg-[#121622] border border-white/[0.08] hover:border-purple-500/40 transition-all duration-300 space-y-4 group",
                        div {
                            class: "w-12 h-12 rounded-2xl bg-purple-500/10 border border-purple-500/20 flex items-center justify-center text-purple-400 font-mono font-bold text-lg group-hover:scale-110 transition-transform",
                            "04"
                        }
                        h3 {
                            class: "text-2xl font-bold text-white group-hover:text-purple-300 transition-colors",
                            "공장 발주 대행 & 실물 조립 브링업"
                        }
                        p {
                            class: "text-slate-300 text-sm sm:text-base leading-relaxed",
                            "JLCPCB PCBA 발주, JLC3DP 외함 가공, LCSC / 디바이스마트 핵심 부품 소싱을 원스톱 대행합니다. 도착한 실물 기판과 하우징을 직접 조립·납땜하고, 펌웨어를 플래싱하여 손에서 버튼을 누르면 동작하는 완제품으로 납품합니다."
                        }
                        div {
                            class: "pt-2 flex flex-wrap gap-2 text-xs font-mono text-slate-400",
                            span { class: "px-2.5 py-1 rounded-md bg-white/[0.04] border border-white/10", "PCBA 발주 대행" }
                            span { class: "px-2.5 py-1 rounded-md bg-white/[0.04] border border-white/10", "실물 조립 & 브링업" }
                            span { class: "px-2.5 py-1 rounded-md bg-white/[0.04] border border-white/10", "오실로스코프 신호 검증" }
                            span { class: "px-2.5 py-1 rounded-md bg-white/[0.04] border border-white/10", "동작 완제품 택배 발송" }
                        }
                    }
                }
            }
        }
    }
}
