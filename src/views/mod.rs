//! ## Overview
//! Route views module for the Holt Lab web application.
//! Exports top-level page views rendered by the application router.
//!
//! ## Search Tags
//! #views, #routes, #pages, #ui, #demonstrator, #studio, #contact
//!
//! ## Submodules
//! - [`contact`] (`contact.rs`): Dedicated client RFQ intake and engineering consultation view.
//! - [`demonstrator`] (`demonstrator.rs`): Flagship hardware demonstrator and teardown view for Opus 01.
//! - [`home`] (`home.rs`): Primary studio overview landing page view.
//! - [`layout`] (`layout.rs`): Studio application layout framing views with navbar and footer.
//! - [`not_found`] (`not_found.rs`): Minimalist 404 error fallback view.
//! - [`studio`] (`studio.rs`): Studio turnkey engineering architecture and methodology view.

pub mod contact;
pub mod demonstrator;
pub mod home;
pub mod layout;
pub mod not_found;
pub mod studio;

pub use contact::ContactPage;
pub use demonstrator::Demonstrator;
pub use home::Home;
pub use layout::StudioLayout;
pub use not_found::NotFound;
pub use studio::StudioPage;
