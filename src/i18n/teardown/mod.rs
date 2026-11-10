//! ## Overview
//! Localized data models and telemetry structures for the 4-layer engineering teardown.
//! Coordinates English and Korean data providers across MCAD, ECAD, firmware, and haptics.
//!
//! ## Search Tags
//! #i18n, #teardown, #layers, #subsystems, #telemetry
//!
//! ## Submodules
//! - [`en`] (`en.rs`): English teardown layer data definitions and section strings.
//! - [`ko`] (`ko.rs`): Korean teardown layer data definitions and section strings.

pub mod en;
pub mod ko;

use super::Language;

/// Localized string bundle for the teardown section headings and exploded banner.
#[derive(Clone, Copy)]
pub struct TeardownSectionStrings {
    /// Section eyebrow badge.
    pub eyebrow: &'static str,
    /// Main section title.
    pub title: &'static str,
    /// Section transparency subtitle.
    pub subtitle: &'static str,
    /// Exploded banner eyebrow.
    pub exploded_eyebrow: &'static str,
    /// Exploded banner headline.
    pub exploded_title: &'static str,
    /// Exploded callout tags (tag, label).
    pub callouts: [(&'static str, &'static str); 4],
    /// Exploded bottom telemetry strip items.
    pub bottom_strip: [&'static str; 3],
}

/// Localized subsystem layer record describing physical and electrical telemetry.
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct TeardownLayerData {
    /// Zero-indexed layer identifier.
    pub id: usize,
    /// Numerical display string for the layer sequence.
    pub number: &'static str,
    /// Human-readable subsystem title.
    pub title: &'static str,
    /// High-level engineering subtitle describing core technologies.
    pub subtitle: &'static str,
    /// Asset path to the corresponding visual or engineering drawing.
    pub image_src: &'static str,
    /// Descriptive alt text for accessibility.
    pub image_alt: &'static str,
    /// Deterministic gate verification status badge text.
    pub verification_badge: &'static str,
    /// Primary physical or electrical dimension metric.
    pub primary_spec: &'static str,
    /// Secondary tolerance or stackup clearance specification.
    pub secondary_spec: &'static str,
    /// Bus interface or industrial material composition.
    pub bus_or_material: &'static str,
    /// Deep paragraph detailing the engineering architecture of the layer.
    pub deep_summary: &'static str,
    /// First technical achievement bullet point.
    pub bullet_1: &'static str,
    /// Second technical achievement bullet point.
    pub bullet_2: &'static str,
    /// Third technical achievement bullet point.
    pub bullet_3: &'static str,
}

/// Returns the section-level headings and exploded banner strings.
pub fn get_teardown_section_strings(lang: Language) -> TeardownSectionStrings {
    match lang {
        Language::En => en::get_section_strings(),
        Language::Ko => ko::get_section_strings(),
    }
}

/// Returns the 4 teardown layer definitions for the selected language.
pub fn get_teardown_layers(lang: Language) -> [TeardownLayerData; 4] {
    match lang {
        Language::En => en::get_layers(),
        Language::Ko => ko::get_layers(),
    }
}
