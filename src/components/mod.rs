//! ## Overview
//! UI components module providing modular layout sections for the landing page.
//! Encapsulates reusable design system elements conforming to the Agent-Native specification.
//!
//! ## Search Tags
//! #components, #ui, #sections, #widgets
//!
//! ## Submodules
//! - [`navbar`] (`navbar.rs`): Sticky navigation bar with telemetry and mobile drawer.
//! - [`hero`] (`hero.rs`): Primary studio banner, value proposition, and inspection bench.
//! - [`metrics`] (`metrics.rs`): Quantitative engineering accuracy metrics.
//! - [`comparison`] (`comparison.rs`): Direct comparison between split outsourcing and turnkey architecture.
//! - [`showcase`] (`showcase/mod.rs`): Deep-dive case study tabs for Win95 Minesweeper Touch Keyring.
//! - [`services`] (`services.rs`): 4 core turnkey hardware engineering capabilities.
//! - [`process`] (`process.rs`): 4-stage deterministic engineering verification gates.
//! - [`contact`] (`contact/mod.rs`): Interactive diagnostic RFQ form and SLA assurances.
//! - [`footer`] (`footer.rs`): Studio branding, copyright, and external channel links.

pub mod navbar;
pub mod hero;
pub mod metrics;
pub mod comparison;
pub mod showcase;
pub mod services;
pub mod process;
pub mod contact;
pub mod footer;
