use dioxus::prelude::*;

#[component]
pub fn Contact() -> Element {
    let mut selected_scope = use_signal(|| 0); // 0: Turnkey, 1: ECAD, 2: MCAD, 3: Firmware
    let mut selected_target = use_signal(|| 0); // 0: Grant, 1: Startup, 2: Enterprise, 3: Other
    let mut name = use_signal(|| String::new());
    let mut email = use_signal(|| String::new());
    let mut phone = use_signal(|| String::new());
    let mut description = use_signal(|| String::new());
    let mut is_submitted = use_signal(|| false);

    let handle_submit = move |_| {
        // Form submission triggers direct email generation
        is_submitted.set(true);
    };

    rsx! {
        section {
            id: "contact",
            class: "py-24 bg-[#090D14] relative border-t border-white/[0.08]",
            div {
                class: "max-w-7xl mx-auto px-4 sm:px-6 lg:px-8",
                div {
                    class: "grid grid-cols-1 lg:grid-cols-12 gap-12 items-start",
                    // Left Column: Direct Info & Value Assurance
                    div {
                        class: "lg:col-span-5 space-y-8",
                        div {
                            class: "space-y-4",
                            div {
                                class: "inline-flex items-center gap-2 px-3 py-1 rounded-full bg-orange-500/10 border border-orange-500/20 text-orange-400 text-xs font-mono font-medium",
                                "START YOUR PROJECT // DIRECT INQUIRY"
                            }
                            h2 {
                                class: "text-3xl sm:text-4xl font-extrabold text-white tracking-tight",
                                "아이디어만 들고 오셔도"
                                br {}
                                span {
                                    class: "text-transparent bg-clip-text bg-gradient-to-r from-orange-400 to-amber-300",
                                    "실물로 완성해 드립니다."
                                }
                            }
                            p {
                                class: "text-slate-300 text-sm sm:text-base leading-relaxed",
                                "기획서나 스케치가 완벽하지 않아도 괜찮습니다. 3분 진단 양식을 남겨주시면 "
                                strong { class: "text-white font-semibold", "24시간 이내에 리드 엔지니어가 직접" }
                                " 기술적 실현 가능성과 예상 공정을 검토해 드립니다."
                            }
                        }

                        // Direct Info Cards
                        div {
                            class: "space-y-4 text-xs font-mono",
                            div {
                                class: "p-4 rounded-xl bg-[#121622] border border-white/[0.08] flex items-center gap-4",
                                div { class: "w-10 h-10 rounded-lg bg-orange-500/10 border border-orange-500/20 flex items-center justify-center text-orange-400 text-lg", "✉" },
                                div {
                                    div { class: "text-slate-400 text-[11px]", "공식 문의 이메일" }
                                    a {
                                        href: "mailto:contact@holtlab.io",
                                        class: "text-sm font-semibold text-white hover:text-orange-400 transition-colors",
                                        "contact@holtlab.io"
                                    }
                                }
                            }

                            div {
                                class: "p-4 rounded-xl bg-[#121622] border border-white/[0.08] flex items-center gap-4",
                                div { class: "w-10 h-10 rounded-lg bg-emerald-500/10 border border-emerald-500/20 flex items-center justify-center text-emerald-400 text-lg", "✓" },
                                div {
                                    div { class: "text-slate-400 text-[11px]", "정부지원사업 지출증빙" }
                                    div { class: "text-sm font-semibold text-white", "전자계약 및 세금계산서 100% 지원" }
                                }
                            }

                            div {
                                class: "p-4 rounded-xl bg-[#121622] border border-white/[0.08] flex items-center gap-4",
                                div { class: "w-10 h-10 rounded-lg bg-sky-500/10 border border-sky-500/20 flex items-center justify-center text-sky-400 text-lg", "⏱" },
                                div {
                                    div { class: "text-slate-400 text-[11px]", "회신 속도 보증" }
                                    div { class: "text-sm font-semibold text-white", "영업일 기준 24시간 이내 1차 검토 회신" }
                                }
                            }
                        }
                    }

                    // Right Column: Interactive Diagnostic Form
                    div {
                        class: "lg:col-span-7 rounded-3xl bg-[#121722] border border-white/10 p-6 sm:p-10 shadow-2xl backdrop-blur-xl space-y-6",
                        if is_submitted() {
                            div {
                                class: "p-8 rounded-2xl bg-emerald-500/10 border border-emerald-500/30 text-center space-y-4",
                                div { class: "text-3xl", "🎉" }
                                h3 { class: "text-xl font-bold text-white", "사전 진단 양식이 준비되었습니다!" }
                                p {
                                    class: "text-slate-300 text-sm leading-relaxed",
                                    "작성하신 내용을 바탕으로 이메일 클라이언트가 열립니다. 내용 확인 후 전송 버튼만 눌러주시면 즉시 검토를 시작합니다."
                                }
                                a {
                                    href: format!(
                                        "mailto:contact@holtlab.io?subject=[Holt Lab 의뢰] {} 님의 프로젝트 문의&body=이름/회사: {}%0D%0A연락처: {}%0D%0A이메일: {}%0D%0A의뢰유형: {}%0D%0A프로젝트 설명:%0D%0A{}",
                                        name(), name(), phone(), email(),
                                        if selected_scope() == 0 { "기구+회로+펌웨어 턴키" } else if selected_scope() == 1 { "회로(ECAD)" } else if selected_scope() == 2 { "기구(MCAD)" } else { "펌웨어" },
                                        description()
                                    ),
                                    class: "inline-flex items-center justify-center px-6 py-3 rounded-xl bg-orange-500 hover:bg-orange-600 text-white font-semibold text-sm shadow-md transition-all",
                                    "이메일 발송 창 열기 →"
                                }
                            }
                        } else {
                            div {
                                class: "space-y-6",
                                // 1. Scope Selection
                                div {
                                    class: "space-y-2.5",
                                    label { class: "block text-xs font-mono font-semibold uppercase tracking-wider text-slate-300", "01. 희망 개발 범위" }
                                    div {
                                        class: "grid grid-cols-2 gap-2.5",
                                        button {
                                            r#type: "button",
                                            onclick: move |_| selected_scope.set(0),
                                            class: if selected_scope() == 0 {
                                                "p-3 rounded-xl border border-orange-500 bg-orange-500/10 text-white text-xs font-semibold text-left transition-all"
                                            } else {
                                                "p-3 rounded-xl border border-white/10 bg-white/[0.02] text-slate-400 hover:text-white text-xs text-left transition-all"
                                            },
                                            "⚡ 기구+회로+펌웨어 턴키 (추천)"
                                        }
                                        button {
                                            r#type: "button",
                                            onclick: move |_| selected_scope.set(1),
                                            class: if selected_scope() == 1 {
                                                "p-3 rounded-xl border border-orange-500 bg-orange-500/10 text-white text-xs font-semibold text-left transition-all"
                                            } else {
                                                "p-3 rounded-xl border border-white/10 bg-white/[0.02] text-slate-400 hover:text-white text-xs text-left transition-all"
                                            },
                                            "📟 4/6층 회로 & PCB (ECAD)"
                                        }
                                        button {
                                            r#type: "button",
                                            onclick: move |_| selected_scope.set(2),
                                            class: if selected_scope() == 2 {
                                                "p-3 rounded-xl border border-orange-500 bg-orange-500/10 text-white text-xs font-semibold text-left transition-all"
                                            } else {
                                                "p-3 rounded-xl border border-white/10 bg-white/[0.02] text-slate-400 hover:text-white text-xs text-left transition-all"
                                            },
                                            "📐 3D 기구 & 외함 (MCAD)"
                                        }
                                        button {
                                            r#type: "button",
                                            onclick: move |_| selected_scope.set(3),
                                            class: if selected_scope() == 3 {
                                                "p-3 rounded-xl border border-orange-500 bg-orange-500/10 text-white text-xs font-semibold text-left transition-all"
                                            } else {
                                                "p-3 rounded-xl border border-white/10 bg-white/[0.02] text-slate-400 hover:text-white text-xs text-left transition-all"
                                            },
                                            "💻 임베디드 펌웨어 개발"
                                        }
                                    }
                                }

                                // 2. Client Category
                                div {
                                    class: "space-y-2.5",
                                    label { class: "block text-xs font-mono font-semibold uppercase tracking-wider text-slate-300", "02. 고객 유형" }
                                    div {
                                        class: "grid grid-cols-2 sm:grid-cols-4 gap-2",
                                        button {
                                            r#type: "button",
                                            onclick: move |_| selected_target.set(0),
                                            class: if selected_target() == 0 {
                                                "p-2.5 rounded-lg border border-orange-500 bg-orange-500/10 text-white text-xs font-medium text-center"
                                            } else {
                                                "p-2.5 rounded-lg border border-white/10 bg-white/[0.02] text-slate-400 text-xs text-center hover:text-white"
                                            },
                                            "예창패/청창사"
                                        }
                                        button {
                                            r#type: "button",
                                            onclick: move |_| selected_target.set(1),
                                            class: if selected_target() == 1 {
                                                "p-2.5 rounded-lg border border-orange-500 bg-orange-500/10 text-white text-xs font-medium text-center"
                                            } else {
                                                "p-2.5 rounded-lg border border-white/10 bg-white/[0.02] text-slate-400 text-xs text-center hover:text-white"
                                            },
                                            "초기 스타트업"
                                        }
                                        button {
                                            r#type: "button",
                                            onclick: move |_| selected_target.set(2),
                                            class: if selected_target() == 2 {
                                                "p-2.5 rounded-lg border border-orange-500 bg-orange-500/10 text-white text-xs font-medium text-center"
                                            } else {
                                                "p-2.5 rounded-lg border border-white/10 bg-white/[0.02] text-slate-400 text-xs text-center hover:text-white"
                                            },
                                            "기업 연구소"
                                        }
                                        button {
                                            r#type: "button",
                                            onclick: move |_| selected_target.set(3),
                                            class: if selected_target() == 3 {
                                                "p-2.5 rounded-lg border border-orange-500 bg-orange-500/10 text-white text-xs font-medium text-center"
                                            } else {
                                                "p-2.5 rounded-lg border border-white/10 bg-white/[0.02] text-slate-400 text-xs text-center hover:text-white"
                                            },
                                            "개인/기타"
                                        }
                                    }
                                }

                                // 3. Text Inputs
                                div {
                                    class: "grid grid-cols-1 sm:grid-cols-2 gap-4",
                                    div {
                                        label { class: "block text-xs font-mono text-slate-300 mb-1.5", "성함 / 회사명 *" }
                                        input {
                                            r#type: "text",
                                            placeholder: "홍길동 / (주)홀트테크",
                                            value: "{name}",
                                            oninput: move |e| name.set(e.value()),
                                            class: "w-full px-3.5 py-2.5 rounded-xl bg-white/[0.04] border border-white/10 text-white text-sm focus:border-orange-500 focus:outline-none"
                                        }
                                    }
                                    div {
                                        label { class: "block text-xs font-mono text-slate-300 mb-1.5", "연락처 / 전화번호 *" }
                                        input {
                                            r#type: "tel",
                                            placeholder: "010-0000-0000",
                                            value: "{phone}",
                                            oninput: move |e| phone.set(e.value()),
                                            class: "w-full px-3.5 py-2.5 rounded-xl bg-white/[0.04] border border-white/10 text-white text-sm focus:border-orange-500 focus:outline-none"
                                        }
                                    }
                                }

                                div {
                                    label { class: "block text-xs font-mono text-slate-300 mb-1.5", "이메일 주소 *" }
                                    input {
                                        r#type: "email",
                                        placeholder: "contact@company.com",
                                        value: "{email}",
                                        oninput: move |e| email.set(e.value()),
                                        class: "w-full px-3.5 py-2.5 rounded-xl bg-white/[0.04] border border-white/10 text-white text-sm focus:border-orange-500 focus:outline-none"
                                    }
                                }

                                div {
                                    label { class: "block text-xs font-mono text-slate-300 mb-1.5", "프로젝트 내용 & 목표 일정" }
                                    textarea {
                                        rows: 4,
                                        placeholder: "예) 배터리 기반 소형 IoT 센서 장치 시제품을 4주 내로 만들고 싶습니다. 2.0인치 화면과 블루투스 기능이 필요합니다.",
                                        value: "{description}",
                                        oninput: move |e| description.set(e.value()),
                                        class: "w-full px-3.5 py-2.5 rounded-xl bg-white/[0.04] border border-white/10 text-white text-sm focus:border-orange-500 focus:outline-none"
                                    }
                                }

                                // Submit Button
                                button {
                                    r#type: "button",
                                    onclick: handle_submit,
                                    class: "w-full py-4 rounded-xl bg-orange-500 hover:bg-orange-600 text-white font-bold text-base shadow-xl shadow-orange-500/25 hover:shadow-orange-500/40 transition-all active:scale-[0.99]",
                                    "30분 무료 타당성 진단 신청하기 →"
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
