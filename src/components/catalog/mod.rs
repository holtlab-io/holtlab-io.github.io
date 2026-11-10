//! ## Overview
//! Flagship product catalog grid module displaying physical computing hardware reference units.
//! Emulates the Teenage Engineering product laboratory layout with neutral gray cards and quiet typography.
//!
//! ## Search Tags
//! #catalog, #hardware, #grid, #inspector, #teenage-engineering
//!
//! ## Submodules
//! - [`data`] (`data.rs`): Hardware catalog data definitions and technical specifications catalog.
//! - [`card`] (`card.rs`): Single hardware catalog card component.
//! - [`modal`] (`modal.rs`): Swiss engineering specification modal overlay.

use dioxus::prelude::*;

pub mod data;
pub mod card;
pub mod modal;

pub use data::{HardwareUnit, HARDWARE_UNITS};
pub use card::CatalogCard;
pub use modal::HardwareModal;

/// Primary catalog grid component displaying hardware units and managing detail modal inspection.
#[component]
pub fn Catalog(active_filter: Signal<&'static str>) -> Element {
    let mut selected_unit = use_signal(|| None::<HardwareUnit>);

    let current_filter = active_filter();
    let filtered_units: Vec<&HardwareUnit> = HARDWARE_UNITS
        .iter()
        .filter(|u| current_filter == "all" || u.category == current_filter)
        .collect();

    rsx! {
        section {
            id: "catalog",
            class: "bg-white pt-2 pb-16 sm:pb-24",
            div {
                class: "max-w-[1600px] mx-auto px-4 sm:px-8",

                if current_filter == "all" && filtered_units.len() >= 2 {
                    div {
                        class: "grid grid-cols-1 md:grid-cols-2 gap-6 lg:gap-8 mb-12 sm:mb-16",
                        CatalogCard { unit: filtered_units[0].clone(), on_select: move |u| selected_unit.set(Some(u)) }
                        CatalogCard { unit: filtered_units[1].clone(), on_select: move |u| selected_unit.set(Some(u)) }
                    }
                    div {
                        class: "grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-6 lg:gap-8 mb-12 sm:mb-16",
                        for unit in filtered_units.iter().skip(2).take(3) {
                            CatalogCard { unit: (*unit).clone(), on_select: move |u| selected_unit.set(Some(u)) }
                        }
                    }
                    div {
                        class: "grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-6 lg:gap-8",
                        for unit in filtered_units.iter().skip(5) {
                            CatalogCard { unit: (*unit).clone(), on_select: move |u| selected_unit.set(Some(u)) }
                        }
                    }
                } else {
                    div {
                        class: "grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-6 lg:gap-8",
                        for unit in filtered_units.iter() {
                            CatalogCard { unit: (*unit).clone(), on_select: move |u| selected_unit.set(Some(u)) }
                        }
                    }
                }
            }

            if let Some(unit) = selected_unit() {
                HardwareModal {
                    unit,
                    on_close: move |_| selected_unit.set(None),
                }
            }
        }
    }
}
