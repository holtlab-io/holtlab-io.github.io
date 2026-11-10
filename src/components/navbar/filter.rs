//! ## Overview
//! Horizontal subcategory filter strip and mobile navigation drawer components.
//! Enables dynamic hardware discipline filtering and responsive anchor navigation.
//!
//! ## Search Tags
//! #filter, #drawer, #categories, #mobile, #navigation

use dioxus::prelude::*;

const CATEGORIES: [(&str, &str); 8] = [
    ("all", "all hardware"),
    ("keyrings", "keyrings"),
    ("pcba", "pcba"),
    ("enclosures", "enclosures"),
    ("firmware", "firmware"),
    ("specs", "specs"),
    ("about", "about"),
    ("contact", "contact"),
];

/// Horizontal category filter strip directly below the header.
#[component]
pub fn FilterStrip(mut active_filter: Signal<&'static str>) -> Element {
    rsx! {
        div {
            class: "bg-white px-4 sm:px-8 py-3 flex items-center justify-between text-xs sm:text-[13px] font-sans lowercase select-none overflow-x-auto",
            div {
                class: "flex items-center gap-6 sm:gap-7 whitespace-nowrap",
                for (cat_id, label) in CATEGORIES.iter() {
                    button {
                        r#type: "button",
                        onclick: move |_| {
                            active_filter.set(cat_id);
                        },
                        class: if active_filter() == *cat_id {
                            "text-zinc-950 font-medium transition-colors"
                        } else {
                            "text-zinc-400 hover:text-zinc-950 transition-colors font-normal"
                        },
                        if *cat_id == "specs" {
                            a { href: "#specs", class: "block", "{label}" }
                        } else if *cat_id == "about" {
                            a { href: "#about", class: "block", "{label}" }
                        } else if *cat_id == "contact" {
                            a { href: "#contact", class: "block", "{label}" }
                        } else {
                            span { "{label}" }
                        }
                    }
                }
            }
            span {
                class: "hidden md:inline-block text-xs font-sans text-zinc-950 whitespace-nowrap pl-4",
                "8 hardware units"
            }
        }
    }
}

/// Mobile dropdown menu drawer for smaller viewports.
#[component]
pub fn NavMobileDrawer(mut is_open: Signal<bool>, mut active_filter: Signal<&'static str>) -> Element {
    rsx! {
        div {
            class: "lg:hidden bg-white border-t border-zinc-200 px-6 py-6 space-y-4 font-sans text-sm lowercase",
            div {
                class: "grid grid-cols-2 gap-4 pb-4 border-b border-zinc-100",
                button {
                    r#type: "button",
                    onclick: move |_| {
                        active_filter.set("all");
                        is_open.set(false);
                    },
                    class: "text-left text-zinc-900 font-medium",
                    "all hardware"
                }
                button {
                    r#type: "button",
                    onclick: move |_| {
                        active_filter.set("keyrings");
                        is_open.set(false);
                    },
                    class: "text-left text-zinc-600 hover:text-zinc-950",
                    "keyrings"
                }
                button {
                    r#type: "button",
                    onclick: move |_| {
                        active_filter.set("pcba");
                        is_open.set(false);
                    },
                    class: "text-left text-zinc-600 hover:text-zinc-950",
                    "pcba modules"
                }
                button {
                    r#type: "button",
                    onclick: move |_| {
                        active_filter.set("enclosures");
                        is_open.set(false);
                    },
                    class: "text-left text-zinc-600 hover:text-zinc-950",
                    "mjf enclosures"
                }
                button {
                    r#type: "button",
                    onclick: move |_| {
                        active_filter.set("firmware");
                        is_open.set(false);
                    },
                    class: "text-left text-zinc-600 hover:text-zinc-950",
                    "firmware runtime"
                }
                a {
                    href: "#specs",
                    onclick: move |_| is_open.set(false),
                    class: "text-zinc-600 hover:text-zinc-950",
                    "specifications"
                }
                a {
                    href: "#about",
                    onclick: move |_| is_open.set(false),
                    class: "text-zinc-600 hover:text-zinc-950",
                    "studio protocol"
                }
                a {
                    href: "#contact",
                    onclick: move |_| is_open.set(false),
                    class: "text-zinc-600 hover:text-zinc-950 font-bold",
                    "inquire →"
                }
            }
            p {
                class: "text-[10px] font-mono text-zinc-400 leading-normal",
                "0.00 mm³ CAD collision tolerance · direct lead architect turnkey"
            }
        }
    }
}
