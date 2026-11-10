//! ## Overview
//! UI components module providing modular layout sections for the Holt Lab hardware demonstrator showcase.
//! Encapsulates reusable design system elements conforming to the Agent-Native specification.
//!
//! ## Search Tags
//! #components, #ui, #demonstrator, #sections, #widgets, #capabilities
//!
//! ## Submodules
//! - [`capabilities`] (`capabilities.rs`): Studio engineering capabilities presenting 4 core disciplines.
//! - [`contact`] (`contact/mod.rs`): Direct studio inquiry dispatch form and contact coordinates.
//! - [`footer`] (`footer.rs`): Minimalist wireframe footer and studio credits.
//! - [`head`] (`head.rs`): Global document head metadata, styles, and script injection.
//! - [`hero`] (`hero.rs`): Monumental hero flagship hardware demonstrator section.
//! - [`navbar`] (`navbar/mod.rs`): Minimalist industrial navigation bar and mobile drawer.
//! - [`specs`] (`specs.rs`): High-density Swiss engineering specification matrix table.
//! - [`studio`] (`studio.rs`): Studio turnkey engineering model and co-design pipeline.
//! - [`teardown`] (`teardown/mod.rs`): Interactive exploded-view anatomical engineering teardown.
//! - [`teasers`] (`teasers.rs`): Featured demonstrator teasers and studio methodology preview cards.
//! - [`workbench`] (`workbench.rs`): Interactive 4-view CAD inspection workbench console.

pub mod capabilities;
pub mod contact;
pub mod footer;
pub mod head;
pub mod hero;
pub mod navbar;
pub mod specs;
pub mod studio;
pub mod teardown;
pub mod teasers;
pub mod workbench;

pub use head::HeadMetadata;
