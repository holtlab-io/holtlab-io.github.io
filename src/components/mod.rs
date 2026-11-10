//! ## Overview
//! UI components module providing modular layout sections for the Holt Lab catalog application.
//! Encapsulates reusable design system elements conforming to the Agent-Native specification.
//!
//! ## Search Tags
//! #components, #ui, #catalog, #sections, #widgets
//!
//! ## Submodules
//! - [`navbar`] (`navbar/mod.rs`): Minimalist wireframe navigation bar and subcategory filter strip.
//! - [`catalog`] (`catalog/mod.rs`): Flagship product catalog grid and interactive hardware spec drawer.
//! - [`specs`] (`specs.rs`): High-density Swiss engineering specification matrix table.
//! - [`philosophy`] (`philosophy.rs`): Studio engineering protocol and turnkey hardware philosophy.
//! - [`contact`] (`contact/mod.rs`): Quiet studio inquiry dispatch form and contact coordinates.
//! - [`footer`] (`footer.rs`): Minimalist wireframe footer and studio credits.

pub mod navbar;
pub mod catalog;
pub mod specs;
pub mod philosophy;
pub mod contact;
pub mod footer;
