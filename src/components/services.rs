use dioxus::prelude::*;

#[component]
pub fn Services() -> Element {
    rsx! {
        section {
            id: "capabilities",
            class: "py-20 bg-[#06080C] relative border-b border-white/[0.08] font-mono",
            div {
                class: "max-w-7xl mx-auto px-4 sm:px-6 lg:px-8",
                // Header
                div {
                    class: "max-w-3xl space-y-3 mb-12",
                    div {
                        class: "inline-flex items-center gap-2 px-2.5 py-1 rounded bg-white/[0.03] border border-white/10 text-zinc-400 text-xs tracking-widest uppercase",
                        "UNIFIED HARDWARE CAPABILITIES"
                    }
                    h2 {
                        class: "text-2xl sm:text-4xl font-bold text-white tracking-tight font-sans",
                        "아이디어 스케치부터 실물까지 "
                        br {}
                        span { class: "text-zinc-400", "4대 하드웨어 전문 영역" }
                    }
                    p {
                        class: "text-zinc-400 text-sm sm:text-base leading-relaxed font-sans",
                        "공정마다 업체를 쪼개서 발주할 필요 없이, 단일 리드 엔지니어가 전체 공정을 통일된 공차 규격으로 완주합니다."
                    }
                }

                // 4 Capability Technical Panels
                div {
                    class: "grid grid-cols-1 md:grid-cols-2 gap-5",
                    // 01: ECAD
                    div {
                        class: "p-6 rounded border border-white/[0.08] bg-[#0A0D13] hover:border-white/20 transition-all space-y-4",
                        div {
                            class: "flex items-center justify-between text-xs pb-3 border-b border-white/[0.06]",
                            span { class: "text-[#FF5722] font-bold", "01 // ECAD" },
                            span { class: "text-zinc-500", "KICAD 9 / HIGH-SPEED / RF" }
                        }
                        h3 {
                            class: "text-lg font-bold text-white font-sans",
                            "전자 회로 & 4/6층 PCB 설계"
                        }
                        p {
                            class: "text-zinc-300 text-xs sm:text-sm font-sans leading-relaxed",
                            "KiCad 9 기반으로 4~6층 고밀도 PCB를 제도합니다. ESP32, Nordic nRF, STM32, RP2040 등 주요 SoC와 전원단(Buck/Boost, LDO), 배터리 충전·PCM 보호 회로, 고속 SPI/RF 임피던스 제어 배선을 완벽히 설계합니다."
                        }
                        div {
                            class: "pt-2 flex flex-wrap gap-1.5 text-[11px] text-zinc-400",
                            span { class: "px-2 py-0.5 rounded bg-white/[0.03] border border-white/[0.06]", "KiCad 9" }
                            span { class: "px-2 py-0.5 rounded bg-white/[0.03] border border-white/[0.06]", "4/6-Layer Stackup" }
                            span { class: "px-2 py-0.5 rounded bg-white/[0.03] border border-white/[0.06]", "Impedance Control" }
                            span { class: "px-2 py-0.5 rounded bg-white/[0.03] border border-white/[0.06]", "Battery PCM" }
                            span { class: "px-2 py-0.5 rounded bg-white/[0.03] border border-white/[0.06]", "JLCPCB SMT" }
                        }
                    }

                    // 02: MCAD
                    div {
                        class: "p-6 rounded border border-white/[0.08] bg-[#0A0D13] hover:border-white/20 transition-all space-y-4",
                        div {
                            class: "flex items-center justify-between text-xs pb-3 border-b border-white/[0.06]",
                            span { class: "text-[#FF5722] font-bold", "02 // MCAD" },
                            span { class: "text-zinc-500", "B-REP / 0.00mm³ COLLISION" }
                        }
                        h3 {
                            class: "text-lg font-bold text-white font-sans",
                            "3D 기구 & 맞춤 외함 설계"
                        }
                        p {
                            class: "text-zinc-300 text-xs sm:text-sm font-sans leading-relaxed",
                            "오픈캐스케이드 B-Rep 솔리드 커널 기반으로 회로 부품의 3D STEP 모델과 0.00mm 단위로 결합되는 외함을 설계합니다. HP MJF 나일론 3D 프린팅, SLA 투명 레진, 또는 정밀 CNC 알루미늄 가공 DFM 공차를 완벽 준수합니다."
                        }
                        div {
                            class: "pt-2 flex flex-wrap gap-1.5 text-[11px] text-zinc-400",
                            span { class: "px-2 py-0.5 rounded bg-white/[0.03] border border-white/[0.06]", "B-Rep 3D CAD" }
                            span { class: "px-2 py-0.5 rounded bg-white/[0.03] border border-white/[0.06]", "0.00 mm³ 간섭 검증" }
                            span { class: "px-2 py-0.5 rounded bg-white/[0.03] border border-white/[0.06]", "HP MJF PA12" }
                            span { class: "px-2 py-0.5 rounded bg-white/[0.03] border border-white/[0.06]", "CNC 알루미늄" }
                            span { class: "px-2 py-0.5 rounded bg-white/[0.03] border border-white/[0.06]", "JLC3DP DFM" }
                        }
                    }

                    // 03: Firmware
                    div {
                        class: "p-6 rounded border border-white/[0.08] bg-[#0A0D13] hover:border-white/20 transition-all space-y-4",
                        div {
                            class: "flex items-center justify-between text-xs pb-3 border-b border-white/[0.06]",
                            span { class: "text-[#FF5722] font-bold", "03 // FIRMWARE" },
                            span { class: "text-zinc-500", "BARE-METAL / RTOS / RUST" }
                        }
                        h3 {
                            class: "text-lg font-bold text-white font-sans",
                            "임베디드 펌웨어 & 초저전력 제어"
                        }
                        p {
                            class: "text-zinc-300 text-xs sm:text-sm font-sans leading-relaxed",
                            "고객사의 유지보수와 인수인계 편의성에 맞춰 C/C++, Arduino, ESP-IDF부터 고성능 Rust(esp-hal / embassy)까지 최적의 아키텍처로 개발합니다. Zero-Copy DMA 디스플레이 구동과 3.3V 배터리 딥슬립을 구현합니다."
                        }
                        div {
                            class: "pt-2 flex flex-wrap gap-1.5 text-[11px] text-zinc-400",
                            span { class: "px-2 py-0.5 rounded bg-white/[0.03] border border-white/[0.06]", "C / C++ / Arduino" }
                            span { class: "px-2 py-0.5 rounded bg-white/[0.03] border border-white/[0.06]", "ESP-IDF / Zephyr" }
                            span { class: "px-2 py-0.5 rounded bg-white/[0.03] border border-white/[0.06]", "Rust (esp-hal)" }
                            span { class: "px-2 py-0.5 rounded bg-white/[0.03] border border-white/[0.06]", "Zero-Copy GDMA" }
                            span { class: "px-2 py-0.5 rounded bg-white/[0.03] border border-white/[0.06]", "Low-Power Sleep" }
                        }
                    }

                    // 04: Factory Turnkey
                    div {
                        class: "p-6 rounded border border-white/[0.08] bg-[#0A0D13] hover:border-white/20 transition-all space-y-4",
                        div {
                            class: "flex items-center justify-between text-xs pb-3 border-b border-white/[0.06]",
                            span { class: "text-[#FF5722] font-bold", "04 // FACTORY & ASSEMBLY" },
                            span { class: "text-zinc-500", "SMT / SOURCING / BRING-UP" }
                        }
                        h3 {
                            class: "text-lg font-bold text-white font-sans",
                            "공장 발주 대행 & 실물 조립 브링업"
                        }
                        p {
                            class: "text-zinc-300 text-xs sm:text-sm font-sans leading-relaxed",
                            "JLCPCB PCBA 발주, JLC3DP 외함 가공, LCSC / 디바이스마트 핵심 부품 소싱을 원스톱 대행합니다. 도착한 실물 기판과 하우징을 직접 조립·납땜하고, 펌웨어를 플래싱하여 손에서 버튼을 누르면 동작하는 완제품으로 납품합니다."
                        }
                        div {
                            class: "pt-2 flex flex-wrap gap-1.5 text-[11px] text-zinc-400",
                            span { class: "px-2 py-0.5 rounded bg-white/[0.03] border border-white/[0.06]", "PCBA 발주 대행" }
                            span { class: "px-2 py-0.5 rounded bg-white/[0.03] border border-white/[0.06]", "실물 조립 & 브링업" }
                            span { class: "px-2 py-0.5 rounded bg-white/[0.03] border border-white/[0.06]", "계측 신호 검증" }
                            span { class: "px-2 py-0.5 rounded bg-white/[0.03] border border-white/[0.06]", "동작 완제품 택배 납품" }
                        }
                    }
                }
            }
        }
    }
}
