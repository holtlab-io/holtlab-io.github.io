//! ## Overview
//! Dispatch confirmation view rendering formatted intake telemetry and mailto dispatch triggers.
//! Provides clear review of submitted specifications with an edit button to return to form editing.
//!
//! ## Search Tags
//! #contact, #dispatch, #submitted, #buffer, #mailto
//!
//! ## Submodules
//! None. This module provides atomic dispatch confirmation components.

use dioxus::prelude::*;

/// Confirmation message and email mailto dispatch trigger.
#[component]
pub fn SubmittedView(
    name: Signal<String>,
    email: Signal<String>,
    timeline: Signal<String>,
    budget: Signal<String>,
    cad_pcb_reqs: Signal<String>,
    project_brief: Signal<String>,
    selected_scope: Signal<usize>,
    selected_target: Signal<usize>,
    is_submitted: Signal<bool>,
) -> Element {
    let scope_text = match selected_scope() {
        0 => "turnkey (mcad+ecad+firmware)",
        1 => "4-layer pcba (kicad ecad)",
        2 => "3d enclosure (build123d mcad)",
        _ => "bare-metal firmware (rust/c)",
    };

    let target_text = match selected_target() {
        0 => "turnkey hardware commission",
        1 => "early-stage startup",
        2 => "r&d laboratory",
        _ => "direct component bring-up",
    };

    let raw_timeline = timeline();
    let timeline_val = if raw_timeline.trim().is_empty() {
        "standard timeline".to_string()
    } else {
        raw_timeline
    };

    let raw_budget = budget();
    let budget_val = if raw_budget.trim().is_empty() {
        "standard budget envelope".to_string()
    } else {
        raw_budget
    };

    let raw_cad = cad_pcb_reqs();
    let cad_val = if raw_cad.trim().is_empty() {
        "standard specifications".to_string()
    } else {
        raw_cad
    };

    let mailto_url = format!(
        "mailto:contact@holtlab.io?subject=[Holt Lab Commission] {} — {}&body=organization: {}%0D%0Acontact email: {}%0D%0Aproject profile: {}%0D%0Adiscipline: {}%0D%0Atarget timeline: {}%0D%0Atarget budget: {}%0D%0Acad/pcb requirements: {}%0D%0Aproject brief & deliverables:%0D%0A{}",
        name(), scope_text, name(), email(), target_text, scope_text, timeline_val, budget_val, cad_val, project_brief()
    );

    rsx! {
        div {
            class: "p-8 border border-zinc-200 bg-zinc-50 text-center space-y-4 rounded-xs font-mono",
            SubmittedHeader {}
            SubmittedSummaryCard {
                name: name(),
                email: email(),
                scope: scope_text,
                target: target_text,
                timeline: timeline_val,
                budget: budget_val,
                cad: cad_val,
            }
            SubmittedActions { mailto_url, is_submitted }
        }
    }
}

/// Header message block for the dispatch confirmation buffer.
#[component]
fn SubmittedHeader() -> Element {
    rsx! {
        div {
            class: "space-y-2",
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
        }
    }
}

/// Summary specification card rendering client intake data.
#[component]
fn SubmittedSummaryCard(
    name: String,
    email: String,
    scope: &'static str,
    target: &'static str,
    timeline: String,
    budget: String,
    cad: String,
) -> Element {
    rsx! {
        div {
            class: "text-left bg-white border border-zinc-200 p-4 rounded-xs text-[11px] font-mono text-zinc-700 space-y-1 my-3",
            div { span { class: "text-zinc-400", "ORGANIZATION: " } "{name}" }
            div { span { class: "text-zinc-400", "EMAIL: " } "{email}" }
            div { span { class: "text-zinc-400", "DISCIPLINE: " } "{scope}" }
            div { span { class: "text-zinc-400", "PROFILE: " } "{target}" }
            div { span { class: "text-zinc-400", "TIMELINE: " } "{timeline}" }
            div { span { class: "text-zinc-400", "BUDGET: " } "{budget}" }
            div { span { class: "text-zinc-400", "CAD/PCB: " } "{cad}" }
        }
    }
}

/// Action buttons allowing email client trigger and specification editing.
#[component]
fn SubmittedActions(mailto_url: String, mut is_submitted: Signal<bool>) -> Element {
    rsx! {
        div {
            class: "flex flex-col sm:flex-row items-center justify-center gap-3 pt-2",
            a {
                href: "{mailto_url}",
                class: "w-full sm:w-auto inline-flex items-center justify-center px-6 py-3 bg-zinc-950 hover:bg-zinc-800 text-white font-bold text-xs font-mono tracking-wider uppercase transition-colors shadow-xs",
                "open mail client →"
            }
            button {
                r#type: "button",
                onclick: move |_| is_submitted.set(false),
                class: "w-full sm:w-auto inline-flex items-center justify-center px-6 py-3 bg-white hover:bg-zinc-100 border border-zinc-300 text-zinc-900 font-bold text-xs font-mono tracking-wider uppercase transition-colors shadow-2xs",
                "← edit specifications"
            }
        }
    }
}
