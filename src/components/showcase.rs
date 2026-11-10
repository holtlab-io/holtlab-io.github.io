use dioxus::prelude::*;

#[component]
pub fn Showcase() -> Element {
    let mut active_tab = use_signal(|| 0); // 0: MCAD, 1: ECAD, 2: Firmware

    rsx! {
        section {
            id: "showcase",
            class: "py-20 bg-[#07090E] relative border-b border-white/[0.08] font-mono",
            div {
                class: "max-w-7xl mx-auto px-4 sm:px-6 lg:px-8",
                // Header
                div {
                    class: "max-w-3xl space-y-3 mb-10",
                    div {
                        class: "inline-flex items-center gap-2 px-2.5 py-1 rounded bg-white/[0.03] border border-white/10 text-zinc-400 text-xs tracking-widest uppercase",
                        "FLAGSHIP CASE STUDY // PROVEN REFERENCE"
                    }
                    h2 {
                        class: "text-2xl sm:text-4xl font-bold text-white tracking-tight font-sans",
                        "Win95 지뢰찾기 터치 키링"
                    }
                    p {
                        class: "text-zinc-400 text-sm sm:text-base leading-relaxed font-sans",
                        "3D 기구 모델링부터 4층 PCB 제조, ESP32-C3 베어메탈 DMA 펌웨어까지 단 한 명의 엔지니어가 완성한 "
                        span { class: "text-white font-medium", "실물 턴키 개발 레퍼런스" }
                        "입니다."
                    }
                }

                // Tactile Industrial Tab Switcher
                div {
                    class: "flex flex-wrap gap-2 mb-8 text-xs",
                    button {
                        r#type: "button",
                        onclick: move |_| active_tab.set(0),
                        class: if active_tab() == 0 {
                            "px-4 py-2 rounded bg-[#FF5722] text-white font-bold border border-[#FF5722] shadow transition-all"
                        } else {
                            "px-4 py-2 rounded bg-white/[0.02] text-zinc-400 hover:text-white border border-white/10 hover:border-white/20 transition-all"
                        },
                        "01 // MCAD B-REP 외함"
                    }
                    button {
                        r#type: "button",
                        onclick: move |_| active_tab.set(1),
                        class: if active_tab() == 1 {
                            "px-4 py-2 rounded bg-[#FF5722] text-white font-bold border border-[#FF5722] shadow transition-all"
                        } else {
                            "px-4 py-2 rounded bg-white/[0.02] text-zinc-400 hover:text-white border border-white/10 hover:border-white/20 transition-all"
                        },
                        "02 // ECAD 4층 PCBA"
                    }
                    button {
                        r#type: "button",
                        onclick: move |_| active_tab.set(2),
                        class: if active_tab() == 2 {
                            "px-4 py-2 rounded bg-[#FF5722] text-white font-bold border border-[#FF5722] shadow transition-all"
                        } else {
                            "px-4 py-2 rounded bg-white/[0.02] text-zinc-400 hover:text-white border border-white/10 hover:border-white/20 transition-all"
                        },
                        "03 // 펌웨어 60Hz GDMA"
                    }
                }

                // High-Density Engineering Inspection Bench
                div {
                    class: "hardware-panel rounded-lg border border-white/[0.08] bg-[#0A0D14] p-5 sm:p-8 shadow-2xl relative",
                    if active_tab() == 0 {
                        // MCAD Tab Content
                        div {
                            class: "grid grid-cols-1 lg:grid-cols-12 gap-8 items-center",
                            div {
                                class: "lg:col-span-7 space-y-6",
                                div {
                                    class: "flex flex-wrap items-center gap-2 text-xs",
                                    span { class: "px-2 py-0.5 rounded bg-white/[0.05] text-[#FF5722] border border-[#FF5722]/30 font-bold", "MCAD // BUILD123D B-REP" }
                                    span { class: "text-zinc-500", "•" }
                                    span { class: "text-emerald-400", "COLLISION: 0.00 mm³" }
                                    span { class: "text-zinc-500", "•" }
                                    span { class: "text-zinc-400", "TOLERANCE: ±0.05mm" }
                                }
                                h3 {
                                    class: "text-xl sm:text-2xl font-bold text-white font-sans",
                                    "0.00mm 정밀 공차의 산업용 MJF 나일론 외함"
                                }
                                p {
                                    class: "text-zinc-300 text-xs sm:text-sm font-sans leading-relaxed",
                                    "ESP32-C3 모듈, 2.0인치 터치 LCD, 400mAh 리튬폴리머 배터리, 1027 진동 모터가 13.78mm 두께 안에 0.01mm의 간섭 없이 정밀 결합되도록 오픈캐스케이드 B-Rep 솔리드 커널 기반으로 설계되었습니다."
                                }
                                // Technical Parameter Matrix
                                div {
                                    class: "border border-white/[0.08] rounded divide-y divide-white/[0.06] text-xs font-mono bg-white/[0.01]",
                                    div {
                                        class: "grid grid-cols-2 p-2.5 sm:px-4",
                                        span { class: "text-zinc-500 uppercase", "외형 치수 (W×H×D)" },
                                        span { class: "text-white font-semibold", "43.40 × 69.20 × 13.78 mm" }
                                    }
                                    div {
                                        class: "grid grid-cols-2 p-2.5 sm:px-4",
                                        span { class: "text-zinc-500 uppercase", "제작 공법 & 소재" },
                                        span { class: "text-white font-semibold", "HP MJF Nylon PA12 (블랙)" }
                                    }
                                    div {
                                        class: "grid grid-cols-2 p-2.5 sm:px-4",
                                        span { class: "text-zinc-500 uppercase", "파스너 체결" },
                                        span { class: "text-white font-semibold", "M1.4 4점 나사산 체결 보스" }
                                    }
                                    div {
                                        class: "grid grid-cols-2 p-2.5 sm:px-4",
                                        span { class: "text-zinc-500 uppercase", "디스플레이 마운트" },
                                        span { class: "text-white font-semibold", "듀얼 개구부 일체형 베젤 립" }
                                    }
                                }
                            }
                            div {
                                class: "lg:col-span-5 relative rounded bg-[#06080C] border border-white/[0.08] aspect-square sm:aspect-[4/3] flex items-center justify-center p-3",
                                img {
                                    src: "assets/showcase/mcad_exploded.png",
                                    alt: "MCAD Exploded Assembly View",
                                    class: "w-full h-full object-contain"
                                }
                                div {
                                    class: "absolute bottom-2 right-2 px-2 py-0.5 rounded bg-black/80 border border-white/10 text-[10px] text-zinc-400",
                                    "EXPLODED ASSEMBLY // STEP VERIFIED"
                                }
                            }
                        }
                    } else if active_tab() == 1 {
                        // ECAD Tab Content
                        div {
                            class: "grid grid-cols-1 lg:grid-cols-12 gap-8 items-center",
                            div {
                                class: "lg:col-span-7 space-y-6",
                                div {
                                    class: "flex flex-wrap items-center gap-2 text-xs",
                                    span { class: "px-2 py-0.5 rounded bg-white/[0.05] text-[#FF5722] border border-[#FF5722]/30 font-bold", "ECAD // KICAD 9 4-LAYER" }
                                    span { class: "text-zinc-500", "•" }
                                    span { class: "text-emerald-400", "STACKUP: JLC04161H-7628" }
                                    span { class: "text-zinc-500", "•" }
                                    span { class: "text-zinc-400", "IMPEDANCE: 50Ω RF / 90Ω USB" }
                                }
                                h3 {
                                    class: "text-xl sm:text-2xl font-bold text-white font-sans",
                                    "4층 고밀도 PCB & 하드웨어 PCM 보호 회로"
                                }
                                p {
                                    class: "text-zinc-300 text-xs sm:text-sm font-sans leading-relaxed",
                                    "JLCPCB 4층 스택업 기반으로 ESP32-C3의 2.4GHz RF 안테나 매칭, 26.67MHz SPI 고속 버스 배선, DW01A+8205A 하드웨어 PCM 배터리 보호 회로를 40×64.6mm 초소형 보드에 직접 설계 및 실장했습니다."
                                }
                                // Technical Parameter Matrix
                                div {
                                    class: "border border-white/[0.08] rounded divide-y divide-white/[0.06] text-xs font-mono bg-white/[0.01]",
                                    div {
                                        class: "grid grid-cols-2 p-2.5 sm:px-4",
                                        span { class: "text-zinc-500 uppercase", "PCB 크기 & 층수" },
                                        span { class: "text-white font-semibold", "40.0 × 64.6 mm (4-Layer 1.6mm)" }
                                    }
                                    div {
                                        class: "grid grid-cols-2 p-2.5 sm:px-4",
                                        span { class: "text-zinc-500 uppercase", "메인 프로세서" },
                                        span { class: "text-white font-semibold", "ESP32-C3-WROOM-02-N4 (160MHz)" }
                                    }
                                    div {
                                        class: "grid grid-cols-2 p-2.5 sm:px-4",
                                        span { class: "text-zinc-500 uppercase", "디스플레이 & 터치 IC" },
                                        span { class: "text-white font-semibold", "ST7789T3 (SPI) + CST816D (I2C)" }
                                    }
                                    div {
                                        class: "grid grid-cols-2 p-2.5 sm:px-4",
                                        span { class: "text-zinc-500 uppercase", "배터리 PCM 컷오프" },
                                        span { class: "text-white font-semibold", "DW01A + 8205A (2.45V 컷오프)" }
                                    }
                                }
                            }
                            div {
                                class: "lg:col-span-5 relative rounded bg-[#06080C] border border-white/[0.08] aspect-square sm:aspect-[4/3] flex items-center justify-center p-3",
                                img {
                                    src: "assets/showcase/pcb_iso.png",
                                    alt: "ECAD 4-Layer PCB Render",
                                    class: "w-full h-full object-contain"
                                }
                                div {
                                    class: "absolute bottom-2 right-2 px-2 py-0.5 rounded bg-black/80 border border-white/10 text-[10px] text-zinc-400",
                                    "4-LAYER ISOMETRIC PCBA"
                                }
                            }
                        }
                    } else {
                        // Firmware Tab Content
                        div {
                            class: "grid grid-cols-1 lg:grid-cols-12 gap-8 items-center",
                            div {
                                class: "lg:col-span-7 space-y-6",
                                div {
                                    class: "flex flex-wrap items-center gap-2 text-xs",
                                    span { class: "px-2 py-0.5 rounded bg-white/[0.05] text-[#FF5722] border border-[#FF5722]/30 font-bold", "FIRMWARE // ZERO-COPY GDMA" }
                                    span { class: "text-zinc-500", "•" }
                                    span { class: "text-emerald-400", "CLK: 26.67 MHz (80MHz / 3)" }
                                    span { class: "text-zinc-500", "•" }
                                    span { class: "text-zinc-400", "LATENCY: 0.79ms / TILE" }
                                }
                                h3 {
                                    class: "text-xl sm:text-2xl font-bold text-white font-sans",
                                    "26.67MHz SPI & Zero-Copy AHB GDMA 초고속 구동"
                                }
                                p {
                                    class: "text-zinc-300 text-xs sm:text-sm font-sans leading-relaxed",
                                    "캔 내부 499Ω 직렬 댐핑 저항 한계를 극복하고 26.67MHz 고속 SPI 버스를 안정화했습니다. 153.6KB SRAM 백버퍼와 AHB GDMA를 통해 CPU 개입 없이 60Hz 고속 부분 갱신을 실현했습니다."
                                }
                                // Technical Parameter Matrix
                                div {
                                    class: "border border-white/[0.08] rounded divide-y divide-white/[0.06] text-xs font-mono bg-white/[0.01]",
                                    div {
                                        class: "grid grid-cols-2 p-2.5 sm:px-4",
                                        span { class: "text-zinc-500 uppercase", "SPI 버스 클럭" },
                                        span { class: "text-white font-semibold", "26.67 MHz (80MHz / 3 Divider)" }
                                    }
                                    div {
                                        class: "grid grid-cols-2 p-2.5 sm:px-4",
                                        span { class: "text-zinc-500 uppercase", "부분 갱신 지연" },
                                        span { class: "text-white font-semibold", "타일당 0.79ms (Zero-Copy GDMA)" }
                                    }
                                    div {
                                        class: "grid grid-cols-2 p-2.5 sm:px-4",
                                        span { class: "text-zinc-500 uppercase", "터치 인터럽트" },
                                        span { class: "text-white font-semibold", "GPIO FallingEdge 하드웨어 IRQ" }
                                    }
                                    div {
                                        class: "grid grid-cols-2 p-2.5 sm:px-4",
                                        span { class: "text-zinc-500 uppercase", "전원 관리 아키텍처" },
                                        span { class: "text-white font-semibold", "3.30V ADC 모니터링 & 자동 딥슬립" }
                                    }
                                }
                            }
                            div {
                                class: "lg:col-span-5 relative rounded bg-[#06080C] border border-white/[0.08] aspect-square sm:aspect-[4/3] flex items-center justify-center p-3",
                                img {
                                    src: "assets/showcase/mcad_drawing.png",
                                    alt: "Firmware Timing Blueprint",
                                    class: "w-full h-full object-contain"
                                }
                                div {
                                    class: "absolute bottom-2 right-2 px-2 py-0.5 rounded bg-black/80 border border-white/10 text-[10px] text-zinc-400",
                                    "TIMING DIAGRAM & BLUEPRINT"
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
