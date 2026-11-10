use dioxus::prelude::*;
use crate::components::{
    navbar::Navbar,
    hero::Hero,
    metrics::Metrics,
    comparison::Comparison,
    showcase::Showcase,
    services::Services,
    process::Process,
    contact::Contact,
    footer::Footer,
};

#[component]
pub fn Home() -> Element {
    rsx! {
        div {
            class: "min-h-screen bg-[#0B0F17] text-slate-100 flex flex-col selection:bg-orange-500 selection:text-white",
            Navbar {}
            main {
                class: "flex-grow",
                Hero {}
                Metrics {}
                Comparison {}
                Showcase {}
                Services {}
                Process {}
                Contact {}
            }
            Footer {}
        }
    }
}
