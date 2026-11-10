use dioxus::prelude::*;

#[component]
pub fn Showcase() -> Element {
    let mut active_tab = use_signal(|| 0); // 0: MCAD, 1: ECAD, 2: Firmware

    rsx! {
        section {
            id: "showcase",
            class: "py-24 bg-[#0B0F17] relative border-t border-white/[0.08]",
            div {
                class: "max-w-7xl mx-auto px-4 sm:px-6 lg:px-8",
                // Header
                div {
                    class: "max-w-3xl mx-auto text-center space-y-4 mb-14",
                    div {
                        class: "inline-flex items-center gap-2 px-3 py-1 rounded-full bg-sky-500/10 border border-sky-500/20 text-sky-400 text-xs font-mono font-medium",
                        "FLAGSHIP CASE STUDY // VERIFIED REFERENCE"
                    }
                    h2 {
                        class: "text-3xl sm:text-4xl lg:text-5xl font-extrabold text-white tracking-tight",
                        "Win95 지뢰찾기 터치 키링"
                    }
                    p {
                        class: "text-slate-300 text-base sm:text-lg leading-relaxed",
                        "기구 모델링부터 4층 PCB 제조, ESP32-C3 고속 DMA 펌웨어까지 단 한 명의 엔지니어가 완성한 "
                        strong { class: "text-white font-semibold", "실물 턴키 개발 레퍼런스" }
                        "입니다."
                    }
                }

                // Interactive Tab Buttons
                div {
                    class: "flex justify-center mb-10",
                    div {
                        class: "inline-flex p-1.5 rounded-xl bg-white/[0.04] border border-white/10 backdrop-blur-md",
                        button {
                            r#type: "button",
                            onclick: move |_| active_tab.set(0),
                            class: if active_tab() == 0 {
                                "px-5 py-2.5 rounded-lg bg-orange-500 text-white font-semibold text-sm shadow-md transition-all"
                            } else {
                                "px-5 py-2.5 rounded-lg text-slate-400 hover:text-white font-medium text-sm transition-all"
                            },
                            "01. 기구 & 3D 외함 (MCAD)"
                        }
                        button {
                            r#type: "button",
                            onclick: move |_| active_tab.set(1),
                            class: if active_tab() == 1 {
                                "px-5 py-2.5 rounded-lg bg-orange-500 text-white font-semibold text-sm shadow-md transition-all"
                            } else {
                                "px-5 py-2.5 rounded-lg text-slate-400 hover:text-white font-medium text-sm transition-all"
                            },
                            "02. 4층 초소형 PCB (ECAD)"
                        }
                        button {
                            r#type: "button",
                            onclick: move |_| active_tab.set(2),
                            class: if active_tab() == 2 {
                                "px-5 py-2.5 rounded-lg bg-orange-500 text-white font-semibold text-sm shadow-md transition-all"
                            } else {
                                "px-5 py-2.5 rounded-lg text-slate-400 hover:text-white font-medium text-sm transition-all"
                            },
                            "03. 펌웨어 & 60Hz DMA"
                        }
                    }
                }

                // Showcase Detail Card
                div {
                    class: "rounded-3xl border border-white/10 bg-[#121722] p-6 sm:p-10 shadow-2xl backdrop-blur-xl relative overflow-hidden",
                    if active_tab() == 0 {
                        // MCAD Tab Content
                        div {
                            class: "grid grid-cols-1 lg:grid-cols-12 gap-8 items-center",
                            div {
                                class: "lg:col-span-7 space-y-6",
                                div {
                                    class: "flex items-center gap-3",
                                    span { class: "px-2.5 py-1 rounded bg-orange-500/20 text-orange-400 border border-orange-500/30 text-xs font-mono font-semibold", "MCAD // BUILD123D B-REP" }
                                    span { class: "text-xs font-mono text-slate-400", "COLLISION: 0.00 mm³" }
                                }
                                h3 {
                                    class: "text-2xl sm:text-3xl font-bold text-white",
                                    "0.00mm 정밀 공차의 산업용 MJF 나일론 외함"
                                }
                                p {
                                    class: "text-slate-300 text-sm sm:text-base leading-relaxed",
                                    "ESP32-C3 모듈, 2.0인치 터치 LCD, 400mAh 리튬폴리머 배터리, 1027 진동 모터가 13.78mm 두께 안에 0.01mm의 간섭 없이 정밀 결합되도록 오픈캐스케이드 B-Rep 커널 기반으로 설계되었습니다."
                                }
                                // Tech Specs Grid
                                div {
                                    class: "grid grid-cols-2 gap-4 text-xs font-mono",
                                    div {
                                        class: "p-3.5 rounded-xl bg-white/[0.03] border border-white/[0.06]",
                                        div { class: "text-slate-400", "외형 치수" }
                                        div { class: "text-base font-semibold text-white mt-1", "43.40 × 69.20 × 13.78 mm" }
                                    }
                                    div {
                                        class: "p-3.5 rounded-xl bg-white/[0.03] border border-white/[0.06]",
                                        div { class: "text-slate-400", "제작 공법" }
                                        div { class: "text-base font-semibold text-white mt-1", "HP MJF Nylon PA12 (블랙)" }
                                    }
                                    div {
                                        class: "p-3.5 rounded-xl bg-white/[0.03] border border-white/[0.06]",
                                        div { class: "text-slate-400", "조립 구조" }
                                        div { class: "text-base font-semibold text-white mt-1", "M1.4 4점 체결 나사산 보스" }
                                    }
                                    div {
                                        class: "p-3.5 rounded-xl bg-white/[0.03] border border-white/[0.06]",
                                        div { class: "text-slate-400", "베젤 구조" }
                                        div { class: "text-base font-semibold text-white mt-1", "듀얼 개구부 트랙패드 립" }
                                    }
                                }
                            }
                            div {
                                class: "lg:col-span-5 relative rounded-2xl overflow-hidden bg-[#0A0D14] border border-white/10 aspect-square sm:aspect-[4/3] flex items-center justify-center p-2",
                                img {
                                    src: "assets/showcase/mcad_exploded.png",
                                    alt: "MCAD Exploded View",
                                    class: "w-full h-full object-contain hover:scale-105 transition-transform duration-500"
                                }
                                div {
                                    class: "absolute bottom-3 right-3 px-2.5 py-1 rounded bg-black/70 backdrop-blur-md border border-white/20 text-[11px] font-mono text-slate-300",
                                    "EXPLODED ASSEMBLY VIEW"
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
                                    class: "flex items-center gap-3",
                                    span { class: "px-2.5 py-1 rounded bg-sky-500/20 text-sky-400 border border-sky-500/30 text-xs font-mono font-semibold", "ECAD // KICAD 9 4-LAYER" }
                                    span { class: "text-xs font-mono text-slate-400", "IMPEDANCE: 50Ω / 90Ω CONTROLLED" }
                                }
                                h3 {
                                    class: "text-2xl sm:text-3xl font-bold text-white",
                                    "4층 고밀도 PCB & 하드웨어 배터리 PCM 회로"
                                }
                                p {
                                    class: "text-slate-300 text-sm sm:text-base leading-relaxed",
                                    "JLCPCB 4층 스택업(JLC04161H-7628) 기반으로 ESP32-C3의 2.4GHz RF 안테나 매칭, 26.67MHz SPI 고속 버스 배선, 그리고 DW01A+8205A 하드웨어 PCM 2.45V 차단 회로를 40×64.6mm 초소형 면적에 집적했습니다."
                                }
                                // Tech Specs Grid
                                div {
                                    class: "grid grid-cols-2 gap-4 text-xs font-mono",
                                    div {
                                        class: "p-3.5 rounded-xl bg-white/[0.03] border border-white/[0.06]",
                                        div { class: "text-slate-400", "PCB 크기 & 층수" }
                                        div { class: "text-base font-semibold text-white mt-1", "40.0 × 64.6 mm (4-Layer)" }
                                    }
                                    div {
                                        class: "p-3.5 rounded-xl bg-white/[0.03] border border-white/[0.06]",
                                        div { class: "text-slate-400", "메인 SoC" }
                                        div { class: "text-base font-semibold text-white mt-1", "ESP32-C3 RISC-V 160MHz" }
                                    }
                                    div {
                                        class: "p-3.5 rounded-xl bg-white/[0.03] border border-white/[0.06]",
                                        div { class: "text-slate-400", "디스플레이 & 터치" }
                                        div { class: "text-base font-semibold text-white mt-1", "2.0\" ST7789T3 + CST816D" }
                                    }
                                    div {
                                        class: "p-3.5 rounded-xl bg-white/[0.03] border border-white/[0.06]",
                                        div { class: "text-slate-400", "배터리 보호 회로" }
                                        div { class: "text-base font-semibold text-white mt-1", "DW01A + 8205A PCM (2.45V)" }
                                    }
                                }
                            }
                            div {
                                class: "lg:col-span-5 relative rounded-2xl overflow-hidden bg-[#0A0D14] border border-white/10 aspect-square sm:aspect-[4/3] flex items-center justify-center p-2",
                                img {
                                    src: "assets/showcase/pcb_iso.png",
                                    alt: "ECAD 4-Layer PCB Render",
                                    class: "w-full h-full object-contain hover:scale-105 transition-transform duration-500"
                                }
                                div {
                                    class: "absolute bottom-3 right-3 px-2.5 py-1 rounded bg-black/70 backdrop-blur-md border border-white/20 text-[11px] font-mono text-slate-300",
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
                                    class: "flex items-center gap-3",
                                    span { class: "px-2.5 py-1 rounded bg-emerald-500/20 text-emerald-400 border border-emerald-500/30 text-xs font-mono font-semibold", "FIRMWARE // ZERO-COPY GDMA" }
                                    span { class: "text-xs font-mono text-slate-400", "REFRESH: 60Hz AHB DMA" }
                                }
                                h3 {
                                    class: "text-2xl sm:text-3xl font-bold text-white",
                                    "26.67MHz SPI & Zero-Copy AHB GDMA 초고속 구동"
                                }
                                p {
                                    class: "text-slate-300 text-sm sm:text-base leading-relaxed",
                                    "모듈 내부의 499Ω 직렬 댐핑 저항 한계를 극복하고 26.67MHz 고속 SPI 버스를 안정화했습니다. 153.6KB SRAM 백버퍼와 AHB GDMA를 통해 CPU 점유율을 최소화하면서 60Hz 부드러운 터치 애니메이션을 실현했습니다."
                                }
                                // Tech Specs Grid
                                div {
                                    class: "grid grid-cols-2 gap-4 text-xs font-mono",
                                    div {
                                        class: "p-3.5 rounded-xl bg-white/[0.03] border border-white/[0.06]",
                                        div { class: "text-slate-400", "SPI 버스 대역폭" }
                                        div { class: "text-base font-semibold text-white mt-1", "26.67 MHz (80MHz / 3)" }
                                    }
                                    div {
                                        class: "p-3.5 rounded-xl bg-white/[0.03] border border-white/[0.06]",
                                        div { class: "text-slate-400", "부분 갱신 지연" }
                                        div { class: "text-base font-semibold text-white mt-1", "타일당 0.79ms (초고속)" }
                                    }
                                    div {
                                        class: "p-3.5 rounded-xl bg-white/[0.03] border border-white/[0.06]",
                                        div { class: "text-slate-400", "터치 반응 방식" }
                                        div { class: "text-base font-semibold text-white mt-1", "FallingEdge 하드웨어 IRQ" }
                                    }
                                    div {
                                        class: "p-3.5 rounded-xl bg-white/[0.03] border border-white/[0.06]",
                                        div { class: "text-slate-400", "배터리 절전 모드" }
                                        div { class: "text-base font-semibold text-white mt-1", "3.30V 자동 슬립 & 60초 디밍" }
                                    }
                                }
                            }
                            div {
                                class: "lg:col-span-5 relative rounded-2xl overflow-hidden bg-[#0A0D14] border border-white/10 aspect-square sm:aspect-[4/3] flex items-center justify-center p-2",
                                img {
                                    src: "assets/showcase/mcad_drawing.png",
                                    alt: "Firmware Timing & Hardware Drawing",
                                    class: "w-full h-full object-contain hover:scale-105 transition-transform duration-500"
                                }
                                div {
                                    class: "absolute bottom-3 right-3 px-2.5 py-1 rounded bg-black/70 backdrop-blur-md border border-white/20 text-[11px] font-mono text-slate-300",
                                    "ENGINEERING BLUEPRINT & TELEMETRY"
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
