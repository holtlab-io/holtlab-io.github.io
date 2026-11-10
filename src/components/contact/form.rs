//! ## Overview
//! Interactive diagnostic RFQ form allowing prospective clients to submit hardware scope and specifications.
//! Employs clean Swiss typography with quiet inputs and a direct mailto dispatcher.
//!
//! ## Search Tags
//! #contact, #form, #rfq, #inquiry, #interactive

use dioxus::prelude::*;
use super::selectors::{ScopeButtons, TargetButtons};

/// Interactive RFQ diagnostic form component with scope selector and email dispatcher.
#[component]
pub fn ContactForm(
    selected_scope: Signal<usize>,
    selected_target: Signal<usize>,
    name: Signal<String>,
    email: Signal<String>,
    phone: Signal<String>,
    description: Signal<String>,
    is_submitted: Signal<bool>,
) -> Element {
    rsx! {
        div {
            class: "lg:col-span-7 rounded-xs border border-zinc-200 bg-white p-5 sm:p-8 shadow-2xs relative font-sans",
            if is_submitted() {
                SubmittedView {
                    name,
                    phone,
                    email,
                    selected_scope,
                    description,
                }
            } else {
                FormFields {
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

/// Confirmation message and email mailto dispatch trigger.
#[component]
fn SubmittedView(
    name: Signal<String>,
    phone: Signal<String>,
    email: Signal<String>,
    selected_scope: Signal<usize>,
    description: Signal<String>,
) -> Element {
    let scope_text = match selected_scope() {
        0 => "turnkey (mcad+ecad+firmware)",
        1 => "4-layer pcba (kicad ecad)",
        2 => "3d enclosure (build123d mcad)",
        _ => "bare-metal firmware (rust/c)",
    };

    rsx! {
        div {
            class: "p-8 border border-zinc-200 bg-zinc-50 text-center space-y-4 rounded-xs font-mono",
            div {
                class: "inline-flex items-center gap-1.5 px-3 py-1 rounded-full bg-zinc-200 text-zinc-900 text-xs font-bold uppercase",
                span { class: "w-1.5 h-1.5 rounded-full bg-zinc-950" }
                "DISPATCH BUFFER READY"
            }
            h3 { class: "text-lg font-bold text-zinc-950 lowercase", "inquiry ready for dispatch" }
            p {
                class: "text-zinc-600 text-xs font-sans leading-relaxed max-w-md mx-auto",
                "your project specifications have been formatted. click below to open your mail client and dispatch directly to holt lab engineering."
            }
            a {
                href: format!(
                    "mailto:contact@holtlab.io?subject=[Holt Lab Commission] {}&body=organization: {}%0D%0Acontact: {}%0D%0Aemail: {}%0D%0Adiscipline: {}%0D%0Aspecifications:%0D%0A{}",
                    name(), name(), phone(), email(), scope_text, description()
                ),
                class: "inline-flex items-center justify-center px-6 py-3 bg-zinc-950 hover:bg-zinc-800 text-white font-bold text-xs font-mono tracking-wider uppercase transition-colors shadow-xs",
                "open mail client →"
            }
        }
    }
}

/// Form input fields for active RFQ creation.
#[component]
fn FormFields(
    selected_scope: Signal<usize>,
    selected_target: Signal<usize>,
    name: Signal<String>,
    email: Signal<String>,
    phone: Signal<String>,
    description: Signal<String>,
    mut is_submitted: Signal<bool>,
) -> Element {
    rsx! {
        form {
            class: "space-y-5",
            onsubmit: move |e| {
                e.prevent_default();
                if !email().is_empty() {
                    is_submitted.set(true);
                }
            },
            ScopeButtons { selected: selected_scope }
            TargetButtons { selected: selected_target }
            ContactInputs { name, email }
            ScopeTimelineInputs { phone, description }
            SubmitRow {}
        }
    }
}

/// Inputs for name and contact email.
#[component]
fn ContactInputs(mut name: Signal<String>, mut email: Signal<String>) -> Element {
    rsx! {
        div {
            class: "grid grid-cols-1 sm:grid-cols-2 gap-4 text-xs font-mono lowercase",
            div {
                class: "space-y-1.5",
                label { class: "block text-zinc-500", "name / organization" }
                input {
                    r#type: "text",
                    required: true,
                    placeholder: "e.g. acme robotics / jane doe",
                    value: "{name}",
                    oninput: move |e| name.set(e.value()),
                    class: "w-full p-3 rounded-xs border border-zinc-200 bg-white focus:border-zinc-950 focus:outline-none text-zinc-900 transition-colors"
                }
            }
            div {
                class: "space-y-1.5",
                label { class: "block text-zinc-500", "contact email" }
                input {
                    r#type: "email",
                    required: true,
                    placeholder: "name@domain.com",
                    value: "{email}",
                    oninput: move |e| email.set(e.value()),
                    class: "w-full p-3 rounded-xs border border-zinc-200 bg-white focus:border-zinc-950 focus:outline-none text-zinc-900 transition-colors"
                }
            }
        }
    }
}

/// Inputs for timeline and technical specifications.
#[component]
fn ScopeTimelineInputs(mut phone: Signal<String>, mut description: Signal<String>) -> Element {
    rsx! {
        div {
            class: "space-y-4 text-xs font-mono lowercase",
            div {
                class: "space-y-1.5",
                label { class: "block text-zinc-500", "target timeline / constraints" }
                input {
                    r#type: "text",
                    placeholder: "e.g. 4-6 weeks to working prototype / budget envelope",
                    value: "{phone}",
                    oninput: move |e| phone.set(e.value()),
                    class: "w-full p-3 rounded-xs border border-zinc-200 bg-white focus:border-zinc-950 focus:outline-none text-zinc-900 transition-colors"
                }
            }
            div {
                class: "space-y-1.5",
                label { class: "block text-zinc-500", "technical specifications & scope" }
                textarea {
                    rows: 4,
                    required: true,
                    placeholder: "describe dimensions, silicon choice, battery, interfaces, and deliverables...",
                    value: "{description}",
                    oninput: move |e| description.set(e.value()),
                    class: "w-full p-3 rounded-xs border border-zinc-200 bg-white focus:border-zinc-950 focus:outline-none text-zinc-900 transition-colors font-sans text-xs leading-relaxed"
                }
            }
        }
    }
}

/// Submit button and direct dispatch indicator.
#[component]
fn SubmitRow() -> Element {
    rsx! {
        div {
            class: "pt-2 flex items-center justify-between",
            button {
                r#type: "submit",
                class: "inline-flex items-center gap-2 px-6 py-3 rounded-xs bg-zinc-950 hover:bg-zinc-800 text-white font-mono text-xs font-bold tracking-wider uppercase transition-colors shadow-xs",
                "dispatch inquiry"
                span { "→" }
            }
            span { class: "text-[11px] font-mono text-zinc-400 lowercase", "direct architect dispatch" }
        }
    }
}
