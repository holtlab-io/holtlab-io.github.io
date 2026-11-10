use dioxus::prelude::*;

#[component]
pub fn Footer() -> Element {
    rsx! {
        footer {
            class: "py-16 bg-[#070A0F] border-t border-white/[0.08] text-slate-400 text-xs font-mono",
            div {
                class: "max-w-7xl mx-auto px-4 sm:px-6 lg:px-8",
                div {
                    class: "flex flex-col md:flex-row items-center justify-between gap-8 pb-12 border-b border-white/[0.06]",
                    // Brand Info
                    div {
                        class: "flex items-center gap-3.5",
                        img {
                            src: "assets/branding/avatar.jpg",
                            alt: "Holt Lab Luwim Avatar",
                            class: "w-10 h-10 rounded-xl object-cover border border-white/20"
                        }
                        div {
                            div {
                                class: "text-white font-bold text-sm tracking-wide",
                                "HOLT LAB"
                            }
                            div {
                                class: "text-slate-400 text-[11px]",
                                "Boutique Hardware, MCAD, ECAD & Firmware Studio"
                            }
                        }
                    }

                    // Social & Channel Links
                    div {
                        class: "flex items-center gap-6 text-sm",
                        a {
                            href: "https://github.com/holtlab-io",
                            target: "_blank",
                            class: "hover:text-white transition-colors flex items-center gap-1.5",
                            span { "GitHub" }
                            span { class: "text-[10px] text-slate-500", "↗" }
                        }
                        a {
                            href: "https://youtube.com/@holt-lab",
                            target: "_blank",
                            class: "hover:text-white transition-colors flex items-center gap-1.5",
                            span { "YouTube" }
                            span { class: "text-[10px] text-slate-500", "↗" }
                        }
                        a {
                            href: "mailto:contact@holtlab.io",
                            class: "hover:text-white transition-colors flex items-center gap-1.5",
                            span { "contact@holtlab.io" }
                        }
                    }
                }

                // Bottom Copyright
                div {
                    class: "pt-8 flex flex-col sm:flex-row items-center justify-between gap-4 text-slate-500 text-[11px]",
                    div {
                        "© 2026 Holt Lab (대표: Wim). All rights reserved. Zero-Pingpong Turnkey Engineering."
                    }
                    div {
                        "Built with Rust & Dioxus 0.7 · Deployed on GitHub Pages"
                    }
                }
            }
        }
    }
}
