//! ## Overview
//! Contact and RFQ consultation module connecting clients directly with Holt Lab's lead engineer.
//! Provides quiet Swiss styling and direct email dispatch without sales marketing funnel copy.
//!
//! ## Search Tags
//! #contact, #rfq, #inquiry, #consultation, #sla
//!
//! ## Submodules
//! - [`info`] (`info.rs`): Contact info and institutional coordinates.
//! - [`form`] (`form.rs`): Quiet RFQ form and email dispatcher.
//! - [`selectors`] (`selectors.rs`): Engineering discipline and project profile selection buttons.

use dioxus::prelude::*;

pub mod info;
pub mod form;
pub mod selectors;

pub use info::ContactInfo;
pub use form::ContactForm;

/// Direct inquiry and project diagnostic section rendered at `#contact`.
#[component]
pub fn Contact() -> Element {
    let selected_scope = use_signal(|| 0);
    let selected_target = use_signal(|| 0);
    let name = use_signal(String::new);
    let email = use_signal(String::new);
    let phone = use_signal(String::new);
    let description = use_signal(String::new);
    let is_submitted = use_signal(|| false);

    rsx! {
        section {
            id: "contact",
            class: "py-16 sm:py-24 bg-[#FAFAFA] border-t border-zinc-200 font-sans",
            div {
                class: "max-w-[1600px] mx-auto px-4 sm:px-8",
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
