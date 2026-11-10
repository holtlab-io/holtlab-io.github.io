//! ## Overview
//! Contact and RFQ consultation module connecting clients directly with Holt Lab's lead engineer.
//! Features institutional SLA guarantees, grant compliance notes, and an interactive diagnostic form.
//!
//! ## Search Tags
//! #contact, #rfq, #inquiry, #consultation, #sla
//!
//! ## Submodules
//! - [`info`] (`info.rs`): Contact info and institutional assurances.
//! - [`form`] (`form.rs`): Diagnostic RFQ form and email dispatcher.
//! - [`selectors`] (`selectors.rs`): Scope and target category selection buttons.

use dioxus::prelude::*;

pub mod info;
pub mod form;
pub mod selectors;

pub use info::ContactInfo;
pub use form::ContactForm;

/// Direct inquiry and project diagnostic section.
#[component]
pub fn Contact() -> Element {
    let selected_scope = use_signal(|| 0); // 0: Turnkey, 1: ECAD, 2: MCAD, 3: Firmware
    let selected_target = use_signal(|| 0); // 0: Grant, 1: Startup, 2: Enterprise, 3: Other
    let name = use_signal(String::new);
    let email = use_signal(String::new);
    let phone = use_signal(String::new);
    let description = use_signal(String::new);
    let is_submitted = use_signal(|| false);

    rsx! {
        section {
            id: "contact",
            class: "py-20 bg-[#FAFAFA] relative font-mono",
            div {
                class: "max-w-7xl mx-auto px-4 sm:px-6 lg:px-8",
                div {
                    class: "grid grid-cols-1 lg:grid-cols-12 gap-10 items-start",
                    ContactInfo {}
                    ContactForm {
                        selected_scope,
                        selected_target,
                        name,
                        email,
                        phone,
                        description,
                        is_submitted,
                    }
                }
            }
        }
    }
}
