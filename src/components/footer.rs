use dioxus::prelude::*;

#[component]
pub fn Footer() -> Element {
    rsx! {
        footer {
            class: "py-12 bg-[#05070B] border-t border-white/[0.08] text-zinc-500 text-xs font-mono",
            div {
                class: "max-w-7xl mx-auto px-4 sm:px-6 lg:px-8",
                div {
                    class: "flex flex-col md:flex-row items-center justify-between gap-6 pb-8 border-b border-white/[0.06]",
                    // Brand Info
                    div {
                        class: "flex items-center gap-3",
                        img {
                            src: "assets/branding/avatar.jpg",
                            alt: "Holt Lab Luwim Avatar",
                            class: "w-8 h-8 rounded object-cover border border-white/20"
                        }
                        div {
                            div {
                                class: "text-white font-bold text-xs tracking-wider",
                                "HOLT LAB // BOUTIQUE HARDWARE STUDIO"
                            }
                            div {
                                class: "text-zinc-500 text-[10px]",
                                "Mechanical (MCAD) · Electronics (ECAD) · Bare-Metal Firmware"
                            }
                        }
                    }

                    // Social & Channel Links
                    div {
                        class: "flex items-center gap-6 text-xs",
                        a {
                            href: "https://github.com/holtlab-io",
                            target: "_blank",
                            class: "hover:text-white transition-colors flex items-center gap-1",
                            span { "GITHUB" }
                            span { class: "text-[10px] text-zinc-600", "↗" }
                        }
                        a {
                            href: "https://youtube.com/@holt-lab",
                            target: "_blank",
                            class: "hover:text-white transition-colors flex items-center gap-1",
                            span { "YOUTUBE" }
                            span { class: "text-[10px] text-zinc-600", "↗" }
                        }
                        a {
                            href: "mailto:contact@holtlab.io",
                            class: "hover:text-white transition-colors flex items-center gap-1",
                            span { "contact@holtlab.io" }
                        }
                    }
                }

                // Bottom Copyright
                div {
                    class: "pt-6 flex flex-col sm:flex-row items-center justify-between gap-3 text-zinc-500 text-[10px]",
                    div {
                        "© 2026 HOLT LAB. ALL RIGHTS RESERVED. ZERO-PINGPONG HARDWARE ARCHITECTURE."
                    }
                    div {
                        "BUILT WITH RUST & DIOXUS 0.7 (WASM) // GITHUB PAGES"
                    }
                }
            }
        }
    }
}
