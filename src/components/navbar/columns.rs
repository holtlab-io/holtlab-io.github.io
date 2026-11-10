//! ## Overview
//! Desktop 4-column wireframe navigation menu modeled after Teenage Engineering's store header.
//! Features line-art icons and lowercase links for hardware, catalog, studio, and engineering specs.
//!
//! ## Search Tags
//! #columns, #icons, #wireframe, #navigation, #sublinks

use dioxus::prelude::*;

/// Desktop 4-column wireframe icon navigation.
#[component]
pub fn NavColumns() -> Element {
    rsx! {
        div {
            class: "hidden lg:flex items-start gap-8 xl:gap-11 select-none",
            HardwareColumn {}
            CatalogColumn {}
            StudioColumn {}
            SpecsColumn {}
        }
    }
}

/// Column 1: Hardware instruments and sub-assemblies.
#[component]
fn HardwareColumn() -> Element {
    rsx! {
        div {
            class: "flex items-start gap-2.5",
            div {
                class: "flex flex-col items-center pt-0.5",
                svg {
                    class: "w-6 h-6 fill-zinc-950 shrink-0",
                    view_box: "0 0 24 24",
                    circle { cx: "12", cy: "12", r: "2.5" }
                    path { d: "M12 9.5V2a3 3 0 0 1 3 3v4.5h-3zM14.5 12H22a3 3 0 0 1-3 3h-4.5v-3zM12 14.5V22a3 3 0 0 1-3-3v-4.5h3zM9.5 12H2a3 3 0 0 1 3-3h4.5v3z" }
                }
                div { class: "w-[1px] h-9 bg-zinc-950 mt-1" }
            }
            div {
                class: "flex flex-col",
                span { class: "text-[17px] font-normal font-sans lowercase text-zinc-950 leading-tight", "hardware" }
                div {
                    class: "flex flex-col text-[11px] font-sans lowercase text-zinc-500 leading-[1.35] pt-1 space-y-0.5",
                    a { href: "#catalog", class: "hover:text-zinc-950 transition-colors", "instruments" }
                    a { href: "#catalog", class: "hover:text-zinc-950 transition-colors", "sub-assemblies" }
                    a { href: "#catalog", class: "hover:text-zinc-950 transition-colors", "keyrings" }
                }
            }
        }
    }
}

/// Column 2: Full hardware catalog and B-Rep models.
#[component]
fn CatalogColumn() -> Element {
    rsx! {
        div {
            class: "flex items-start gap-2.5",
            svg {
                class: "w-6 h-10 shrink-0 fill-zinc-950 mt-0.5",
                view_box: "0 0 24 40",
                path { d: "M2 2h20v36H2V2zm4 4v4h12V6H6z", fill_rule: "evenodd" }
            }
            div {
                class: "flex flex-col",
                span { class: "text-[17px] font-normal font-sans lowercase text-zinc-950 leading-tight", "catalog" }
                div {
                    class: "flex flex-col text-[11px] font-sans lowercase text-zinc-500 leading-[1.35] pt-1 space-y-0.5",
                    a { href: "#catalog", class: "hover:text-zinc-950 transition-colors", "all units" }
                    a { href: "#catalog", class: "hover:text-zinc-950 transition-colors", "b-rep cad" }
                    a { href: "#catalog", class: "hover:text-zinc-950 transition-colors", "firmware" }
                }
            }
        }
    }
}

/// Column 3: Studio turnkey model and direct factory dispatch.
#[component]
fn StudioColumn() -> Element {
    rsx! {
        div {
            class: "flex items-start gap-2.5",
            svg {
                class: "w-8 h-8 shrink-0 stroke-zinc-950 stroke-[3] fill-none mt-0.5",
                view_box: "0 0 24 24",
                rect { x: "2", y: "2", width: "20", height: "20" }
            }
            div {
                class: "flex flex-col",
                span { class: "text-[17px] font-normal font-sans lowercase text-zinc-950 leading-tight", "studio" }
                div {
                    class: "flex flex-col text-[11px] font-sans lowercase text-zinc-500 leading-[1.35] pt-1 space-y-0.5",
                    a { href: "#about", class: "hover:text-zinc-950 transition-colors", "turnkey 1:1" }
                    a { href: "#specs", class: "hover:text-zinc-950 transition-colors", "b-rep solid" }
                    a { href: "#about", class: "hover:text-zinc-950 transition-colors", "jlcpcb direct" }
                }
            }
        }
    }
}

/// Column 4: Engineering specifications and clock frequencies.
#[component]
fn SpecsColumn() -> Element {
    rsx! {
        div {
            class: "flex items-start gap-2.5",
            svg {
                class: "w-8 h-8 shrink-0 stroke-zinc-950 stroke-[1.5] fill-none mt-0.5",
                view_box: "0 0 32 32",
                path { d: "M4 4l8 8m0 0H6m6 0V6" }
                rect { x: "16", y: "4", width: "12", height: "12", class: "fill-zinc-950 stroke-none" }
                circle { cx: "8", cy: "24", r: "5" }
                path { d: "M12 28l4 4" }
                circle { cx: "22", cy: "22", r: "6" }
                circle { cx: "22", cy: "18", r: "1.5", class: "fill-zinc-950 stroke-none" }
                circle { cx: "25", cy: "21", r: "1.5", class: "fill-zinc-950 stroke-none" }
                circle { cx: "23", cy: "25", r: "1.5", class: "fill-zinc-950 stroke-none" }
                circle { cx: "19", cy: "24", r: "1.5", class: "fill-zinc-950 stroke-none" }
            }
            div {
                class: "flex flex-col",
                span { class: "text-[17px] font-normal font-sans lowercase text-zinc-950 leading-tight", "specs" }
                div {
                    class: "flex flex-col text-[11px] font-sans lowercase text-zinc-500 leading-[1.35] pt-1 space-y-0.5",
                    a { href: "#specs", class: "hover:text-zinc-950 transition-colors", "0.00 mm³ collision" }
                    a { href: "#specs", class: "hover:text-zinc-950 transition-colors", "esp32-c3 160mhz" }
                    a { href: "#specs", class: "hover:text-zinc-950 transition-colors", "26.67mhz gdma" }
                }
            }
        }
    }
}
