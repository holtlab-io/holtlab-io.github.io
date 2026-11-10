//! ## Overview
//! Hardware capability matrix outlining the studio's 4 core service disciplines.
//! Covers KiCad 9 multi-layer ECAD, Build123d B-Rep MCAD, bare-metal firmware, and turnkey factory assembly.
//!
//! ## Search Tags
//! #services, #capabilities, #ecad, #mcad, #firmware, #manufacturing

use dioxus::prelude::*;

/// Capabilities overview section presenting 4 core turnkey engineering disciplines.
#[component]
pub fn Services() -> Element {
    rsx! {
        section {
            id: "capabilities",
            class: "py-20 bg-[#FFFFFF] relative border-b border-zinc-200 font-mono",
            div {
                class: "max-w-7xl mx-auto px-4 sm:px-6 lg:px-8",
                // Header
                div {
                    class: "max-w-3xl space-y-3 mb-12",
                    div {
                        class: "inline-flex items-center gap-2 px-2.5 py-1 rounded-full bg-zinc-100 border border-zinc-200 text-zinc-600 text-xs tracking-wider uppercase",
                        "UNIFIED HARDWARE CAPABILITIES"
                    }
                    h2 {
                        class: "text-2xl sm:text-4xl font-bold text-zinc-900 tracking-tight font-sans",
                        "아이디어 스케치부터 실물까지 "
                        br {}
                        span { class: "text-zinc-500", "4대 하드웨어 전문 영역" }
                    }
                    p {
                        class: "text-zinc-600 text-sm sm:text-base leading-relaxed font-sans",
                        "공정마다 업체를 쪼개서 발주할 필요 없이, 단일 리드 엔지니어가 전체 공정을 통일된 공차 규격으로 완주합니다."
                    }
                }

                // 4 Capability Technical Panels
                div {
                    class: "grid grid-cols-1 md:grid-cols-2 gap-5",
                    EcadCard {}
                    McadCard {}
                    FirmwareCard {}
                    FactoryCard {}
                }
            }
        }
    }
}

/// ECAD hardware capability card.
#[component]
fn EcadCard() -> Element {
    rsx! {
        div {
            class: "p-6 rounded-xl border border-zinc-200 bg-[#FAFAFA] hover:border-zinc-300 shadow-2xs transition-all space-y-4",
            div {
                class: "flex items-center justify-between text-xs pb-3 border-b border-zinc-200/80",
                span { class: "text-zinc-900 font-bold", "01 // ECAD" }
                span { class: "text-zinc-500", "KICAD 9 / HIGH-SPEED / RF" }
            }
            h3 {
                class: "text-lg font-bold text-zinc-900 font-sans",
                "전자 회로 & 4/6층 PCB 설계"
            }
            p {
                class: "text-zinc-600 text-xs sm:text-sm font-sans leading-relaxed",
                "KiCad 9 기반으로 4~6층 고밀도 PCB를 제도합니다. ESP32, Nordic nRF, STM32, RP2040 등 주요 SoC와 전원단(Buck/Boost, LDO), 배터리 충전·PCM 보호 회로, 고속 SPI/RF 임피던스 제어 배선을 완벽히 설계합니다."
            }
            div {
                class: "pt-2 flex flex-wrap gap-1.5 text-[11px] text-zinc-600",
                span { class: "px-2 py-0.5 rounded bg-white border border-zinc-200", "KiCad 9" }
                span { class: "px-2 py-0.5 rounded bg-white border border-zinc-200", "4/6-Layer Stackup" }
                span { class: "px-2 py-0.5 rounded bg-white border border-zinc-200", "Impedance Control" }
                span { class: "px-2 py-0.5 rounded bg-white border border-zinc-200", "Battery PCM" }
                span { class: "px-2 py-0.5 rounded bg-white border border-zinc-200", "JLCPCB SMT" }
            }
        }
    }
}

/// MCAD enclosure engineering capability card.
#[component]
fn McadCard() -> Element {
    rsx! {
        div {
            class: "p-6 rounded-xl border border-zinc-200 bg-[#FAFAFA] hover:border-zinc-300 shadow-2xs transition-all space-y-4",
            div {
                class: "flex items-center justify-between text-xs pb-3 border-b border-zinc-200/80",
                span { class: "text-zinc-900 font-bold", "02 // MCAD" }
                span { class: "text-zinc-500", "B-REP / 0.00mm³ COLLISION" }
            }
            h3 {
                class: "text-lg font-bold text-zinc-900 font-sans",
                "3D 기구 & 맞춤 외함 설계"
            }
            p {
                class: "text-zinc-600 text-xs sm:text-sm font-sans leading-relaxed",
                "오픈캐스케이드 B-Rep 솔리드 커널 기반으로 회로 부품의 3D STEP 모델과 0.00mm 단위로 결합되는 외함을 설계합니다. HP MJF 나일론 3D 프린팅, SLA 투명 레진, 또는 정밀 CNC 알루미늄 가공 DFM 공차를 완벽 준수합니다."
            }
            div {
                class: "pt-2 flex flex-wrap gap-1.5 text-[11px] text-zinc-600",
                span { class: "px-2 py-0.5 rounded bg-white border border-zinc-200", "B-Rep 3D CAD" }
                span { class: "px-2 py-0.5 rounded bg-white border border-zinc-200", "0.00 mm³ 간섭 검증" }
                span { class: "px-2 py-0.5 rounded bg-white border border-zinc-200", "HP MJF PA12" }
                span { class: "px-2 py-0.5 rounded bg-white border border-zinc-200", "CNC 알루미늄" }
                span { class: "px-2 py-0.5 rounded bg-white border border-zinc-200", "JLC3DP DFM" }
            }
        }
    }
}

/// Bare-metal and RTOS firmware engineering capability card.
#[component]
fn FirmwareCard() -> Element {
    rsx! {
        div {
            class: "p-6 rounded-xl border border-zinc-200 bg-[#FAFAFA] hover:border-zinc-300 shadow-2xs transition-all space-y-4",
            div {
                class: "flex items-center justify-between text-xs pb-3 border-b border-zinc-200/80",
                span { class: "text-zinc-900 font-bold", "03 // FIRMWARE" }
                span { class: "text-zinc-500", "BARE-METAL / RTOS / RUST" }
            }
            h3 {
                class: "text-lg font-bold text-zinc-900 font-sans",
                "임베디드 펌웨어 & 초저전력 제어"
            }
            p {
                class: "text-zinc-600 text-xs sm:text-sm font-sans leading-relaxed",
                "고객사의 유지보수와 인수인계 편의성에 맞춰 C/C++, Arduino, ESP-IDF부터 고성능 Rust(esp-hal / embassy)까지 최적의 아키텍처로 개발합니다. Zero-Copy DMA 디스플레이 구동과 3.3V 배터리 딥슬립을 구현합니다."
            }
            div {
                class: "pt-2 flex flex-wrap gap-1.5 text-[11px] text-zinc-600",
                span { class: "px-2 py-0.5 rounded bg-white border border-zinc-200", "C / C++ / Arduino" }
                span { class: "px-2 py-0.5 rounded bg-white border border-zinc-200", "ESP-IDF / Zephyr" }
                span { class: "px-2 py-0.5 rounded bg-white border border-zinc-200", "Rust (esp-hal)" }
                span { class: "px-2 py-0.5 rounded bg-white border border-zinc-200", "Zero-Copy GDMA" }
                span { class: "px-2 py-0.5 rounded bg-white border border-zinc-200", "Low-Power Sleep" }
            }
        }
    }
}

/// Turnkey factory manufacturing and hardware bring-up card.
#[component]
fn FactoryCard() -> Element {
    rsx! {
        div {
            class: "p-6 rounded-xl border border-zinc-200 bg-[#FAFAFA] hover:border-zinc-300 shadow-2xs transition-all space-y-4",
            div {
                class: "flex items-center justify-between text-xs pb-3 border-b border-zinc-200/80",
                span { class: "text-zinc-900 font-bold", "04 // FACTORY & ASSEMBLY" }
                span { class: "text-zinc-500", "SMT / SOURCING / BRING-UP" }
            }
            h3 {
                class: "text-lg font-bold text-zinc-900 font-sans",
                "원스톱 부품 소싱 & 공장 직발주 양산"
            }
            p {
                class: "text-zinc-600 text-xs sm:text-sm font-sans leading-relaxed",
                "LCSC, JLCPCB, 국내 총판 등 글로벌 공급망에서 부품 재고와 COTS 규격을 사전 동결합니다. 해외 PCB/SMT 조립 및 3D 하우징 가공을 직접 발주하고 실물 수령 후 전원 인가·신호 브링업까지 완주합니다."
            }
            div {
                class: "pt-2 flex flex-wrap gap-1.5 text-[11px] text-zinc-600",
                span { class: "px-2 py-0.5 rounded bg-white border border-zinc-200", "JLCPCB SMT 직발주" }
                span { class: "px-2 py-0.5 rounded bg-white border border-zinc-200", "LCSC 부품 수급 락" }
                span { class: "px-2 py-0.5 rounded bg-white border border-zinc-200", "전원/신호 브링업" }
                span { class: "px-2 py-0.5 rounded bg-white border border-zinc-200", "초기 불량 전수 검수" }
                span { class: "px-2 py-0.5 rounded bg-white border border-zinc-200", "완성품 1:1 인도" }
            }
        }
    }
}
