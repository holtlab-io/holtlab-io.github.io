//! Holt Lab — Official Studio Landing Page
//!
//! Built with Dioxus 0.7 WebAssembly & Tailwind CSS

use dioxus::prelude::*;

mod components;
mod views;

use views::Home;

const MAIN_CSS: Asset = asset!("assets/tailwind.css");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        document::Link { rel: "stylesheet", href: "https://cdn.jsdelivr.net/gh/orioncactus/pretendard/dist/web/static/pretendard.css" }
        document::Link { rel: "icon", href: "assets/branding/avatar.jpg" }
        document::Meta { name: "viewport", content: "width=device-width, initial-scale=1.0" }
        document::Meta { name: "description", content: "Holt Lab — Boutique Hardware, MCAD, ECAD & Firmware Engineering Studio. Zero-Pingpong Turnkey Development." }
        document::Meta { property: "og:title", content: "Holt Lab — From Silicon to Screen" }
        document::Meta { property: "og:description", content: "외주 핑퐁 없는 1인 턴키 하드웨어 랩. 기구·회로·펌웨어 원스톱 실물 완성." }
        document::Meta { property: "og:image", content: "assets/branding/banner.png" }

        Home {}
    }
}
