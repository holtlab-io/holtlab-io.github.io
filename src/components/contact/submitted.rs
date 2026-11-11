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
use crate::i18n::{contact::get_contact_strings, ContactBufferStrings, Language};

/// Confirmation message and email mailto dispatch trigger.
#[component]
pub fn SubmittedView(
    name: Signal<String>,
    email: Signal<String>,
    timeline: Signal<String>,
    budget: Signal<String>,
    project_brief: Signal<String>,
    selected_scope: Signal<usize>,
    is_submitted: Signal<bool>,
) -> Element {
    let lang = use_context::<Signal<Language>>();
    let contact_strings = get_contact_strings(lang());
    let buffer = contact_strings.buffer;
    let is_ko = lang() == Language::Ko;

    let scope_text = contact_strings.selectors.scopes[selected_scope().min(contact_strings.selectors.scopes.len() - 1)];

    let raw_timeline = timeline();
    let timeline_val = if raw_timeline.trim().is_empty() {
        if is_ko { "표준 일정".to_string() } else { "standard timeline".to_string() }
    } else {
        raw_timeline
    };

    let raw_budget = budget();
    let budget_val = if raw_budget.trim().is_empty() {
        if is_ko { "표준 예산 범위".to_string() } else { "standard budget envelope".to_string() }
    } else {
        raw_budget
    };

    let subject_tag = if is_ko { "Holt Lab 하드웨어 개발 의뢰" } else { "Holt Lab Commission" };
    let raw_subject = format!("[{}] {} — {}", subject_tag, name(), scope_text);
    let raw_body = format!(
        "name / organization: {}\ncontact email: {}\ndiscipline: {}\ntarget timeline: {}\ntarget budget: {}\n\nproject brief & requirements:\n{}",
        name(), email(), scope_text, timeline_val, budget_val, project_brief()
    );
    let mailto_url = format!(
        "mailto:contact@holtlab.io?subject={}&body={}",
        percent_encode(&raw_subject),
        percent_encode(&raw_body)
    );

    rsx! {
        div {
            class: "p-6 sm:p-8 border border-zinc-200 bg-zinc-50 text-center space-y-4 rounded-xs font-mono",
            SubmittedHeader { buffer }
            SubmittedSummaryCard {
                name: name(),
                email: email(),
                scope: scope_text,
                timeline: timeline_val,
                budget: budget_val,
                brief: project_brief(),
                is_ko,
            }
            SubmittedActions { mailto_url, is_submitted, buffer }
        }
    }
}

/// Header message block for the dispatch confirmation buffer.
#[component]
fn SubmittedHeader(buffer: ContactBufferStrings) -> Element {
    rsx! {
        div {
            class: "space-y-2",
            div {
                class: "inline-flex items-center gap-1.5 px-3 py-1 rounded-full bg-zinc-200 text-zinc-900 text-xs font-bold uppercase",
                span { class: "w-1.5 h-1.5 rounded-full bg-zinc-950" }
                "{buffer.buffer_badge}"
            }
            h3 { class: "text-lg font-bold text-zinc-950 font-sans", "{buffer.buffer_title}" }
            p {
                class: "text-zinc-600 text-xs font-sans leading-relaxed max-w-md mx-auto",
                "{buffer.buffer_desc}"
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
    timeline: String,
    budget: String,
    brief: String,
    is_ko: bool,
) -> Element {
    let lbl_org = if is_ko { "성함/기업명: " } else { "NAME/ORG: " };
    let lbl_email = if is_ko { "이메일: " } else { "EMAIL: " };
    let lbl_disc = if is_ko { "개발분야: " } else { "DISCIPLINE: " };
    let lbl_time = if is_ko { "희망일정: " } else { "TIMELINE: " };
    let lbl_budg = if is_ko { "희망예산: " } else { "BUDGET: " };
    let lbl_brief = if is_ko { "요구사항: " } else { "REQUIREMENTS: " };

    rsx! {
        div {
            class: "text-left bg-white border border-zinc-200 p-4 rounded-xs text-[11px] font-mono text-zinc-700 space-y-1.5 my-3",
            div { span { class: "text-zinc-400", "{lbl_org}" } "{name}" }
            div { span { class: "text-zinc-400", "{lbl_email}" } "{email}" }
            div { span { class: "text-zinc-400", "{lbl_disc}" } "{scope}" }
            div { span { class: "text-zinc-400", "{lbl_time}" } "{timeline}" }
            div { span { class: "text-zinc-400", "{lbl_budg}" } "{budget}" }
            div {
                class: "pt-1 border-t border-zinc-100",
                span { class: "text-zinc-400 block mb-0.5", "{lbl_brief}" }
                span { class: "font-sans text-xs text-zinc-800 whitespace-pre-wrap break-words", "{brief}" }
            }
        }
    }
}

/// Action buttons allowing email client trigger and specification editing.
#[component]
fn SubmittedActions(
    mailto_url: String,
    mut is_submitted: Signal<bool>,
    buffer: ContactBufferStrings,
) -> Element {
    rsx! {
        div {
            class: "flex flex-col sm:flex-row items-center justify-center gap-3 pt-2 select-none",
            a {
                href: "{mailto_url}",
                class: "w-full sm:w-auto inline-flex items-center justify-center px-6 py-3 bg-zinc-950 hover:bg-zinc-800 text-white font-bold text-xs font-mono tracking-wider uppercase transition-colors shadow-xs cursor-pointer",
                "{buffer.btn_open_mail}"
            }
            button {
                r#type: "button",
                onclick: move |_| is_submitted.set(false),
                class: "w-full sm:w-auto inline-flex items-center justify-center px-6 py-3 bg-white hover:bg-zinc-100 border border-zinc-300 text-zinc-900 font-bold text-xs font-mono tracking-wider uppercase transition-colors shadow-2xs cursor-pointer",
                "{buffer.btn_edit_specs}"
            }
        }
    }
}

/// Encodes query parameters conforming to RFC 3986 and RFC 6068 mailto URI standards.
fn percent_encode(input: &str) -> String {
    let mut out = String::with_capacity(input.len() * 3);
    for byte in input.as_bytes() {
        match *byte {
            b'a'..=b'z' | b'A'..=b'Z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~' | b'@' => {
                out.push(*byte as char);
            }
            b'\n' => out.push_str("%0D%0A"),
            b'\r' => {},
            b => {
                use std::fmt::Write;
                let _ = write!(out, "%{:02X}", b);
            }
        }
    }
    out
}
