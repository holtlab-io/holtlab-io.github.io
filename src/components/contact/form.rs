//! ## Overview
//! Interactive diagnostic RFQ form allowing prospective clients to submit hardware scope and specifications.
//! Employs clean Swiss typography with streamlined inputs and direct mail client dispatching.
//!
//! ## Search Tags
//! #contact, #form, #rfq, #inquiry, #interactive
//!
//! ## Submodules
//! None. This module provides atomic RFQ intake form components.

use dioxus::prelude::*;
use super::selectors::ScopeButtons;
use super::submitted::SubmittedView;
use crate::i18n::{contact::get_contact_strings, ContactFormStrings, Language};

/// Interactive RFQ diagnostic form component with scope selector and email dispatcher.
#[component]
pub fn ContactForm(
    selected_scope: Signal<usize>,
    name: Signal<String>,
    email: Signal<String>,
    timeline: Signal<String>,
    budget: Signal<String>,
    project_brief: Signal<String>,
    is_submitted: Signal<bool>,
) -> Element {
    let lang = use_context::<Signal<Language>>();
    let form_strings = get_contact_strings(lang()).form;

    rsx! {
        div {
            class: "lg:col-span-7 rounded-xs border border-zinc-200 bg-white p-5 sm:p-8 shadow-2xs relative font-sans",
            if is_submitted() {
                SubmittedView {
                    name,
                    email,
                    timeline,
                    budget,
                    project_brief,
                    selected_scope,
                    is_submitted,
                }
            } else {
                FormFields {
                    selected_scope,
                    name,
                    email,
                    timeline,
                    budget,
                    project_brief,
                    is_submitted,
                    form_strings,
                }
            }
        }
    }
}

/// Form input fields for active RFQ creation.
#[component]
fn FormFields(
    selected_scope: Signal<usize>,
    name: Signal<String>,
    email: Signal<String>,
    timeline: Signal<String>,
    budget: Signal<String>,
    project_brief: Signal<String>,
    mut is_submitted: Signal<bool>,
    form_strings: ContactFormStrings,
) -> Element {
    rsx! {
        form {
            class: "space-y-4 sm:space-y-5",
            onsubmit: move |e| {
                e.prevent_default();
                if !email().is_empty() {
                    is_submitted.set(true);
                }
            },
            ScopeButtons { selected: selected_scope }
            ContactInputs { name, email, form_strings }
            BriefInput { project_brief, form_strings }
            IntakeParametersInputs { timeline, budget, form_strings }
            SubmitRow { form_strings }
        }
    }
}

/// Inputs for client name or organization and contact email.
#[component]
fn ContactInputs(
    mut name: Signal<String>,
    mut email: Signal<String>,
    form_strings: ContactFormStrings,
) -> Element {
    rsx! {
        div {
            class: "grid grid-cols-1 sm:grid-cols-2 gap-3.5 sm:gap-4 text-xs font-mono lowercase",
            div {
                class: "space-y-1.5",
                label { class: "block text-zinc-500", "{form_strings.name_label}" }
                input {
                    r#type: "text",
                    required: true,
                    placeholder: "{form_strings.name_placeholder}",
                    value: "{name}",
                    oninput: move |e| name.set(e.value()),
                    class: "w-full p-3 rounded-xs border border-zinc-200 bg-white focus:border-zinc-950 focus:outline-none text-zinc-900 transition-colors"
                }
            }
            div {
                class: "space-y-1.5",
                label { class: "block text-zinc-500", "{form_strings.contact_email_label}" }
                input {
                    r#type: "email",
                    required: true,
                    placeholder: "{form_strings.contact_email_placeholder}",
                    value: "{email}",
                    oninput: move |e| email.set(e.value()),
                    class: "w-full p-3 rounded-xs border border-zinc-200 bg-white focus:border-zinc-950 focus:outline-none text-zinc-900 transition-colors"
                }
            }
        }
    }
}

/// Inputs for target timeline and budget envelope.
#[component]
fn IntakeParametersInputs(
    mut timeline: Signal<String>,
    mut budget: Signal<String>,
    form_strings: ContactFormStrings,
) -> Element {
    rsx! {
        div {
            class: "grid grid-cols-2 gap-3 sm:gap-4 text-xs font-mono lowercase",
            div {
                class: "space-y-1.5",
                label { class: "block text-zinc-500", "{form_strings.timeline_label}" }
                input {
                    r#type: "text",
                    placeholder: "{form_strings.timeline_placeholder}",
                    value: "{timeline}",
                    oninput: move |e| timeline.set(e.value()),
                    class: "w-full p-3 rounded-xs border border-zinc-200 bg-white focus:border-zinc-950 focus:outline-none text-zinc-900 transition-colors"
                }
            }
            div {
                class: "space-y-1.5",
                label { class: "block text-zinc-500", "{form_strings.budget_label}" }
                input {
                    r#type: "text",
                    placeholder: "{form_strings.budget_placeholder}",
                    value: "{budget}",
                    oninput: move |e| budget.set(e.value()),
                    class: "w-full p-3 rounded-xs border border-zinc-200 bg-white focus:border-zinc-950 focus:outline-none text-zinc-900 transition-colors"
                }
            }
        }
    }
}

/// Textarea input for comprehensive project brief and requirements.
#[component]
fn BriefInput(
    mut project_brief: Signal<String>,
    form_strings: ContactFormStrings,
) -> Element {
    rsx! {
        div {
            class: "space-y-1.5 text-xs font-mono lowercase",
            label { class: "block text-zinc-500", "{form_strings.brief_label}" }
            textarea {
                rows: 5,
                required: true,
                placeholder: "{form_strings.brief_placeholder}",
                value: "{project_brief}",
                oninput: move |e| project_brief.set(e.value()),
                class: "w-full p-3 rounded-xs border border-zinc-200 bg-white focus:border-zinc-950 focus:outline-none text-zinc-900 transition-colors font-sans text-xs leading-relaxed"
            }
        }
    }
}

/// Submit button and direct dispatch indicator.
#[component]
fn SubmitRow(form_strings: ContactFormStrings) -> Element {
    rsx! {
        div {
            class: "pt-2 flex flex-col sm:flex-row sm:items-center justify-between gap-3 select-none",
            button {
                r#type: "submit",
                class: "inline-flex items-center justify-center gap-2 px-6 py-3 rounded-xs bg-zinc-950 hover:bg-zinc-800 text-white font-mono text-xs font-bold tracking-wider uppercase transition-colors shadow-xs cursor-pointer",
                "{form_strings.submit_button}"
                span { "→" }
            }
            span { class: "text-[11px] font-mono text-zinc-400 lowercase tracking-tight", "{form_strings.dispatch_note}" }
        }
    }
}
