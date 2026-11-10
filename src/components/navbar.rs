//! ## Overview
//! Sticky navigation bar providing brand identity, quick anchors, telemetry status, and mobile drawer.
//! Employs an off-white backdrop with deep charcoal typography and a restrained vermilion CTA button.
//!
//! ## Search Tags
//! #navbar, #header, #navigation, #telemetry, #menu

use dioxus::prelude::*;

/// Primary navigation bar component rendered at the top of the viewport.
#[component]
pub fn Navbar() -> Element {
    let mut is_mobile_open = use_signal(|| false);

    rsx! {
        header {
            class: "sticky top-0 left-0 right-0 z-50 bg-[#FAFAFA]/90 backdrop-blur-md border-b border-zinc-200 transition-all duration-200",
            div {
                class: "max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 h-16 sm:h-20 flex items-center justify-between",
                NavBrand {}
                NavTelemetry {}
                NavLinks {}

                // Right CTA & Mobile Toggle
                div {
                    class: "flex items-center gap-3",
                    a {
                        href: "#contact",
                        class: "hidden md:inline-flex items-center gap-2 px-4 py-2 rounded-md bg-[#E64A19] hover:bg-[#D84315] text-white text-xs font-mono font-bold tracking-wider uppercase transition-colors shadow-xs tactile-button",
                        "INQUIRE // 의뢰하기"
                        span { class: "font-mono", "→" }
                    }
                    button {
                        r#type: "button",
                        aria_label: "Toggle Navigation Menu",
                        class: "md:hidden p-2 rounded-md border border-zinc-300 text-zinc-700 hover:text-zinc-950 bg-white font-mono text-sm shadow-2xs",
                        onclick: move |_| is_mobile_open.set(!is_mobile_open()),
                        if is_mobile_open() { "✕" } else { "☰" }
                    }
                }
            }

            if is_mobile_open() {
                NavMobileDrawer { is_open: is_mobile_open }
            }
        }
    }
}

/// Brand logo and title lockup.
#[component]
fn NavBrand() -> Element {
    rsx! {
        a {
            href: "#",
            class: "flex items-center gap-3 group cursor-pointer",
            img {
                src: "assets/branding/avatar.jpg",
                alt: "Holt Lab Luwim Avatar",
                class: "w-8 h-8 sm:w-9 sm:h-9 rounded-md object-cover border border-zinc-200 group-hover:border-zinc-400 transition-colors"
            }
            div {
                class: "flex flex-col",
                div {
                    class: "flex items-center gap-2",
                    span {
                        class: "text-base sm:text-lg font-bold font-mono tracking-tight text-zinc-900 group-hover:text-zinc-950 transition-colors",
                        "HOLT LAB"
                    }
                    span {
                        class: "px-1.5 py-0.5 text-[9px] font-mono font-semibold uppercase tracking-wider rounded bg-zinc-100 text-zinc-600 border border-zinc-200",
                        "HW STUDIO"
                    }
                }
                span {
                    class: "text-[10px] font-mono text-zinc-500 tracking-wider uppercase hidden sm:inline",
                    "Silicon · Mechanical · Firmware"
                }
            }
        }
    }
}

/// Center telemetry status pill for desktop viewports.
#[component]
fn NavTelemetry() -> Element {
    rsx! {
        div {
            class: "hidden xl:flex items-center gap-2.5 px-3 py-1 rounded-full bg-zinc-100 border border-zinc-200 text-[11px] font-mono text-zinc-600",
            span { class: "w-1.5 h-1.5 rounded-full bg-zinc-500 inline-block" }
            span { class: "text-zinc-400", "CORE:" }
            span { class: "text-zinc-800 font-semibold", "0.00 mm³ B-REP" }
            span { class: "text-zinc-300", "//" }
            span { class: "text-zinc-800 font-semibold", "4-LAYER PCBA" }
            span { class: "text-zinc-300", "//" }
            span { class: "text-zinc-800 font-semibold", "26.67MHz GDMA" }
        }
    }
}

/// Desktop navigation link set.
#[component]
fn NavLinks() -> Element {
    rsx! {
        nav {
            class: "hidden md:flex items-center gap-6 lg:gap-8 text-xs font-mono font-medium text-zinc-600",
            a {
                href: "#why-turnkey",
                class: "hover:text-zinc-950 transition-colors cursor-pointer tracking-wider",
                "01. ARCHITECTURE"
            }
            a {
                href: "#showcase",
                class: "hover:text-zinc-950 transition-colors cursor-pointer tracking-wider",
                "02. CASE STUDY"
            }
            a {
                href: "#capabilities",
                class: "hover:text-zinc-950 transition-colors cursor-pointer tracking-wider",
                "03. CAPABILITIES"
            }
            a {
                href: "#process",
                class: "hover:text-zinc-950 transition-colors cursor-pointer tracking-wider",
                "04. RIGOR"
            }
        }
    }
}

/// Mobile dropdown menu drawer.
#[component]
fn NavMobileDrawer(mut is_open: Signal<bool>) -> Element {
    rsx! {
        div {
            class: "md:hidden bg-[#FAFAFA] border-b border-zinc-200 px-4 pt-3 pb-6 space-y-2 font-mono text-xs",
            a {
                href: "#why-turnkey",
                onclick: move |_| is_open.set(false),
                class: "block py-2.5 text-zinc-700 hover:text-zinc-950 border-b border-zinc-200/60",
                "01 // ARCHITECTURE (외주 핑퐁 종결)"
            }
            a {
                href: "#showcase",
                onclick: move |_| is_open.set(false),
                class: "block py-2.5 text-zinc-700 hover:text-zinc-950 border-b border-zinc-200/60",
                "02 // CASE STUDY (Win95 실물 레퍼런스)"
            }
            a {
                href: "#capabilities",
                onclick: move |_| is_open.set(false),
                class: "block py-2.5 text-zinc-700 hover:text-zinc-950 border-b border-zinc-200/60",
                "03 // CAPABILITIES (4대 하드웨어 스펙)"
            }
            a {
                href: "#process",
                onclick: move |_| is_open.set(false),
                class: "block py-2.5 text-zinc-700 hover:text-zinc-950 border-b border-zinc-200/60",
                "04 // RIGOR (4단계 검증 프로토콜)"
            }
            div {
                class: "pt-3",
                a {
                    href: "#contact",
                    onclick: move |_| is_open.set(false),
                    class: "block w-full py-3 text-center rounded-md bg-[#E64A19] hover:bg-[#D84315] text-white font-bold tracking-wider uppercase shadow-xs transition-colors",
                    "30분 기술 타당성 진단 신청 →"
                }
            }
        }
    }
}
