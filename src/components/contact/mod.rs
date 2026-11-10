//! ## Overview
//! Contact and RFQ consultation module connecting clients directly with Holt Lab's lead engineer.
//! Provides quiet Swiss styling and direct email dispatch without sales marketing funnel copy.
//!
//! ## Search Tags
//! #contact, #rfq, #inquiry, #consultation, #dispatch
//!
//! ## Submodules
//! - [`form`] (`form.rs`): Quiet RFQ form and email dispatcher.
//! - [`info`] (`info.rs`): Contact info and institutional coordinates.
//! - [`selectors`] (`selectors.rs`): Engineering discipline and project profile selection buttons.

use dioxus::prelude::*;

pub mod form;
pub mod info;
pub mod selectors;

pub use form::ContactForm;
pub use info::ContactInfo;

/// Direct inquiry and project diagnostic section rendered at `#inquire`.
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
            id: "inquire",
            class: "py-16 sm:py-24 bg-[#FAFAFA] border-t border-zinc-200 font-sans",
            div {
                class: "max-w-[1500px] mx-auto px-4 sm:px-8",
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
