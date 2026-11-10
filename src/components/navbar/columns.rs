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
            class: "hidden lg:flex items-start gap-8 xl:gap-12",
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
            class: "flex flex-col space-y-1",
            div {
                class: "flex items-center gap-2",
                svg {
                    class: "w-5 h-5 text-zinc-900 stroke-current fill-none stroke-[1.5]",
                    view_box: "0 0 24 24",
                    circle { cx: "12", cy: "12", r: "2.5", fill: "currentColor" }
                    path { d: "M12 9V3a3 3 0 0 1 3 3v3h-3zM15 12h6a3 3 0 0 1-3 3h-3v-3zM12 15v6a3 3 0 0 1-3-3v-3h3zM9 12H3a3 3 0 0 1 3-3h3v3z", fill: "currentColor" }
                }
                span { class: "text-sm font-bold font-sans lowercase text-zinc-950", "hardware" }
            }
            div {
                class: "flex flex-col text-[11px] font-sans lowercase text-zinc-500 leading-tight space-y-0.5 pt-0.5",
                a { href: "#catalog", class: "hover:text-zinc-950 transition-colors", "instruments" }
                a { href: "#catalog", class: "hover:text-zinc-950 transition-colors", "sub-assemblies" }
                a { href: "#catalog", class: "hover:text-zinc-950 transition-colors", "keyrings" }
            }
        }
    }
}

/// Column 2: Full hardware catalog and B-Rep models.
#[component]
fn CatalogColumn() -> Element {
    rsx! {
        div {
            class: "flex flex-col space-y-1",
            div {
                class: "flex items-center gap-2",
                svg {
                    class: "w-5 h-5 text-zinc-900 stroke-current fill-none stroke-[1.5]",
                    view_box: "0 0 24 24",
                    rect { x: "5", y: "3", width: "14", height: "18", rx: "2" }
                    rect { x: "7", y: "5", width: "10", height: "7" }
                    circle { cx: "9", cy: "15", r: "1", fill: "currentColor" }
                    circle { cx: "15", cy: "15", r: "1", fill: "currentColor" }
                }
                span { class: "text-sm font-bold font-sans lowercase text-zinc-950", "catalog" }
            }
            div {
                class: "flex flex-col text-[11px] font-sans lowercase text-zinc-500 leading-tight space-y-0.5 pt-0.5",
                a { href: "#catalog", class: "hover:text-zinc-950 transition-colors", "all units" }
                a { href: "#catalog", class: "hover:text-zinc-950 transition-colors", "b-rep cad" }
                a { href: "#catalog", class: "hover:text-zinc-950 transition-colors", "firmware" }
            }
        }
    }
}

/// Column 3: Studio turnkey model and direct factory dispatch.
#[component]
fn StudioColumn() -> Element {
    rsx! {
        div {
            class: "flex flex-col space-y-1",
            div {
                class: "flex items-center gap-2",
                svg {
                    class: "w-5 h-5 text-zinc-900 stroke-current fill-none stroke-[1.5]",
                    view_box: "0 0 24 24",
                    rect { x: "3", y: "3", width: "18", height: "18" }
                    rect { x: "7", y: "7", width: "10", height: "10" }
                }
                span { class: "text-sm font-bold font-sans lowercase text-zinc-950", "studio" }
            }
            div {
                class: "flex flex-col text-[11px] font-sans lowercase text-zinc-500 leading-tight space-y-0.5 pt-0.5",
                a { href: "#about", class: "hover:text-zinc-950 transition-colors", "turnkey 1:1" }
                a { href: "#specs", class: "hover:text-zinc-950 transition-colors", "b-rep solid" }
                a { href: "#about", class: "hover:text-zinc-950 transition-colors", "jlcpcb direct" }
            }
        }
    }
}

/// Column 4: Engineering specifications and clock frequencies.
#[component]
fn SpecsColumn() -> Element {
    rsx! {
        div {
            class: "flex flex-col space-y-1",
            div {
                class: "flex items-center gap-2",
                svg {
                    class: "w-5 h-5 text-zinc-900 stroke-current fill-none stroke-[1.5]",
                    view_box: "0 0 24 24",
                    circle { cx: "12", cy: "12", r: "7" }
                    path { d: "M12 2v3M12 19v3M2 12h3M19 12h3" }
                    circle { cx: "12", cy: "12", r: "2", fill: "currentColor" }
                }
                span { class: "text-sm font-bold font-sans lowercase text-zinc-950", "specs" }
            }
            div {
                class: "flex flex-col text-[11px] font-sans lowercase text-zinc-500 leading-tight space-y-0.5 pt-0.5",
                a { href: "#specs", class: "hover:text-zinc-950 transition-colors", "0.00 mm³ collision" }
                a { href: "#specs", class: "hover:text-zinc-950 transition-colors", "esp32-c3 160mhz" }
                a { href: "#specs", class: "hover:text-zinc-950 transition-colors", "26.67mhz gdma" }
            }
        }
    }
}
