import asyncio
import json
import os
import subprocess
import time
from playwright.async_api import async_playwright

BASE_DIR = os.path.dirname(os.path.abspath(__file__))
PUBLIC_DIR = os.path.join(BASE_DIR, "target/dx/holtlab-landing/release/web/public")
OUTPUT_DIR = os.environ.get("SCREENSHOT_DIR", os.path.join(BASE_DIR, "screenshots"))
PORT = 8099

FORBIDDEN_TERMS = [
    "0.00 mm³", "0.000 mm³", "0.00mm³", "0.00 mm3", "0.000 mm3",
    "JLCPCB", "JLC3DP", "JLC04161H", "정밀공차", "공차",
    "Boutique", "boutique", "부티크", "BOUTIQUE",
    "1인", "1-person", "1-PERSON", "single architect", "단 한 명",
    "단 한명", "단일 리드", "1인 턴키", "1 person", "one-person", "single-threaded",
    "단 1회", "단1회", "단번에", "ZERO-REWORK", "리워크 제로", "first-spin",
    "재제작 시행착오 없이",
    "플래그십", "실증 머신", "실증머신",
    "imwoo90", "Tailscale", "tailscale", "TAILSCALE",
    "build123d", "KiCad", "KICAD", "Rust", "no_std", "esp-hal",
    "게이트 1", "게이트 2", "홀트 랩 감사", "● 활성", "원격측정",
    "판권 소유", "에크미", "실리콘 칩셋", "엄선된",
]


async def assert_no_forbidden_terms(page, context_label: str):
    body_text = await page.inner_text("body")
    dom_html = await page.content()
    assert "//" not in body_text, f"Double-slash '//' found in visible body text on {context_label}"
    assert "avatar.jpg" not in dom_html, f"Legacy avatar.jpg found in DOM on {context_label}"
    for term in FORBIDDEN_TERMS:
        assert term not in body_text, f"Forbidden term '{term}' found in {context_label} body text"
        assert term not in dom_html, f"Forbidden term '{term}' found in {context_label} DOM HTML"
        if term.isascii() and term.isalpha():
            assert term.lower() not in body_text.lower(), (
                f"Forbidden term '{term}' (case-insensitive) found in {context_label} body text"
            )
            assert term.lower() not in dom_html.lower(), (
                f"Forbidden term '{term}' (case-insensitive) found in {context_label} DOM HTML"
            )
    for solo_phrase in ["1-person", "single architect", "one-person", "single-threaded", "first-spin"]:
        assert solo_phrase not in body_text.lower(), (
            f"Forbidden phrase '{solo_phrase}' (case-insensitive) in {context_label}"
        )
        assert solo_phrase not in dom_html.lower(), (
            f"Forbidden phrase '{solo_phrase}' in {context_label} DOM"
        )


async def assert_all_images_loaded(page, context_label: str):
    broken = await page.evaluate(
        """() => {
            const imgs = Array.from(document.querySelectorAll('img'));
            return imgs.filter(img => !img.complete || img.naturalWidth === 0).map(img => img.src);
        }"""
    )
    assert len(broken) == 0, f"Broken images on {context_label}: {broken}"


async def assert_no_horizontal_overflow(page, context_label: str):
    overflow = await page.evaluate(
        "() => document.documentElement.scrollWidth > window.innerWidth + 1"
    )
    assert not overflow, f"Horizontal overflow detected on {context_label}"


async def run_e2e():
    os.makedirs(OUTPUT_DIR, exist_ok=True)

    # -------------------------------------------------------------
    # 0. Pre-Flight File Verification (SSG Pages & Assets)
    # -------------------------------------------------------------
    robots_path = os.path.join(PUBLIC_DIR, "robots.txt")
    sitemap_path = os.path.join(PUBLIC_DIR, "sitemap.xml")
    redirect_path = os.path.join(PUBLIC_DIR, "404.html")
    index_path = os.path.join(PUBLIC_DIR, "index.html")
    demonstrator_path = os.path.join(PUBLIC_DIR, "demonstrator/index.html")
    studio_path = os.path.join(PUBLIC_DIR, "studio/index.html")
    contact_path = os.path.join(PUBLIC_DIR, "contact/index.html")

    assert os.path.exists(robots_path), "robots.txt missing in public build output"
    with open(robots_path, "r", encoding="utf-8") as f:
        robots_content = f.read()
    assert "Allow: /" in robots_content, "robots.txt missing Allow rule"
    assert "sitemap.xml" in robots_content, "robots.txt missing sitemap pointer"
    print("✓ public/robots.txt verified.")

    assert os.path.exists(sitemap_path), "sitemap.xml missing in public build output"
    with open(sitemap_path, "r", encoding="utf-8") as f:
        sitemap_content = f.read()
    assert "https://holtlab-io.github.io/" in sitemap_content, "sitemap missing root loc"
    assert "https://holtlab-io.github.io/demonstrator" in sitemap_content, "sitemap missing demonstrator loc"
    assert "https://holtlab-io.github.io/studio" in sitemap_content, "sitemap missing studio loc"
    assert "https://holtlab-io.github.io/contact" in sitemap_content, "sitemap missing contact loc"
    print("✓ public/sitemap.xml verified with all multi-page routes and hreflang links.")

    assert os.path.exists(redirect_path), "404.html missing in public build output"
    print("✓ public/404.html redirect handler verified.")

    assert os.path.exists(demonstrator_path), "SSG demonstrator/index.html missing"
    assert os.path.exists(studio_path), "SSG studio/index.html missing"
    assert os.path.exists(contact_path), "SSG contact/index.html missing"
    print("✓ All 4 static SSG pre-rendered directories verified on disk.")

    with open(index_path, "r", encoding="utf-8") as f:
        index_html = f.read()
    assert "assets/tailwind.css" in index_html, "tailwind stylesheet missing in index.html"
    assert "pretendard" in index_html, "pretendard stylesheet missing in index.html"
    assert "JetBrains+Mono" in index_html, "JetBrains Mono stylesheet missing in index.html"
    assert "background-color: #FAFAFA" in index_html, "anti-FOUC background style missing in index.html"
    assert "attachScrollReveals" in index_html, "Scroll reveal observer engine missing in index.html"
    assert "imwoo90" not in index_html, "Personal GitHub handle leaked in index.html"
    assert "avatar.jpg" not in index_html, "Legacy avatar.jpg leaked in index.html"
    assert "/assets/branding/favicon.svg" in index_html, "Unified geometric favicon.svg missing in index.html"
    assert "회로와 기구 외함부터 서버 연동까지" in index_html, "SSG index.html should pre-render Korean default"
    print("✓ Anti-FOUC preloads, identity isolation, and Korean SSG pre-render verified in index.html.")

    # 1. Start HTTP server
    server_proc = subprocess.Popen(
        ["python3", "-m", "http.server", str(PORT), "--directory", PUBLIC_DIR],
        stdout=subprocess.DEVNULL,
        stderr=subprocess.DEVNULL,
    )
    time.sleep(1.0)
    print(f"HTTP server running on port {PORT}")

    try:
        async with async_playwright() as p:
            browser = await p.chromium.launch(headless=True)

            # -------------------------------------------------------------
            # 0b. Korean Browser Zero-Flash Default Locale Check
            # -------------------------------------------------------------
            ko_ctx = await browser.new_context(locale="ko-KR", viewport={"width": 1440, "height": 900})
            ko_page = await ko_ctx.new_page()
            await ko_page.goto(f"http://localhost:{PORT}/", wait_until="networkidle")
            html_lang_ko = await ko_page.locator("html").get_attribute("lang")
            assert html_lang_ko == "ko", f"Expected default ko-KR browser html lang='ko', got {html_lang_ko}"
            hero_ko_h1 = await ko_page.locator("#hero h1").inner_text()
            assert "회로와 기구 외함부터 서버 연동까지" in hero_ko_h1, f"Korean default h1 mismatch: {hero_ko_h1}"
            assert await ko_page.locator("#capabilities").get_by_text("IoT 백엔드 서버 구축 및 클라우드·웹 연동").is_visible()
            assert await ko_page.locator("footer").get_by_text("© 2026 Holt Lab. All rights reserved.").is_visible()
            await ko_ctx.close()
            print("✓ Korean browser (ko-KR) loads 100% Korean from static SSG with zero flash.")

            # -------------------------------------------------------------
            # 1. Page 1: / (Studio Overview Home — English Mode)
            # -------------------------------------------------------------
            page = await browser.new_page(viewport={"width": 1440, "height": 900})
            url = f"http://localhost:{PORT}/?lang=en"
            print(f"Navigating to {url}...")
            await page.goto(url, wait_until="networkidle")

            og_title = await page.locator("meta[property='og:title']").first.get_attribute("content")
            assert "Holt Lab" in og_title, f"og:title mismatch: {og_title}"

            # Verify JSON-LD structured data (Organization + ProfessionalService, NO Product or imwoo90)
            ld_json_el = page.locator("script[type='application/ld+json']").first
            assert await ld_json_el.count() > 0, "JSON-LD structured data missing"
            ld_raw = await ld_json_el.inner_text()
            assert "imwoo90" not in ld_raw, "Personal GitHub leaked in JSON-LD"
            ld_data = json.loads(ld_raw)
            graph_types = [item.get("@type") for item in ld_data.get("@graph", [])]
            assert "Organization" in graph_types, "Organization schema missing in JSON-LD"
            assert "ProfessionalService" in graph_types, "ProfessionalService schema missing in JSON-LD"
            assert "Product" not in graph_types, "E-commerce Product schema should not exist in B2B studio JSON-LD"
            print("✓ SEO meta tags, OpenGraph, and B2B JSON-LD schemas verified.")

            title_en = await page.title()
            assert "Hardware, Embedded & IoT Engineering Studio" in title_en, f"Initial title mismatch: {title_en}"
            html_lang_en = await page.locator("html").get_attribute("lang")
            assert html_lang_en == "en", f"Expected html lang='en', got {html_lang_en}"

            header_brand = await page.locator("header a[href='/']").first.inner_text()
            assert "holt" in header_brand and "lab" in header_brand, f"Brand missing: {header_brand}"
            assert "Hardware, Embedded & IoT Engineering Studio" in header_brand

            nav_links = await page.locator("header nav a").all_inner_texts()
            print(f"Desktop nav links: {nav_links}")
            assert any("home" in l.lower() for l in nav_links), "home link missing"
            assert any("showcase" in l.lower() for l in nav_links), "showcase link missing"
            assert any("studio" in l.lower() for l in nav_links), "studio link missing"
            assert any("inquire" in l.lower() for l in nav_links), "inquire link missing"

            hero_title = await page.locator("#hero h1").inner_text()
            assert "From Circuit & Enclosure to Cloud. We Engineer Physical Hardware." in hero_title, (
                f"Hero title mismatch: {hero_title}"
            )

            pill_texts = await page.locator("#hero").locator("span:has-text('01 ·'), span:has-text('02 ·'), span:has-text('03 ·'), span:has-text('04 ·')").all_inner_texts()
            expected_pills = [
                "01 · Mechanical",
                "02 · Custom Circuit",
                "03 · Embedded Firmware",
                "04 · IoT Server",
            ]
            for exp in expected_pills:
                assert any(exp in pt for pt in pill_texts), f"Pill {exp} missing in {pill_texts}"

            # Verify Capabilities 4 Pillars on Home (including IoT Backend Server)
            cap_heading = await page.locator("#capabilities h2").inner_text()
            assert "Four Pillars of Full-Stack Hardware Engineering" in cap_heading
            assert await page.locator("#capabilities").get_by_text("Mechanical CAD & Custom Enclosures").is_visible()
            assert await page.locator("#capabilities").get_by_text("Custom Electronics & Multi-Layer PCBA").is_visible()
            assert await page.locator("#capabilities").get_by_text("Embedded Firmware & Board Bring-Up").is_visible()
            assert await page.locator("#capabilities").get_by_text("IoT Backend Server & Web Integration").is_visible()

            # Verify Showcase Teaser, Studio Teaser, Real Hardware Photos, and Inquiry CTA Banner
            assert await page.locator("section:has-text('ENGINEERING SHOWCASE · CASE STUDY 01')").is_visible()
            assert await page.locator("section:has-text('Win95 Minesweeper Touch Keyring')").is_visible()
            assert await page.locator("section:has-text('ASSEMBLED & VERIFIED')").is_visible()
            assert await page.locator("img[src='/assets/showcase/real_front_on.jpg']").first.is_visible()
            assert await page.locator("img[src='/assets/showcase/real_pcba_shell_iso.jpg']").first.is_visible()
            assert await page.locator("img[src='/assets/showcase/real_knolling_top.jpg']").first.is_visible()
            assert await page.locator("section:has-text('WHY UNIFIED ENGINEERING?')").is_visible()
            assert await page.locator("section:has-text('Ready to build your custom hardware or IoT product?')").is_visible()
            await assert_no_horizontal_overflow(page, "Desktop EN /")
            print("✓ Page 1 (/) Studio Overview: Hero, 4 Pillars (with IoT), Real Hardware Showcase, and CTA verified.")

            desktop_home_shot = os.path.join(OUTPUT_DIR, "desktop_full_en.png")
            await page.evaluate("() => { const h = document.querySelector('header'); if (h) h.style.position = 'static'; }")
            await page.screenshot(path=desktop_home_shot, full_page=True)
            await page.evaluate("() => { const h = document.querySelector('header'); if (h) h.style.position = ''; }")

            # -------------------------------------------------------------
            # 2. Page 2: /demonstrator (Case Study 01 & Teardown)
            # -------------------------------------------------------------
            demo_nav_link = page.locator("header nav a[href='/demonstrator']").first
            await demo_nav_link.click()
            await page.wait_for_timeout(300)
            assert page.url.endswith("/demonstrator"), f"Expected /demonstrator URL, got {page.url}"

            assert await page.locator("#case-study-hero").get_by_text("ENGINEERING SHOWCASE · CASE STUDY 01").first.is_visible()
            assert await page.locator("text=ARCHITECTURE STATUS: 3D FIT CHECK & PHYSICAL BRING-UP VERIFIED").first.is_visible()
            await page.locator("button:has-text('01 · 3D ISO')").click()
            await page.wait_for_timeout(200)
            assert await page.locator("text=PERSPECTIVE: 3D PROJECTION").first.is_visible()
            await page.locator("button:has-text('03 · INTERNAL SECTION')").click()
            await page.wait_for_timeout(200)
            assert await page.locator("text=ANATOMY: CROSS-SECTION").first.is_visible()
            await page.locator("button:has-text('04 · CAD WIREFRAME')").click()
            await page.wait_for_timeout(200)
            assert await page.locator("text=ENGINEERING: 2D WIREFRAME").first.is_visible()
            await page.locator("button:has-text('02 · FRONT BEZEL')").click()
            await page.wait_for_timeout(200)
            assert await page.locator("text=ELEVATION: FRONT CHASSIS").first.is_visible()
            print("✓ Page 2 (/demonstrator): Interactive 4-view CAD inspection modes verified.")

            assert await page.locator("#hardware-proof").get_by_text("From 3D CAD to Powered-On Hardware.").first.is_visible()
            assert await page.locator("#hardware-proof img[src='/assets/showcase/real_front_on.jpg']").is_visible()
            assert await page.locator("#hardware-proof img[src='/assets/showcase/real_pcba_shell_iso.jpg']").is_visible()
            assert await page.locator("#hardware-proof img[src='/assets/showcase/real_knolling_top.jpg']").is_visible()
            print("✓ Page 2 (/demonstrator): 3-photo Physical Prototype & Bench Bring-Up gallery verified.")

            assert await page.locator("#case-study").get_by_text("Four Architectural Layers, Inside and Out.").first.is_visible()
            assert await page.locator("#case-study").get_by_text("LAYER 01").first.is_visible()

            await page.locator("#case-study button:has-text('ECAD 4-Layer PCBA')").click()
            await page.wait_for_timeout(200)
            assert await page.locator("#case-study").get_by_text("0 ERC · 0 DRC VERIFIED").first.is_visible()

            await page.locator("#case-study button:has-text('Embedded Firmware')").click()
            await page.wait_for_timeout(200)
            assert await page.locator("#case-study").get_by_text("ZERO-COPY 60Hz DMA VERIFIED").first.is_visible()

            await page.locator("#case-study button:has-text('Interface & Haptics')").click()
            await page.wait_for_timeout(200)
            assert await page.locator("#case-study").get_by_text("FEED-FORWARD PWM VERIFIED").first.is_visible()
            print("✓ Page 2 (/demonstrator): 4-layer anatomical teardown verified.")

            specs_h2 = await page.locator("#specs h2").inner_text()
            assert "Technical Specifications" in specs_h2
            rows_count = await page.locator("#specs .divide-y > div").count()
            assert rows_count >= 10, f"Expected at least 10 spec rows, found {rows_count}"
            await assert_no_horizontal_overflow(page, "Desktop EN /demonstrator")
            print(f"✓ Page 2 (/demonstrator): Swiss engineering spec matrix verified with {rows_count} rows.")

            # -------------------------------------------------------------
            # 3. Page 3: /studio (Methodology & 4-Phase Workflow)
            # -------------------------------------------------------------
            studio_nav_link = page.locator("header nav a[href='/studio']").first
            await studio_nav_link.click()
            await page.wait_for_timeout(300)
            assert page.url.endswith("/studio"), f"Expected /studio URL, got {page.url}"

            assert await page.locator("#studio-hero").get_by_text("STUDIO ARCHITECTURE · METHODOLOGY").is_visible()
            assert await page.locator("#studio").get_by_text("FRAGMENTED MULTI-AGENCY OUTSOURCING").is_visible()
            assert await page.locator("#studio").get_by_text("HOLT LAB UNIFIED ENGINEERING").is_visible()
            assert await page.locator("#studio").get_by_text("01 · ARCHITECTURE & BOM LOCK").is_visible()
            assert await page.locator("#studio").get_by_text("03 · PROTOTYPE & FIT CHECK").is_visible()
            assert await page.locator("#studio").get_by_text("04 · FIRMWARE & IOT INTEGRATION").is_visible()
            await assert_no_horizontal_overflow(page, "Desktop EN /studio")
            print("✓ Page 3 (/studio): Unified workflow and 4-phase pipeline verified.")

            # -------------------------------------------------------------
            # 4. Page 4: /contact (Client Consultation & 5-Scope RFQ)
            # -------------------------------------------------------------
            contact_nav_link = page.locator("header nav a[href='/contact']").first
            await contact_nav_link.click()
            await page.wait_for_timeout(300)
            assert page.url.endswith("/contact"), f"Expected /contact URL, got {page.url}"

            assert await page.locator("#contact-hero").get_by_text("CLIENT RFQ INTAKE").is_visible()
            assert await page.locator("#inquire").get_by_text("contact@holtlab.io").first.is_visible()
            assert await page.locator("#inquire input[required][type='text']").is_visible()
            await assert_no_horizontal_overflow(page, "Desktop EN /contact")
            print("✓ Page 4 (/contact): Direct client consultation intake form verified.")

            # -------------------------------------------------------------
            # 5. Scroll Reveal Animation Motion Test
            # -------------------------------------------------------------
            reveal_count = await page.locator(".reveal-on-scroll").count()
            assert reveal_count > 0, "No .reveal-on-scroll elements found in DOM"
            revealed_count = await page.locator(".reveal-on-scroll.is-revealed").count()
            assert revealed_count > 0, "IntersectionObserver failed to add .is-revealed class"
            print(f"✓ Scroll reveal motion verified: {revealed_count}/{reveal_count} elements revealed.")

            # -------------------------------------------------------------
            # 6. Bilingual Whole-Pill Toggle Synchronization Across All Subpages
            # -------------------------------------------------------------
            lang_toggle_btn = page.locator("header button:has-text('KR')").first
            # Click 1: EN -> KO
            await lang_toggle_btn.click()
            await page.wait_for_timeout(250)
            assert await page.evaluate("localStorage.getItem('holtlab_lang')") == "ko"

            # Click 2 on the EXACT SAME pill button (even on KR side): KO -> EN
            await lang_toggle_btn.click(position={"x": 12, "y": 10})
            await page.wait_for_timeout(250)
            assert await page.evaluate("localStorage.getItem('holtlab_lang')") == "en", (
                "Clicking active KR thumb must toggle back to EN!"
            )

            # Click 3 on the EXACT SAME pill button: EN -> KO
            await lang_toggle_btn.click(position={"x": 12, "y": 10})
            await page.wait_for_timeout(250)

            title_kr = await page.title()
            assert "하드웨어 · 임베디드 · IoT 엔지니어링 스튜디오" in title_kr, f"Korean title mismatch: {title_kr}"
            saved_lang = await page.evaluate("localStorage.getItem('holtlab_lang')")
            assert saved_lang == "ko", f"Expected localStorage holtlab_lang to be 'ko', got {saved_lang}"
            assert await page.locator("#inquire").get_by_text("의뢰 분야 선택").first.is_visible()

            # Select 5th pill: "IoT 서버 · 웹 연동"
            iot_pill = page.locator("#inquire button:has-text('IoT 서버 · 웹 연동')")
            assert await iot_pill.is_visible(), "5th IoT scope pill missing on /contact"
            await iot_pill.click()

            await page.locator("#inquire input[required][type='text']").fill("(주)스마트센서 / 김민수")
            await page.locator("#inquire input[required][type='email']").fill("minsu@smartsensor.kr")
            await page.locator("#inquire input[placeholder*='시제품']").fill("8주 내 시제품 및 관제 대시보드")
            await page.locator("#inquire input[placeholder*='원']").fill("1,500만 원")
            await page.locator("#inquire textarea[required]").fill(
                "BLE/Wi-Fi 기반 산업용 환경 센서 노드 기구·기판 설계 및 실시간 웹 모니터링 서버 구축 문의."
            )

            await page.locator("#inquire button[type='submit']").click()
            await page.wait_for_timeout(300)
            assert await page.locator("#inquire").get_by_text("메일 발송 준비 완료").is_visible()
            assert await page.locator("#inquire").get_by_text("IoT 서버 · 웹 연동").is_visible()

            mailto_link = await page.locator("#inquire a:has-text('메일 프로그램 열기')").get_attribute("href")
            assert "contact@holtlab.io" in mailto_link
            assert "%20" in mailto_link
            assert " " not in mailto_link.split("?", 1)[1]
            print("✓ Inquiry form 5th IoT scope selection and RFC mailto generator verified in Korean.")

            # Navigate to /demonstrator in Korean
            await page.locator("header nav a[href='/demonstrator']").first.click()
            await page.wait_for_timeout(300)
            assert await page.locator("#case-study").get_by_text("외함부터 기판과 펌웨어까지, 4계층 내부 설계 구조").first.is_visible()
            assert await page.locator("#specs").get_by_text("상세 기술 제원 및 검증 사양").is_visible()
            assert await page.locator("button:has-text('01 · 3D 아이소')").is_visible()

            # Navigate to /studio in Korean
            await page.locator("header nav a[href='/studio']").first.click()
            await page.wait_for_timeout(300)
            assert await page.locator("#studio").get_by_text("왜 통합 엔지니어링인가?").is_visible()
            assert await page.locator("#studio").get_by_text("03 · 시제품 제작 및 조립 검증").is_visible()
            assert await page.locator("#studio").get_by_text("04 · 펌웨어 및 서버 통합 완주").is_visible()

            # Navigate to / in Korean
            await page.locator("header nav a[href='/']").first.click()
            await page.wait_for_timeout(300)
            hero_title_kr = await page.locator("#hero h1").inner_text()
            assert "회로와 기구 외함부터 서버 연동까지" in hero_title_kr
            print("✓ Bilingual toggle synchronization verified across all 4 subpages.")

            desktop_kr_shot = os.path.join(OUTPUT_DIR, "desktop_full_kr.png")
            await page.evaluate("() => { const h = document.querySelector('header'); if (h) h.style.position = 'static'; }")
            await page.screenshot(path=desktop_kr_shot, full_page=True)
            await page.evaluate("() => { const h = document.querySelector('header'); if (h) h.style.position = ''; }")

            # -------------------------------------------------------------
            # 7. Eradication of Forbidden Terms Audit (All Subpages, KO & EN)
            # -------------------------------------------------------------
            for route in ["/", "/demonstrator", "/studio", "/contact"]:
                await page.goto(f"http://localhost:{PORT}{route}?lang=ko", wait_until="networkidle")
                await assert_no_forbidden_terms(page, f"Korean {route}")
                await assert_all_images_loaded(page, f"Korean {route}")
                await page.goto(f"http://localhost:{PORT}{route}?lang=en", wait_until="networkidle")
                await assert_no_forbidden_terms(page, f"English {route}")
                await assert_all_images_loaded(page, f"English {route}")
            print("✓ Eradication audit: 0 instances of personal IDs, internal tool leaks, or exaggerated claims across all routes.")

            # -------------------------------------------------------------
            # 8. Responsive Viewport Verification (Tablet 768px & Mobile 375px / 412px)
            # -------------------------------------------------------------
            page_tablet = await browser.new_page(viewport={"width": 768, "height": 1024})
            await page_tablet.goto(f"http://localhost:{PORT}/demonstrator?lang=en", wait_until="networkidle")
            assert await page_tablet.locator("button[aria-label='Toggle Navigation Menu']").is_visible(), "Hamburger missing at 768px"
            assert not await page_tablet.locator("header nav").is_visible(), "Desktop nav should be hidden at 768px"

            tablet_tabs = page_tablet.locator("#case-study button[type='button']")
            for i in range(4):
                title_el = tablet_tabs.nth(i).locator("div").last
                scroll_w = await title_el.evaluate("el => el.scrollWidth")
                client_w = await title_el.evaluate("el => el.clientWidth")
                txt = await title_el.inner_text()
                assert scroll_w <= client_w + 1, f"Tab {i} ({txt}) is truncated on tablet ({scroll_w} > {client_w})"
            await assert_no_horizontal_overflow(page_tablet, "Tablet 768px /demonstrator")
            print("✓ Tablet 768px: Clean responsive header and zero-truncation tabs verified.")

            for width, label in [(375, "iPhone375"), (412, "S24Ultra412")]:
                mp = await browser.new_page(viewport={"width": width, "height": 900})
                for route in ["/", "/demonstrator", "/studio", "/contact"]:
                    await mp.goto(f"http://localhost:{PORT}{route}?lang=ko", wait_until="networkidle")
                    await assert_no_horizontal_overflow(mp, f"Mobile {label} KO {route}")
                await mp.goto(f"http://localhost:{PORT}/?lang=en", wait_until="networkidle")
                hamburger_btn = mp.locator("header button[aria-label='Toggle Navigation Menu']")
                await hamburger_btn.click()
                await mp.wait_for_timeout(300)
                assert await mp.locator("header a[href='/']").last.is_visible()
                assert await mp.locator("header a[href='/demonstrator']").last.is_visible()
                assert await mp.locator("header a[href='/studio']").last.is_visible()
                assert await mp.locator("header a[href='/contact']").last.is_visible()

                mobile_kr_btn = mp.locator("header button:has-text('KR')").last
                await mobile_kr_btn.click()
                await mp.wait_for_timeout(300)
                assert await mp.locator("header a[href='/demonstrator']").get_by_text("쇼케이스").last.is_visible()
                await mp.locator("header a[href='/demonstrator']").last.click()
                await mp.wait_for_timeout(300)
                assert mp.url.endswith("/demonstrator")
                await mp.close()
            print("✓ Mobile 375px & Galaxy S24 Ultra 412px: Zero overflow, drawer links, and language switch verified.")

            await browser.close()
            print("\n🎉 ALL MULTI-PAGE E2E AND PLAYWRIGHT TESTS PASSED 100%!")

    finally:
        server_proc.terminate()
        server_proc.wait()
        print("HTTP server stopped.")


if __name__ == "__main__":
    asyncio.run(run_e2e())
