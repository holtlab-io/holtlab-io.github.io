//! ## Overview
//! Interactive diagnostic RFQ form allowing prospective clients to submit hardware scope and specifications.
//! Employs clean Swiss typography with quiet inputs and a direct mailto dispatcher.
//!
//! ## Search Tags
//! #contact, #form, #rfq, #inquiry, #interactive
//!
//! ## Submodules
//! None. This module provides atomic RFQ intake form components.

use dioxus::prelude::*;
use super::selectors::{ScopeButtons, TargetButtons};
use super::submitted::SubmittedView;

/// Interactive RFQ diagnostic form component with scope selector and email dispatcher.
#[component]
pub fn ContactForm(
    selected_scope: Signal<usize>,
    selected_target: Signal<usize>,
    name: Signal<String>,
    email: Signal<String>,
    timeline: Signal<String>,
    budget: Signal<String>,
    cad_pcb_reqs: Signal<String>,
    project_brief: Signal<String>,
    is_submitted: Signal<bool>,
) -> Element {
    rsx! {
        div {
            class: "lg:col-span-7 rounded-xs border border-zinc-200 bg-white p-5 sm:p-8 shadow-2xs relative font-sans",
            if is_submitted() {
                SubmittedView {
                    name,
                    email,
                    timeline,
                    budget,
                    cad_pcb_reqs,
                    project_brief,
                    selected_scope,
                    selected_target,
                    is_submitted,
                }
            } else {
                FormFields {
                    selected_scope,
                    selected_target,
                    name,
                    email,
                    timeline,
                    budget,
                    cad_pcb_reqs,
                    project_brief,
                    is_submitted,
                }
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
    timeline: Signal<String>,
    budget: Signal<String>,
    cad_pcb_reqs: Signal<String>,
    project_brief: Signal<String>,
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
            IntakeParametersInputs { timeline, budget }
            IntakeSpecsInputs { cad_pcb_reqs, project_brief }
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

/// Inputs for target timeline and budget envelope.
#[component]
fn IntakeParametersInputs(mut timeline: Signal<String>, mut budget: Signal<String>) -> Element {
    rsx! {
        div {
            class: "grid grid-cols-1 sm:grid-cols-2 gap-4 text-xs font-mono lowercase",
            div {
                class: "space-y-1.5",
                label { class: "block text-zinc-500", "target timeline / deadline" }
                input {
                    r#type: "text",
                    placeholder: "e.g. 4-6 weeks to working prototype",
                    value: "{timeline}",
                    oninput: move |e| timeline.set(e.value()),
                    class: "w-full p-3 rounded-xs border border-zinc-200 bg-white focus:border-zinc-950 focus:outline-none text-zinc-900 transition-colors"
                }
            }
            div {
                class: "space-y-1.5",
                label { class: "block text-zinc-500", "target budget envelope" }
                input {
                    r#type: "text",
                    placeholder: "e.g. $5k-$15k USD / KRW 10M",
                    value: "{budget}",
                    oninput: move |e| budget.set(e.value()),
                    class: "w-full p-3 rounded-xs border border-zinc-200 bg-white focus:border-zinc-950 focus:outline-none text-zinc-900 transition-colors"
                }
            }
        }
    }
}

/// Inputs for CAD/PCB requirements and project brief.
#[component]
fn IntakeSpecsInputs(mut cad_pcb_reqs: Signal<String>, mut project_brief: Signal<String>) -> Element {
    rsx! {
        div {
            class: "space-y-4 text-xs font-mono lowercase",
            div {
                class: "space-y-1.5",
                label { class: "block text-zinc-500", "cad & pcb requirements" }
                input {
                    r#type: "text",
                    placeholder: "e.g. ESP32-C3 / RISC-V, 4-layer impedance control, MJF enclosure...",
                    value: "{cad_pcb_reqs}",
                    oninput: move |e| cad_pcb_reqs.set(e.value()),
                    class: "w-full p-3 rounded-xs border border-zinc-200 bg-white focus:border-zinc-950 focus:outline-none text-zinc-900 transition-colors"
                }
            }
            div {
                class: "space-y-1.5",
                label { class: "block text-zinc-500", "project brief & deliverables" }
                textarea {
                    rows: 4,
                    required: true,
                    placeholder: "describe dimensions, interfaces, sensors, battery runtime, and key deliverables...",
                    value: "{project_brief}",
                    oninput: move |e| project_brief.set(e.value()),
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
