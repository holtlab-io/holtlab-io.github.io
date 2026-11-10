use dioxus::prelude::*;

#[component]
pub fn Navbar() -> Element {
    let mut is_mobile_open = use_signal(|| false);

    rsx! {
        header {
            class: "fixed top-0 left-0 right-0 z-50 bg-[#0B0F17]/80 backdrop-blur-md border-b border-white/[0.08] transition-all duration-300",
            div {
                class: "max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 h-20 flex items-center justify-between",
                // Left Brand Logo & Title
                a {
                    href: "#",
                    class: "flex items-center gap-3.5 group cursor-pointer",
                    img {
                        src: "assets/branding/avatar.jpg",
                        alt: "Holt Lab Luwim Avatar",
                        class: "w-10 h-10 rounded-xl object-cover border border-white/20 shadow-md group-hover:border-orange-500 transition-colors"
                    }
                    div {
                        class: "flex flex-col",
                        div {
                            class: "flex items-center gap-2",
                            span {
                                class: "text-lg font-bold tracking-tight text-white group-hover:text-orange-400 transition-colors",
                                "HOLT LAB"
                            }
                            span {
                                class: "px-1.5 py-0.5 text-[10px] font-mono font-semibold uppercase tracking-wider rounded bg-orange-500/10 text-orange-400 border border-orange-500/30",
                                "STUDIO"
                            }
                        }
                        span {
                            class: "text-[11px] font-mono text-slate-400 tracking-wide",
                            "From Silicon to Screen"
                        }
                    }
                }

                // Desktop Navigation Links
                nav {
                    class: "hidden md:flex items-center gap-8 text-sm font-medium text-slate-300",
                    a {
                        href: "#showcase",
                        class: "hover:text-white transition-colors cursor-pointer",
                        "Showcase"
                    }
                    a {
                        href: "#why-turnkey",
                        class: "hover:text-white transition-colors cursor-pointer",
                        "Why Turnkey"
                    }
                    a {
                        href: "#capabilities",
                        class: "hover:text-white transition-colors cursor-pointer",
                        "Capabilities"
                    }
                    a {
                        href: "#process",
                        class: "hover:text-white transition-colors cursor-pointer",
                        "Engineering Rigor"
                    }
                }

                // Right CTA Action
                div {
                    class: "hidden md:flex items-center gap-4",
                    a {
                        href: "#contact",
                        class: "inline-flex items-center gap-2 px-5 py-2.5 rounded-lg bg-orange-500 hover:bg-orange-600 text-white text-sm font-semibold shadow-lg shadow-orange-500/20 hover:shadow-orange-500/30 transition-all active:scale-[0.98]",
                        "프로젝트 의뢰하기"
                        span { class: "font-mono", "→" }
                    }
                }

                // Mobile Menu Button
                button {
                    r#type: "button",
                    class: "md:hidden p-2 rounded-lg text-slate-400 hover:text-white hover:bg-white/5",
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
                    class: "md:hidden bg-[#0F141E] border-b border-white/10 px-4 pt-3 pb-6 space-y-3",
                    a {
                        href: "#showcase",
                        onclick: move |_| is_mobile_open.set(false),
                        class: "block py-2 text-slate-300 hover:text-white text-base font-medium",
                        "Showcase (실물 레퍼런스)"
                    }
                    a {
                        href: "#why-turnkey",
                        onclick: move |_| is_mobile_open.set(false),
                        class: "block py-2 text-slate-300 hover:text-white text-base font-medium",
                        "Why Turnkey (외주 핑퐁 종결)"
                    }
                    a {
                        href: "#capabilities",
                        onclick: move |_| is_mobile_open.set(false),
                        class: "block py-2 text-slate-300 hover:text-white text-base font-medium",
                        "Capabilities (설계 분야)"
                    }
                    a {
                        href: "#process",
                        onclick: move |_| is_mobile_open.set(false),
                        class: "block py-2 text-slate-300 hover:text-white text-base font-medium",
                        "Engineering Rigor (4단계 검증)"
                    }
                    div {
                        class: "pt-2",
                        a {
                            href: "#contact",
                            onclick: move |_| is_mobile_open.set(false),
                            class: "block w-full text-center py-3 rounded-lg bg-orange-500 text-white font-semibold shadow-md",
                            "프로젝트 의뢰하기 →"
                        }
                    }
                }
            }
        }
    }
}
