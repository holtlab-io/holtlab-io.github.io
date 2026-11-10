//! ## Overview
//! Route definitions and navigation enum for Holt Lab's multi-page web platform.
//! Implements Dioxus Routable with clean URLs matching Framework and Linear.
//!
//! ## Search Tags
//! #routes, #router, #navigation, #routable, #pages
//!
//! ## Submodules
//! None. This module provides atomic route tree declarations.

use dioxus::prelude::*;
use crate::views::{ContactPage, Demonstrator, Home, NotFound, StudioLayout, StudioPage};

/// Top-level application routes for the Holt Lab studio platform.
#[derive(Clone, Debug, PartialEq, Routable)]
pub enum Route {
    /// Studio base layout wrapping main views with sticky navbar and footer.
    #[layout(StudioLayout)]
        /// Page 1: Studio Overview Home view (`/`).
        #[route("/")]
        Home {},
        /// Page 2: Flagship hardware demonstrator and teardown view (`/demonstrator`).
        #[route("/demonstrator")]
        Demonstrator {},
        /// Page 3: Studio turnkey architecture and methodology view (`/studio`).
        #[route("/studio")]
        StudioPage {},
        /// Page 4: Dedicated client RFQ intake view (`/contact`).
        #[route("/contact")]
        ContactPage {},
    #[end_layout]
    /// Catch-all fallback route for unknown paths.
    #[route("/:.._segments")]
    NotFound {
        /// Matched path segments for the not found route.
        _segments: Vec<String>,
    },
}
