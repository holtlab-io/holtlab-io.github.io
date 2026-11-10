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
        is_submitted.set(true);
    };

    rsx! {
        section {
            id: "contact",
            class: "py-20 bg-[#06080C] relative font-mono",
            div {
                class: "max-w-7xl mx-auto px-4 sm:px-6 lg:px-8",
                div {
                    class: "grid grid-cols-1 lg:grid-cols-12 gap-10 items-start",
                    // Left Column: Direct Info & Value Assurance
                    div {
                        class: "lg:col-span-5 space-y-6",
                        div {
                            class: "space-y-3",
                            div {
                                class: "inline-flex items-center gap-2 px-2.5 py-1 rounded bg-white/[0.03] border border-white/10 text-zinc-400 text-xs tracking-widest uppercase",
                                "PROJECT RFQ // DIRECT INQUIRY"
                            }
                            h2 {
                                class: "text-2xl sm:text-4xl font-bold text-white tracking-tight font-sans",
                                "아이디어만 들고 오셔도 "
                                br {}
                                span { class: "text-zinc-400", "실물로 완성해 드립니다." }
                            }
                            p {
                                class: "text-zinc-400 text-xs sm:text-sm font-sans leading-relaxed",
                                "기획서나 도면이 완벽하지 않아도 괜찮습니다. 요구사항을 남겨주시면 "
                                span { class: "text-white font-medium", "24시간 이내에 리드 엔지니어가 직접" }
                                " 물리적 실현 가능성, 부품 수급성, 예상 일정과 예산을 검토해 회신드립니다."
                            }
                        }

                        // Direct Info Technical Cards
                        div {
                            class: "space-y-3 text-xs",
                            div {
                                class: "p-4 rounded border border-white/[0.08] bg-[#0A0D13] flex items-center justify-between",
                                div {
                                    div { class: "text-zinc-500 text-[10px] uppercase", "DIRECT CONTACT // EMAIL" }
                                    a {
                                        href: "mailto:contact@holtlab.io",
                                        class: "text-sm font-bold text-white hover:text-[#FF5722] transition-colors mt-0.5 block",
                                        "contact@holtlab.io"
                                    }
                                }
                                span { class: "text-zinc-600 text-xs", "→" }
                            }

                            div {
                                class: "p-4 rounded border border-white/[0.08] bg-[#0A0D13] flex items-center justify-between",
                                div {
                                    div { class: "text-zinc-500 text-[10px] uppercase", "GOVERNMENT GRANT // TAX COMPLIANCE" }
                                    div { class: "text-xs font-semibold text-zinc-200 mt-0.5", "전자계약 및 전자세금계산서 100% 발행 지원" }
                                }
                                span { class: "text-emerald-400 text-xs font-bold", "PASS" }
                            }

                            div {
                                class: "p-4 rounded border border-white/[0.08] bg-[#0A0D13] flex items-center justify-between",
                                div {
                                    div { class: "text-zinc-500 text-[10px] uppercase", "SLA // RESPONSE GUARANTEE" }
                                    div { class: "text-xs font-semibold text-zinc-200 mt-0.5", "영업일 기준 24시간 이내 1차 기술 검토 완료" }
                                }
                                span { class: "text-zinc-400 text-xs", "24H" }
                            }
                        }
                    }

                    // Right Column: Interactive Diagnostic Form Terminal
                    div {
                        class: "lg:col-span-7 rounded border border-white/[0.08] bg-[#0A0D13] p-5 sm:p-8 shadow-2xl relative",
                        if is_submitted() {
                            div {
                                class: "p-6 rounded border border-emerald-500/30 bg-emerald-500/5 text-center space-y-4",
                                div { class: "text-emerald-400 text-xs font-bold tracking-wider", "[+] RFQ DISPATCH BUFFER READY" }
                                h3 { class: "text-lg font-bold text-white font-sans", "프로젝트 문의 메일이 준비되었습니다." }
                                p {
                                    class: "text-zinc-300 text-xs font-sans leading-relaxed",
                                    "작성하신 내용을 바탕으로 메일 클라이언트가 호출됩니다. 내용 확인 후 전송 버튼만 눌러주시면 즉시 분석을 시작합니다."
                                }
                                a {
                                    href: format!(
                                        "mailto:contact@holtlab.io?subject=[Holt Lab 의뢰] {} 님의 프로젝트 문의&body=이름/회사: {}%0D%0A연락처: {}%0D%0A이메일: {}%0D%0A의뢰유형: {}%0D%0A프로젝트 설명:%0D%0A{}",
                                        name(), name(), phone(), email(),
                                        if selected_scope() == 0 { "기구+회로+펌웨어 턴키" } else if selected_scope() == 1 { "회로(ECAD)" } else if selected_scope() == 2 { "기구(MCAD)" } else { "펌웨어" },
                                        description()
                                    ),
                                    class: "inline-flex items-center justify-center px-6 py-3 rounded bg-[#FF5722] hover:bg-[#F4511E] text-white font-bold text-xs tracking-wider uppercase transition-all",
                                    "메일 발송 창 열기 →"
                                }
                            }
                        } else {
                            div {
                                class: "space-y-5 text-xs",
                                // 1. Scope Selection
                                div {
                                    class: "space-y-2",
                                    label { class: "block text-[11px] font-semibold uppercase tracking-wider text-zinc-400", "01 // 희망 개발 범위 (PROJECT SCOPE)" }
                                    div {
                                        class: "grid grid-cols-2 gap-2",
                                        button {
                                            r#type: "button",
                                            onclick: move |_| selected_scope.set(0),
                                            class: if selected_scope() == 0 {
                                                "p-2.5 rounded border border-[#FF5722] bg-[#FF5722]/10 text-white font-semibold text-left transition-all"
                                            } else {
                                                "p-2.5 rounded border border-white/10 bg-white/[0.02] text-zinc-400 hover:text-white text-left transition-all"
                                            },
                                            "[+] 턴키 완주 (기구+회로+펌웨어)"
                                        }
                                        button {
                                            r#type: "button",
                                            onclick: move |_| selected_scope.set(1),
                                            class: if selected_scope() == 1 {
                                                "p-2.5 rounded border border-[#FF5722] bg-[#FF5722]/10 text-white font-semibold text-left transition-all"
                                            } else {
                                                "p-2.5 rounded border border-white/10 bg-white/[0.02] text-zinc-400 hover:text-white text-left transition-all"
                                            },
                                            "[+] 회로 & 4/6층 PCB (ECAD)"
                                        }
                                        button {
                                            r#type: "button",
                                            onclick: move |_| selected_scope.set(2),
                                            class: if selected_scope() == 2 {
                                                "p-2.5 rounded border border-[#FF5722] bg-[#FF5722]/10 text-white font-semibold text-left transition-all"
                                            } else {
                                                "p-2.5 rounded border border-white/10 bg-white/[0.02] text-zinc-400 hover:text-white text-left transition-all"
                                            },
                                            "[+] 3D 기구 & 외함 (MCAD)"
                                        }
                                        button {
                                            r#type: "button",
                                            onclick: move |_| selected_scope.set(3),
                                            class: if selected_scope() == 3 {
                                                "p-2.5 rounded border border-[#FF5722] bg-[#FF5722]/10 text-white font-semibold text-left transition-all"
                                            } else {
                                                "p-2.5 rounded border border-white/10 bg-white/[0.02] text-zinc-400 hover:text-white text-left transition-all"
                                            },
                                            "[+] 임베디드 펌웨어 개발"
                                        }
                                    }
                                }

                                // 2. Client Category
                                div {
                                    class: "space-y-2",
                                    label { class: "block text-[11px] font-semibold uppercase tracking-wider text-zinc-400", "02 // 고객 유형 (CLIENT TYPE)" }
                                    div {
                                        class: "grid grid-cols-2 sm:grid-cols-4 gap-2",
                                        button {
                                            r#type: "button",
                                            onclick: move |_| selected_target.set(0),
                                            class: if selected_target() == 0 {
                                                "p-2 rounded border border-[#FF5722] bg-[#FF5722]/10 text-white font-medium text-center"
                                            } else {
                                                "p-2 rounded border border-white/10 bg-white/[0.02] text-zinc-400 text-center hover:text-white"
                                            },
                                            "예창패/청창사"
                                        }
                                        button {
                                            r#type: "button",
                                            onclick: move |_| selected_target.set(1),
                                            class: if selected_target() == 1 {
                                                "p-2 rounded border border-[#FF5722] bg-[#FF5722]/10 text-white font-medium text-center"
                                            } else {
                                                "p-2 rounded border border-white/10 bg-white/[0.02] text-zinc-400 text-center hover:text-white"
                                            },
                                            "초기 스타트업"
                                        }
                                        button {
                                            r#type: "button",
                                            onclick: move |_| selected_target.set(2),
                                            class: if selected_target() == 2 {
                                                "p-2 rounded border border-[#FF5722] bg-[#FF5722]/10 text-white font-medium text-center"
                                            } else {
                                                "p-2 rounded border border-white/10 bg-white/[0.02] text-zinc-400 text-center hover:text-white"
                                            },
                                            "기업 연구소"
                                        }
                                        button {
                                            r#type: "button",
                                            onclick: move |_| selected_target.set(3),
                                            class: if selected_target() == 3 {
                                                "p-2 rounded border border-[#FF5722] bg-[#FF5722]/10 text-white font-medium text-center"
                                            } else {
                                                "p-2 rounded border border-white/10 bg-white/[0.02] text-zinc-400 text-center hover:text-white"
                                            },
                                            "개인/기타"
                                        }
                                    }
                                }

                                // 3. Text Inputs
                                div {
                                    class: "grid grid-cols-1 sm:grid-cols-2 gap-3",
                                    div {
                                        label { class: "block text-[11px] text-zinc-400 mb-1", "성함 / 회사명 *" }
                                        input {
                                            r#type: "text",
                                            placeholder: "홍길동 / (주)홀트테크",
                                            value: "{name}",
                                            oninput: move |e| name.set(e.value()),
                                            class: "w-full px-3 py-2 rounded bg-white/[0.03] border border-white/10 text-white text-xs focus:border-[#FF5722] focus:outline-none"
                                        }
                                    }
                                    div {
                                        label { class: "block text-[11px] text-zinc-400 mb-1", "연락처 / 전화번호 *" }
                                        input {
                                            r#type: "tel",
                                            placeholder: "010-0000-0000",
                                            value: "{phone}",
                                            oninput: move |e| phone.set(e.value()),
                                            class: "w-full px-3 py-2 rounded bg-white/[0.03] border border-white/10 text-white text-xs focus:border-[#FF5722] focus:outline-none"
                                        }
                                    }
                                }

                                div {
                                    label { class: "block text-[11px] text-zinc-400 mb-1", "이메일 주소 *" }
                                    input {
                                        r#type: "email",
                                        placeholder: "contact@company.com",
                                        value: "{email}",
                                        oninput: move |e| email.set(e.value()),
                                        class: "w-full px-3 py-2 rounded bg-white/[0.03] border border-white/10 text-white text-xs focus:border-[#FF5722] focus:outline-none"
                                    }
                                }

                                div {
                                    label { class: "block text-[11px] text-zinc-400 mb-1", "프로젝트 개요 & 목표 납기" }
                                    textarea {
                                        rows: 3,
                                        placeholder: "배터리 기반 초소형 IoT 센서 하드웨어 시제품 제작 희망. 디스플레이 구동 및 3D 하우징 포함 4주 목표.",
                                        value: "{description}",
                                        oninput: move |e| description.set(e.value()),
                                        class: "w-full px-3 py-2 rounded bg-white/[0.03] border border-white/10 text-white text-xs focus:border-[#FF5722] focus:outline-none"
                                    }
                                }

                                // Submit Button
                                button {
                                    r#type: "button",
                                    onclick: handle_submit,
                                    class: "w-full py-3.5 rounded bg-[#FF5722] hover:bg-[#F4511E] text-white font-bold text-xs tracking-wider uppercase transition-all shadow-md active:scale-[0.99]",
                                    "기술 타당성 검토 요청 →"
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
