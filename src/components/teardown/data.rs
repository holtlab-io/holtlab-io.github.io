//! ## Overview
//! Re-exports and data adapters for the 4 anatomical teardown layers.
//! Integrates localized subsystem models conforming to the Agent-Native specification.
//!
//! ## Search Tags
//! #teardown, #layers, #subsystems, #specs, #data
//!
//! ## Submodules
//! None. This module provides pure structured data models.

pub use crate::i18n::teardown::{
    get_teardown_layers, get_teardown_section_strings, TeardownLayerData as TeardownLayer,
    TeardownSectionStrings,
};
