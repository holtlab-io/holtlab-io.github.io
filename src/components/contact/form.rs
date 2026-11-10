//! ## Overview
//! Interactive diagnostic RFQ form allowing prospective clients to submit hardware scope and specifications.
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
            class: "lg:col-span-7 rounded-xl border border-zinc-200 bg-white p-5 sm:p-8 shadow-xs relative",
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
    let scope_text = if selected_scope() == 0 {
        "기구+회로+펌웨어 턴키"
    } else if selected_scope() == 1 {
        "회로(ECAD)"
    } else if selected_scope() == 2 {
        "기구(MCAD)"
    } else {
        "펌웨어"
    };

    rsx! {
        div {
            class: "p-6 rounded-lg border border-zinc-200 bg-zinc-50 text-center space-y-4",
            div { class: "text-zinc-900 text-xs font-bold tracking-wider", "[+] RFQ DISPATCH BUFFER READY" }
            h3 { class: "text-lg font-bold text-zinc-900 font-sans", "프로젝트 문의 메일이 준비되었습니다." }
            p {
                class: "text-zinc-600 text-xs font-sans leading-relaxed",
                "작성하신 내용을 바탕으로 메일 클라이언트가 호출됩니다. 내용 확인 후 전송 버튼만 눌러주시면 즉시 분석을 시작합니다."
            }
            a {
                href: format!(
                    "mailto:contact@holtlab.io?subject=[Holt Lab 의뢰] {} 님의 프로젝트 문의&body=이름/회사: {}%0D%0A연락처: {}%0D%0A이메일: {}%0D%0A의뢰유형: {}%0D%0A프로젝트 설명:%0D%0A{}",
                    name(), name(), phone(), email(), scope_text, description()
                ),
                class: "inline-flex items-center justify-center px-6 py-3 rounded-md bg-[#EA580C] hover:bg-[#C2410C] text-white font-bold text-xs tracking-wider uppercase transition-all shadow-xs tactile-button",
                "메일 발송 창 열기 →"
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
        div {
            class: "space-y-5 text-xs",
            ScopeButtons { selected: selected_scope }
            TargetButtons { selected: selected_target }
            FormInputs { name, email, phone, description }

            button {
                r#type: "button",
                onclick: move |_| is_submitted.set(true),
                class: "w-full py-3.5 rounded-md bg-[#EA580C] hover:bg-[#C2410C] text-white font-bold text-xs tracking-wider uppercase transition-all shadow-xs tactile-button active:scale-[0.99]",
                "기술 타당성 검토 요청 →"
            }
        }
    }
}

/// Text inputs and description textarea.
#[component]
fn FormInputs(
    mut name: Signal<String>,
    mut email: Signal<String>,
    mut phone: Signal<String>,
    mut description: Signal<String>,
) -> Element {
    rsx! {
        div {
            class: "space-y-3",
            div {
                class: "grid grid-cols-1 sm:grid-cols-2 gap-3",
                div {
                    label { class: "block text-[11px] font-semibold text-zinc-700 mb-1", "성함 / 회사명 *" }
                    input {
                        r#type: "text",
                        placeholder: "홍길동 / (주)홀트테크",
                        value: "{name}",
                        oninput: move |e| name.set(e.value()),
                        class: "w-full px-3 py-2 rounded-md bg-zinc-50 border border-zinc-300 text-zinc-900 text-xs focus:bg-white focus:border-zinc-900 focus:outline-none transition-colors"
                    }
                }
                div {
                    label { class: "block text-[11px] font-semibold text-zinc-700 mb-1", "연락처 / 전화번호 *" }
                    input {
                        r#type: "tel",
                        placeholder: "010-0000-0000",
                        value: "{phone}",
                        oninput: move |e| phone.set(e.value()),
                        class: "w-full px-3 py-2 rounded-md bg-zinc-50 border border-zinc-300 text-zinc-900 text-xs focus:bg-white focus:border-zinc-900 focus:outline-none transition-colors"
                    }
                }
            }

            div {
                label { class: "block text-[11px] font-semibold text-zinc-700 mb-1", "이메일 주소 *" }
                input {
                    r#type: "email",
                    placeholder: "contact@company.com",
                    value: "{email}",
                    oninput: move |e| email.set(e.value()),
                    class: "w-full px-3 py-2 rounded-md bg-zinc-50 border border-zinc-300 text-zinc-900 text-xs focus:bg-white focus:border-zinc-900 focus:outline-none transition-colors"
                }
            }

            div {
                label { class: "block text-[11px] font-semibold text-zinc-700 mb-1", "프로젝트 개요 & 목표 납기" }
                textarea {
                    rows: 3,
                    placeholder: "배터리 기반 초소형 IoT 센서 하드웨어 시제품 제작 희망. 디스플레이 구동 및 3D 하우징 포함 4주 목표.",
                    value: "{description}",
                    oninput: move |e| description.set(e.value()),
                    class: "w-full px-3 py-2 rounded-md bg-zinc-50 border border-zinc-300 text-zinc-900 text-xs focus:bg-white focus:border-zinc-900 focus:outline-none transition-colors"
                }
            }
        }
    }
}
