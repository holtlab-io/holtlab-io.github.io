//! ## Overview
//! Physical prototype verification gallery rendered on the case study showcase page.
//! Displays real studio photography of the powered-on device, fabricated 4-layer SMT PCBA, and MJF PA12 enclosure internals.
//!
//! ## Search Tags
//! #hardware-proof, #prototype, #pcba, #bring-up, #photography, #showcase
//!
//! ## Submodules
//! None. This module provides atomic physical prototype gallery components.

use dioxus::prelude::*;
use crate::i18n::Language;

#[derive(Clone, Copy, PartialEq, Eq)]
struct ProofCardData {
    tag: &'static str,
    title: &'static str,
    desc: &'static str,
    img_src: &'static str,
    img_alt: &'static str,
}

fn proof_cards_en() -> (&'static str, &'static str, &'static str, [ProofCardData; 3]) {
    (
        "PHYSICAL PROTOTYPE // BENCH BRING-UP",
        "From 3D CAD to Powered-On Hardware.",
        "Fabricated enclosure, 4-layer SMT circuit board, and live 60Hz display firmware verified through hands-on bench assembly.",
        [
            ProofCardData {
                tag: "01 // LIVE FIRMWARE & UI",
                title: "Assembled Device & 60Hz Display",
                desc: "Powered-on 2.0\" IPS display and capacitive trackpad cursor running inside the matte black MJF PA12 enclosure.",
                img_src: "/assets/showcase/real_front_on.jpg",
                img_alt: "Powered-on Win95 Minesweeper Touch Keyring prototype",
            },
            ProofCardData {
                tag: "02 // SMT PCBA & SHELL",
                title: "4-Layer SMT Board & PA12 Housing",
                desc: "Custom 4-layer PCB populated with ESP32-C3 wireless module, USB-C, and dual FPC connectors beside the nylon rear shell.",
                img_src: "/assets/showcase/real_pcba_shell_iso.jpg",
                img_alt: "4-layer SMT PCBA beside MJF PA12 rear enclosure",
            },
            ProofCardData {
                tag: "03 // INTERNAL FIT CHECK",
                title: "Component Layout & Cradle Alignment",
                desc: "Top-down inspection of the LiPo battery pocket, mounting bosses, and switch/port cutouts aligned for clean mechanical fit.",
                img_src: "/assets/showcase/real_knolling_top.jpg",
                img_alt: "Top-down flat-lay of SMT PCBA and internal enclosure ribs",
            },
        ],
    )
}

fn proof_cards_ko() -> (&'static str, &'static str, &'static str, [ProofCardData; 3]) {
    (
        "실물 시제품 검증 // 벤치 브링업",
        "3D 설계 도면에서 실물 브링업까지",
        "기구 외함과 4층 SMT 기판을 실제 제작·조립하고, 60Hz 그래픽 UI와 정전식 터치 펌웨어 구동을 벤치에서 직접 검증한 실물 사진입니다.",
        [
            ProofCardData {
                tag: "01 // 실기 구동 · UI 브링업",
                title: "외함 조립 완료 및 60Hz 화면 구동",
                desc: "매트 블랙 MJF PA12 나일론 외함에 조립된 2.0인치 IPS 디스플레이와 정전식 트랙패드 커서 UI의 실제 동작 화면입니다.",
                img_src: "/assets/showcase/real_front_on.jpg",
                img_alt: "실물 조립 및 전원 구동 중인 마인스위퍼 터치 키링",
            },
            ProofCardData {
                tag: "02 // 다층 SMT 기판 · 외함",
                title: "4층 제어 임피던스 기판과 하부 쉘",
                desc: "ESP32-C3 무선 모듈, USB-C 충전 포트, 듀얼 FPC 커넥터가 실장된 4층 기판과 내부 지지 리브가 성형된 하부 외함입니다.",
                img_src: "/assets/showcase/real_pcba_shell_iso.jpg",
                img_alt: "4층 SMT 기판 및 MJF PA12 하부 외함 실물",
            },
            ProofCardData {
                tag: "03 // 내부 안착 · 기구 결합",
                title: "부품 실장 배치 및 안착 구조 점검",
                desc: "리튬폴리머 배터리 안착 포켓, 보드 체결 보스, 스위치·포트 개구부의 간섭 없는 물리 결합 구조를 확인한 평면 배치입니다.",
                img_src: "/assets/showcase/real_knolling_top.jpg",
                img_alt: "기판 상면 부품 배치 및 외함 내부 리브 평면 사진",
            },
        ],
    )
}

/// Real hardware verification gallery section rendered at `#hardware-proof`.
#[component]
pub fn HardwareProofSection() -> Element {
    let lang = use_context::<Signal<Language>>();
    let (eyebrow, title, subtitle, cards) = match lang() {
        Language::En => proof_cards_en(),
        Language::Ko => proof_cards_ko(),
    };

    rsx! {
        section {
            id: "hardware-proof",
            class: "py-16 sm:py-24 bg-zinc-900 text-white border-b border-zinc-800 font-sans relative overflow-hidden",
            div { class: "absolute inset-0 bg-tech-grid opacity-10 pointer-events-none" }
            div {
                class: "max-w-[1400px] mx-auto px-4 sm:px-8 relative z-10 space-y-10",
                div {
                    class: "max-w-3xl space-y-3 reveal-on-scroll",
                    div {
                        class: "inline-flex items-center gap-2 px-3 py-1 rounded-full bg-zinc-800/90 border border-zinc-700 text-zinc-300 font-mono text-[11px] font-semibold tracking-wider uppercase",
                        span { class: "w-2 h-2 rounded-full bg-emerald-400 animate-pulse" }
                        "{eyebrow}"
                    }
                    h2 {
                        class: "text-2xl sm:text-4xl font-bold font-sans tracking-tight text-white leading-tight break-keep",
                        "{title}"
                    }
                    p {
                        class: "text-sm sm:text-base text-zinc-300 font-normal leading-relaxed break-keep",
                        "{subtitle}"
                    }
                }
                ProofBentoGrid { cards }
            }
        }
    }
}

/// Asymmetric bento grid preserving natural portrait and landscape studio camera framing.
#[component]
fn ProofBentoGrid(cards: [ProofCardData; 3]) -> Element {
    rsx! {
        div {
            class: "grid grid-cols-1 lg:grid-cols-12 gap-6 items-stretch reveal-on-scroll delay-100",
            div {
                class: "lg:col-span-5 bg-zinc-950 border border-zinc-800 rounded-xs overflow-hidden flex flex-col shadow-lg group",
                div {
                    class: "aspect-[4/5] overflow-hidden bg-zinc-900",
                    img {
                        src: "{cards[0].img_src}",
                        alt: "{cards[0].img_alt}",
                        class: "w-full h-full object-cover transition-transform duration-500 group-hover:scale-[1.02]",
                    }
                }
                ProofCardCaption { card: cards[0] }
            }

            div {
                class: "lg:col-span-7 flex flex-col gap-6",
                div {
                    class: "bg-zinc-950 border border-zinc-800 rounded-xs overflow-hidden flex flex-col shadow-lg group",
                    div {
                        class: "aspect-[16/10] overflow-hidden bg-zinc-900",
                        img {
                            src: "{cards[1].img_src}",
                            alt: "{cards[1].img_alt}",
                            class: "w-full h-full object-cover transition-transform duration-500 group-hover:scale-[1.02]",
                        }
                    }
                    ProofCardCaption { card: cards[1] }
                }

                div {
                    class: "flex-1 bg-zinc-950 border border-zinc-800 rounded-xs overflow-hidden grid grid-cols-1 sm:grid-cols-12 shadow-lg group",
                    div {
                        class: "sm:col-span-5 aspect-[3/4] sm:aspect-auto overflow-hidden bg-zinc-900",
                        img {
                            src: "{cards[2].img_src}",
                            alt: "{cards[2].img_alt}",
                            class: "w-full h-full object-cover transition-transform duration-500 group-hover:scale-[1.02]",
                        }
                    }
                    div {
                        class: "sm:col-span-7 flex flex-col justify-center",
                        ProofCardCaption { card: cards[2] }
                    }
                }
            }
        }
    }
}

/// Reusable text block rendering hardware verification tag, title, and engineering summary.
#[component]
fn ProofCardCaption(card: ProofCardData) -> Element {
    rsx! {
        div {
            class: "p-5 sm:p-6 space-y-1.5",
            span {
                class: "inline-block font-mono text-[10px] font-semibold text-[#E64A19] tracking-wider uppercase",
                "{card.tag}"
            }
            h3 {
                class: "text-base sm:text-lg font-bold text-white tracking-tight break-keep",
                "{card.title}"
            }
            p {
                class: "text-xs sm:text-sm text-zinc-400 leading-relaxed break-keep",
                "{card.desc}"
            }
        }
    }
}
