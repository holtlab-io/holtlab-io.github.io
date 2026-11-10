//! ## Overview
//! Document head metadata component rendering SEO tags, OpenGraph schemas, fonts, and inline engines.
//! Uses Dioxus document primitives to ensure complete SSR and SSG head hydration.
//!
//! ## Search Tags
//! #head, #seo, #metadata, #opengraph, #scripts, #document
//!
//! ## Submodules
//! None. This module provides atomic document head components.

use dioxus::prelude::*;
use crate::i18n::Language;

const JSON_LD_ORGANIZATION: &str = r#"{"@context":"https://schema.org","@graph":[{"@type":"Organization","@id":"https://holtlab-io.github.io/#organization","name":"Holt Lab","url":"https://holtlab-io.github.io/","logo":"https://holtlab-io.github.io/assets/branding/avatar.jpg","description":"Hardware, Mechanical & Embedded Engineering Studio specializing in unified turnkey hardware execution.","founder":{"@type":"Person","name":"Wim","url":"https://github.com/imwoo90"},"sameAs":["https://github.com/holtlab-io","https://youtube.com/@holt-lab"]},{"@type":"Product","@id":"https://holtlab-io.github.io/#opus01","name":"OPUS 01 // Win95 Minesweeper Touch Keyring","image":"https://holtlab-io.github.io/assets/showcase/mcad_iso_front.png","description":"Ultra-compact interactive pocket demonstrator integrating ESP32-C3, ST7789T3 2.0\" 60Hz IPS, CST816D capacitive touch, and real-time embedded firmware in a 43.4×69.2×13.78mm HP MJF PA12 enclosure.","brand":{"@type":"Brand","name":"Holt Lab"},"category":"Electronics > Custom Hardware","offers":{"@type":"Offer","availability":"https://schema.org/InStock","priceCurrency":"USD","price":"0","description":"Hardware demonstrator / bespoke engineering showcase"}}]}"#;

const SCROLL_REVEAL_ENGINE: &str = r#"
(function() {
  if (typeof window === 'undefined') return;
  function initScrollReveals() {
    function checkViewport() {
      var targets = document.querySelectorAll('.reveal-on-scroll:not(.is-revealed)');
      var vh = window.innerHeight || document.documentElement.clientHeight;
      for (var i = 0; i < targets.length; i++) {
        var rect = targets[i].getBoundingClientRect();
        if (rect.top <= vh - 40 && rect.bottom >= 0) {
          targets[i].classList.add('is-revealed');
        }
      }
    }

    if (!('IntersectionObserver' in window)) {
      var all = document.querySelectorAll('.reveal-on-scroll');
      for (var i = 0; i < all.length; i++) { all[i].classList.add('is-revealed'); }
      return;
    }

    var observer = new IntersectionObserver(function(entries) {
      for (var i = 0; i < entries.length; i++) {
        var entry = entries[i];
        if (entry.isIntersecting) {
          entry.target.classList.add('is-revealed');
        }
      }
    }, { rootMargin: '0px 0px -40px 0px', threshold: 0.08 });

    function attach() {
      var targets = document.querySelectorAll('.reveal-on-scroll:not(.is-revealed)');
      for (var i = 0; i < targets.length; i++) { observer.observe(targets[i]); }
      checkViewport();
    }

    window.attachScrollReveals = attach;
    attach();

    window.addEventListener('scroll', checkViewport, { passive: true });
    window.addEventListener('resize', checkViewport, { passive: true });

    if ('MutationObserver' in window) {
      var isMutating = false;
      var mo = new MutationObserver(function() {
        if (isMutating) return;
        isMutating = true;
        attach();
        setTimeout(function() { isMutating = false; }, 50);
      });
      mo.observe(document.documentElement, {
        childList: true,
        subtree: true,
        attributes: true,
        attributeFilter: ['class']
      });
    }
  }
  if (document.readyState === 'loading') {
    document.addEventListener('DOMContentLoaded', initScrollReveals);
  } else {
    initScrollReveals();
  }
})();
"#;

const SPA_REDIRECT_ENGINE: &str = r#"
(function(l) {
  if (l.search[1] === '/' ) {
    var decoded = l.search.slice(1).split('&').map(function(s) { 
      return s.replace(/~and~/g, '&')
    }).join('?');
    window.history.replaceState(null, null,
        l.pathname.slice(0, -1) + decoded + l.hash
    );
  }
}(window.location))
"#;

/// Injects font links, stylesheets, and inline runtime scripts.
#[component]
fn HeadResources() -> Element {
    rsx! {
        document::Link { rel: "preconnect", href: "https://fonts.googleapis.com" }
        document::Link { rel: "preconnect", href: "https://fonts.gstatic.com", crossorigin: "true" }
        document::Link { rel: "preconnect", href: "https://cdn.jsdelivr.net" }

        document::Link { rel: "stylesheet", href: "/assets/tailwind.css" }
        document::Link { rel: "stylesheet", href: "https://fonts.googleapis.com/css2?family=JetBrains+Mono:wght@400;500;600;700&display=swap" }
        document::Link { rel: "stylesheet", href: "https://cdn.jsdelivr.net/gh/orioncactus/pretendard/dist/web/static/pretendard.css" }

        document::Style {
            r#"
            html, body {{
                margin: 0;
                padding: 0;
                background-color: #FAFAFA;
                color: #09090B;
                font-family: 'Pretendard', -apple-system, BlinkMacSystemFont, system-ui, Roboto, sans-serif;
                -webkit-font-smoothing: antialiased;
                word-break: keep-all;
            }}
            "#
        }

        document::Script { r#type: "application/ld+json", "{JSON_LD_ORGANIZATION}" }
        document::Script { "{SPA_REDIRECT_ENGINE}" }
        document::Script { "{SCROLL_REVEAL_ENGINE}" }
    }
}

/// Injects global document title, metadata, styles, preloads, and scripts into document head.
#[component]
pub fn HeadMetadata() -> Element {
    let lang = use_context::<Signal<Language>>();

    use_effect(move || {
        #[cfg(target_arch = "wasm32")]
        {
            let current_lang = lang();
            if let Some(window) = web_sys::window() {
                if let Some(doc) = window.document() {
                    if let Some(html_el) = doc.document_element() {
                        let code = match current_lang {
                            Language::En => "en",
                            Language::Ko => "ko",
                        };
                        let _ = html_el.set_attribute("lang", code);
                    }
                }
            }
        }
        #[cfg(not(target_arch = "wasm32"))]
        {
            let _ = lang();
        }
    });

    let title = match lang() {
        Language::En => "Holt Lab — Hardware & Embedded Engineering Studio",
        Language::Ko => "Holt Lab — 하드웨어 & 임베디드 엔지니어링 랩",
    };
    let desc = match lang() {
        Language::En => "Holt Lab — Hardware, Mechanical & Embedded Engineering Studio. Unified Turnkey Hardware & Embedded Execution.",
        Language::Ko => "Holt Lab — 하드웨어, 기구 및 임베디드 엔지니어링 랩. 통합 턴키 하드웨어 개발.",
    };

    rsx! {
        document::Title { "{title}" }
        document::Meta { name: "description", content: "{desc}" }
        document::Link { rel: "icon", r#type: "image/jpeg", href: "/assets/branding/avatar.jpg" }
        document::Link { rel: "canonical", href: "https://holtlab-io.github.io/" }
        document::Link { rel: "alternate", hreflang: "ko", href: "https://holtlab-io.github.io/?lang=ko" }
        document::Link { rel: "alternate", hreflang: "en", href: "https://holtlab-io.github.io/?lang=en" }
        document::Link { rel: "alternate", hreflang: "x-default", href: "https://holtlab-io.github.io/" }

        document::Meta { property: "og:type", content: "website" }
        document::Meta { property: "og:title", content: "{title}" }
        document::Meta { property: "og:description", content: "{desc}" }
        document::Meta { property: "og:image", content: "https://holtlab-io.github.io/assets/branding/banner.png" }
        document::Meta { property: "og:url", content: "https://holtlab-io.github.io/" }

        document::Meta { name: "twitter:card", content: "summary_large_image" }
        document::Meta { name: "twitter:title", content: "{title}" }
        document::Meta { name: "twitter:description", content: "{desc}" }
        document::Meta { name: "twitter:image", content: "https://holtlab-io.github.io/assets/branding/banner.png" }

        HeadResources {}
    }
}
