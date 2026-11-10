use dioxus::prelude::*;

#[component]
pub fn Navbar() -> Element {
    let mut is_mobile_open = use_signal(|| false);

    rsx! {
        header {
            class: "fixed top-0 left-0 right-0 z-50 bg-[#06080C]/90 backdrop-blur-md border-b border-white/[0.08] transition-all duration-200",
            div {
                class: "max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 h-16 sm:h-20 flex items-center justify-between",
                // Left Brand Logo & Title
                a {
                    href: "#",
                    class: "flex items-center gap-3 group cursor-pointer",
                    img {
                        src: "assets/branding/avatar.jpg",
                        alt: "Holt Lab Luwim Avatar",
                        class: "w-8 h-8 sm:w-9 sm:h-9 rounded-md object-cover border border-white/20 group-hover:border-[#FF5722] transition-colors"
                    }
                    div {
                        class: "flex flex-col",
                        div {
                            class: "flex items-center gap-2",
                            span {
                                class: "text-base sm:text-lg font-bold font-mono tracking-tight text-white group-hover:text-[#FF5722] transition-colors",
                                "HOLT LAB"
                            }
                            span {
                                class: "px-1.5 py-0.2 text-[9px] font-mono font-semibold uppercase tracking-wider rounded bg-white/[0.05] text-slate-400 border border-white/10",
                                "HW STUDIO"
                            }
                        }
                        span {
                            class: "text-[10px] font-mono text-zinc-500 tracking-wider uppercase hidden sm:inline",
                            "Silicon · Mechanical · Firmware"
                        }
                    }
                }

                // Center Technical Telemetry Tag (Hidden on mobile)
                div {
                    class: "hidden xl:flex items-center gap-2.5 px-3 py-1 rounded bg-white/[0.02] border border-white/[0.06] text-[11px] font-mono text-zinc-400",
                    span { class: "w-1.5 h-1.5 rounded-full bg-emerald-400 animate-pulse inline-block" }
                    span { class: "text-zinc-500", "CORE:" }
                    span { class: "text-zinc-300 font-semibold", "0.00 mm³ B-REP" }
                    span { class: "text-zinc-600", "//" }
                    span { class: "text-zinc-300 font-semibold", "4-LAYER PCBA" }
                    span { class: "text-zinc-600", "//" }
                    span { class: "text-[#FF5722] font-semibold", "26.67MHz GDMA" }
                }

                // Desktop Navigation Links
                nav {
                    class: "hidden md:flex items-center gap-6 lg:gap-8 text-xs font-mono font-medium text-zinc-400",
                    a {
                        href: "#why-turnkey",
                        class: "hover:text-white transition-colors cursor-pointer tracking-wider",
                        "01. ARCHITECTURE"
                    }
                    a {
                        href: "#showcase",
                        class: "hover:text-white transition-colors cursor-pointer tracking-wider",
                        "02. CASE STUDY"
                    }
                    a {
                        href: "#capabilities",
                        class: "hover:text-white transition-colors cursor-pointer tracking-wider",
                        "03. CAPABILITIES"
                    }
                    a {
                        href: "#process",
                        class: "hover:text-white transition-colors cursor-pointer tracking-wider",
                        "04. RIGOR"
                    }
                }

                // Right CTA Action
                div {
                    class: "hidden md:flex items-center gap-3",
                    a {
                        href: "#contact",
                        class: "inline-flex items-center gap-2 px-4 py-2 rounded-md bg-[#FF5722] hover:bg-[#F4511E] text-white text-xs font-mono font-bold tracking-wider uppercase transition-all shadow-md active:scale-[0.98]",
                        "INQUIRE // 의뢰하기"
                        span { class: "font-mono", "→" }
                    }
                }

                // Mobile Menu Button
                button {
                    r#type: "button",
                    aria_label: "Toggle Navigation Menu",
                    class: "md:hidden p-2 rounded border border-white/10 text-zinc-400 hover:text-white bg-white/[0.03] font-mono text-sm",
                    onclick: move |_| is_mobile_open.set(!is_mobile_open()),
                    if is_mobile_open() {
                        "✕"
                    } else {
                        "☰"
                    }
                }
            }

            // Mobile Dropdown Drawer
            if is_mobile_open() {
                div {
                    class: "md:hidden bg-[#0A0D13] border-b border-white/10 px-4 pt-3 pb-6 space-y-2 font-mono text-xs",
                    a {
                        href: "#why-turnkey",
                        onclick: move |_| is_mobile_open.set(false),
                        class: "block py-2.5 text-zinc-300 hover:text-white border-b border-white/[0.04]",
                        "01 // ARCHITECTURE (외주 핑퐁 종결)"
                    }
                    a {
                        href: "#showcase",
                        onclick: move |_| is_mobile_open.set(false),
                        class: "block py-2.5 text-zinc-300 hover:text-white border-b border-white/[0.04]",
                        "02 // CASE STUDY (Win95 실물 레퍼런스)"
                    }
                    a {
                        href: "#capabilities",
                        onclick: move |_| is_mobile_open.set(false),
                        class: "block py-2.5 text-zinc-300 hover:text-white border-b border-white/[0.04]",
                        "03 // CAPABILITIES (기구·회로·펌웨어)"
                    }
                    a {
                        href: "#process",
                        onclick: move |_| is_mobile_open.set(false),
                        class: "block py-2.5 text-zinc-300 hover:text-white border-b border-white/[0.04]",
                        "04 // RIGOR (4단계 검증 게이트)"
                    }
                    div {
                        class: "pt-3",
                        a {
                            href: "#contact",
                            onclick: move |_| is_mobile_open.set(false),
                            class: "block w-full text-center py-3 rounded-md bg-[#FF5722] text-white font-mono font-bold tracking-wider uppercase",
                            "INQUIRE // 프로젝트 의뢰하기 →"
                        }
                    }
                }
            }
        }
    }
}
