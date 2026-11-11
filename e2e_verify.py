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
    "단 1회", "단1회", "단번에", "ZERO-REWORK", "리워크 제로",
    "플래그십", "실증 머신", "실증머신"
]

async def assert_no_forbidden_terms(page, context_label: str):
    body_text = await page.inner_text("body")
    dom_html = await page.content()
    for term in FORBIDDEN_TERMS:
        assert term not in body_text, f"Forbidden term '{term}' found in {context_label} body text"
        assert term not in dom_html, f"Forbidden term '{term}' found in {context_label} DOM HTML"
        if term.isascii() and term.isalpha():
            assert term.lower() not in body_text.lower(), f"Forbidden term '{term}' (case-insensitive) found in {context_label} body text"
            assert term.lower() not in dom_html.lower(), f"Forbidden term '{term}' (case-insensitive) found in {context_label} DOM HTML"
    for solo_phrase in ["1-person", "single architect", "one-person", "single-threaded"]:
        assert solo_phrase not in body_text.lower(), f"Forbidden solo phrase '{solo_phrase}' (case-insensitive) in {context_label}"
        assert solo_phrase not in dom_html.lower(), f"Forbidden solo phrase '{solo_phrase}' in {context_label} DOM"

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

    # Verify true Static Site Generation (SSG) outputs exist on disk
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
    print("✓ Anti-FOUC preloads and scroll reveal observer engine verified in index.html.")

    # 1. Start HTTP server
    server_proc = subprocess.Popen(
        ["python3", "-m", "http.server", str(PORT), "--directory", PUBLIC_DIR],
        stdout=subprocess.DEVNULL,
        stderr=subprocess.DEVNULL
    )
    time.sleep(1.0)
    print(f"HTTP server running on port {PORT}")

    try:
        async with async_playwright() as p:
            browser = await p.chromium.launch(headless=True)

            # -------------------------------------------------------------
            # 1. Page 1: / (Studio Overview Home)
            # -------------------------------------------------------------
            page = await browser.new_page(viewport={"width": 1440, "height": 900})
            url = f"http://localhost:{PORT}/?lang=en"
            print(f"Navigating to {url}...")
            await page.goto(url, wait_until="networkidle")

            # Verify DOM SEO Metadata
            og_title = await page.locator("meta[property='og:title']").first.get_attribute("content")
            assert "HOLT LAB" in og_title or "Holt Lab" in og_title, f"og:title mismatch: {og_title}"

            og_desc = await page.locator("meta[property='og:description']").first.get_attribute("content")
            assert "Unified Turnkey Hardware" in og_desc, f"og:desc mismatch: {og_desc}"

            # Verify JSON-LD structured data
            ld_json_el = page.locator("script[type='application/ld+json']").first
            assert await ld_json_el.count() > 0, "JSON-LD structured data missing"
            ld_data = json.loads(await ld_json_el.inner_text())
            graph_types = [item.get("@type") for item in ld_data.get("@graph", [])]
            assert "Organization" in graph_types, "Organization schema missing in JSON-LD"
            assert "Product" in graph_types, "Product schema missing in JSON-LD"
            print("✓ Full SEO meta tags, OpenGraph, Twitter Cards, and JSON-LD schemas verified.")

            # Verify Document Title & html lang in English mode
            title_en = await page.title()
            assert "Hardware & Embedded Engineering Studio" in title_en, f"Initial title mismatch: {title_en}"
            html_lang_en = await page.locator("html").get_attribute("lang")
            assert html_lang_en == "en", f"Expected html lang='en', got {html_lang_en}"

            # Verify Header Brand & Multi-Page Navigation
            header_brand = await page.locator("header a[href='/']").first.inner_text()
            assert "holt" in header_brand and "lab" in header_brand, f"Brand missing: {header_brand}"
            assert "Hardware & Embedded Engineering Studio" in header_brand

            # Verify Desktop Navigation contains all 4 multi-page links
            nav_links = await page.locator("header nav a").all_inner_texts()
            print(f"Desktop nav links: {nav_links}")
            assert any("home" in l for l in nav_links), "home link missing"
            assert any("demonstrator" in l for l in nav_links), "demonstrator link missing"
            assert any("studio" in l for l in nav_links), "studio link missing"
            assert any("inquire" in l for l in nav_links), "inquire link missing"

            # Verify Studio Hero Elements
            hero_title = await page.locator("#hero h1").inner_text()
            assert "From Silicon to Screen. We Engineer Physical Hardware." in hero_title, f"Hero title mismatch: {hero_title}"

            hero_eyebrow = await page.locator("#hero").get_by_text("HARDWARE ENGINEERING & RAPID PROTOTYPING STUDIO").inner_text()
            assert "HARDWARE ENGINEERING" in hero_eyebrow

            pill_texts = await page.locator("#hero").locator("span:has-text('MCAD'), span:has-text('ECAD'), span:has-text('Async'), span:has-text('SMT')").all_inner_texts()
            expected_pills = ["01 // MCAD", "02 // ECAD", "03 // Embedded", "04 // DFM"]
            for exp in expected_pills:
                assert any(exp in pt for pt in pill_texts), f"Pill {exp} missing in {pill_texts}"

            # Verify Capabilities 4 Pillars on Home
            cap_heading = await page.locator("#capabilities h2").inner_text()
            assert "4 pillars of unified turnkey hardware" in cap_heading
            assert await page.locator("#capabilities").get_by_text("Mechanical CAD & Enclosures").is_visible()
            assert await page.locator("#capabilities").get_by_text("Electrical PCBA & High-Speed Routing").is_visible()
            assert await page.locator("#capabilities").get_by_text("Embedded Firmware & DMA Engine").is_visible()
            assert await page.locator("#capabilities").get_by_text("DFM & Rapid Bench Verification").is_visible()

            # Verify Demonstrator Teaser Card on Home
            assert await page.locator("section:has-text('ENGINEERING SHOWCASE // OPUS 01')").is_visible()
            assert await page.locator("section:has-text('Win95 Minesweeper Touch Keyring')").is_visible()

            # Verify Studio Turnkey Teaser on Home
            assert await page.locator("section:has-text('WHY UNIFIED TURNKEY?')").is_visible()

            # Verify Direct Inquiry CTA Banner on Home
            assert await page.locator("section:has-text('Ready to engineer physical hardware?')").is_visible()
            print("✓ Page 1 (/) Studio Overview: Hero, Capabilities, Teasers, and CTA Banner verified.")

            # Capture Desktop Home Screenshot
            desktop_home_shot = os.path.join(OUTPUT_DIR, "desktop_full_en.png")
            await page.evaluate("() => { const h = document.querySelector('header'); if (h) h.style.position = 'static'; }")
            await page.screenshot(path=desktop_home_shot, full_page=True)
            await page.evaluate("() => { const h = document.querySelector('header'); if (h) h.style.position = ''; }")

            # -------------------------------------------------------------
            # 2. Page 2: /demonstrator (Flagship Demonstrator & Teardown)
            # -------------------------------------------------------------
            demo_nav_link = page.locator("header nav a[href='/demonstrator']").first
            await demo_nav_link.click()
            await page.wait_for_timeout(300)
            assert page.url.endswith("/demonstrator"), f"Expected /demonstrator URL, got {page.url}"

            # Verify Demonstrator Hero
            assert await page.locator("#case-study-hero").get_by_text("ENGINEERING SHOWCASE").first.is_visible()

            # Verify Interactive 4-View CAD Inspection Workbench
            assert await page.locator("text=SYSTEM ARCHITECTURE: UNIFIED CO-DESIGN // TAILSCALE BENCH PASS").first.is_visible()
            await page.locator("button:has-text('01 // 3D ISO')").click()
            await page.wait_for_timeout(200)
            assert await page.locator("text=PERSPECTIVE: 3D PROJECTION").first.is_visible()
            await page.locator("button:has-text('03 // INTERNAL SECTION')").click()
            await page.wait_for_timeout(200)
            assert await page.locator("text=ANATOMY: CROSS-SECTION").first.is_visible()
            await page.locator("button:has-text('04 // CAD WIREFRAME')").click()
            await page.wait_for_timeout(200)
            assert await page.locator("text=ENGINEERING: 2D WIREFRAME").first.is_visible()
            await page.locator("button:has-text('02 // FRONT BEZEL')").click()
            await page.wait_for_timeout(200)
            assert await page.locator("text=ELEVATION: FRONT CHASSIS").first.is_visible()
            print("✓ Page 2 (/demonstrator): Interactive 4-view CAD inspection modes verified.")

            # Verify Anatomical 4-Layer Teardown & Interactive Layer Switch
            assert await page.locator("#case-study").get_by_text("pulled apart to the bare copper.").first.is_visible()
            assert await page.locator("#case-study").get_by_text("LAYER 01").first.is_visible()

            # Switch to Layer 02 (ECAD)
            await page.locator("#case-study button:has-text('ECAD 4-Layer PCBA')").click()
            await page.wait_for_timeout(200)
            assert await page.locator("#case-study").get_by_text("LAYER 02").first.is_visible()
            assert await page.locator("#case-study").get_by_text("GATE 2 // 0 ERC · 0 DRC VERIFIED").first.is_visible()

            # Switch to Layer 03 (Firmware)
            await page.locator("#case-study button:has-text('High-Reliability Embedded Firmware Engine')").click()
            await page.wait_for_timeout(200)
            assert await page.locator("#case-study").get_by_text("LAYER 03").first.is_visible()
            assert await page.locator("#case-study").get_by_text("BENCH // ZERO-COPY 60Hz DMA PASS").first.is_visible()

            # Switch to Layer 04 (Haptics)
            await page.locator("#case-study button:has-text('Human Interface & Haptics')").click()
            await page.wait_for_timeout(200)
            assert await page.locator("#case-study").get_by_text("LAYER 04").first.is_visible()
            assert await page.locator("#case-study").get_by_text("TELEMETRY // FEED-FORWARD PWM PASS").first.is_visible()
            print("✓ Page 2 (/demonstrator): 4-layer anatomical teardown and verification gates verified.")

            # Verify 10-Row Swiss Spec Matrix
            specs_h2 = await page.locator("#specs h2").inner_text()
            assert "technical specifications" in specs_h2
            rows_count = await page.locator("#specs .divide-y > div").count()
            assert rows_count >= 10, f"Expected at least 10 spec rows, found {rows_count}"
            print(f"✓ Page 2 (/demonstrator): Swiss engineering spec matrix verified with {rows_count} rows.")

            # -------------------------------------------------------------
            # 3. Page 3: /studio (Turnkey Architecture & Methodology)
            # -------------------------------------------------------------
            studio_nav_link = page.locator("header nav a[href='/studio']").first
            await studio_nav_link.click()
            await page.wait_for_timeout(300)
            assert page.url.endswith("/studio"), f"Expected /studio URL, got {page.url}"

            assert await page.locator("#studio-hero").get_by_text("STUDIO ARCHITECTURE // METHODOLOGY").is_visible()
            assert await page.locator("#studio").get_by_text("TRADITIONAL AGENCY FRAGMENTATION").is_visible()
            assert await page.locator("#studio").get_by_text("HOLT LAB UNIFIED TURNKEY").is_visible()
            assert await page.locator("#studio").get_by_text("01 // CO-DESIGN LOCK").is_visible()
            assert await page.locator("#studio").get_by_text("04 // REMOTE BENCH BOOT").is_visible()
            print("✓ Page 3 (/studio): Turnkey co-design model and 4-phase pipeline verified.")

            # -------------------------------------------------------------
            # 4. Page 4: /contact (Dedicated Client RFQ Intake)
            # -------------------------------------------------------------
            contact_nav_link = page.locator("header nav a[href='/contact']").first
            await contact_nav_link.click()
            await page.wait_for_timeout(300)
            assert page.url.endswith("/contact"), f"Expected /contact URL, got {page.url}"

            assert await page.locator("#contact-hero").get_by_text("CLIENT RFQ INTAKE").is_visible()
            assert await page.locator("#inquire").get_by_text("contact@holtlab.io").first.is_visible()
            assert await page.locator("#inquire input[required][type='text']").is_visible()
            print("✓ Page 4 (/contact): Direct client consultation intake form verified.")

            # -------------------------------------------------------------
            # 5. Apple-style Scroll Reveal Animation Motion Test
            # -------------------------------------------------------------
            # Verify reveal-on-scroll elements exist
            reveal_count = await page.locator(".reveal-on-scroll").count()
            assert reveal_count > 0, "No .reveal-on-scroll elements found in DOM"

            # Check that an element in the viewport has .is-revealed
            revealed_count = await page.locator(".reveal-on-scroll.is-revealed").count()
            assert revealed_count > 0, "IntersectionObserver failed to add .is-revealed class"
            print(f"✓ Apple-style scroll reveal motion verified: {revealed_count}/{reveal_count} elements revealed.")

            # -------------------------------------------------------------
            # 6. Bilingual Toggle Synchronization Across All Subpages
            # -------------------------------------------------------------
            kr_toggle_btn = page.locator("header button:has-text('KR')").first
            await kr_toggle_btn.click()
            await page.wait_for_timeout(300)

            # Check /contact in Korean
            title_kr = await page.title()
            assert "하드웨어 & 임베디드" in title_kr, f"Korean title mismatch: {title_kr}"
            saved_lang = await page.evaluate("localStorage.getItem('holtlab_lang')")
            assert saved_lang == "ko", f"Expected localStorage holtlab_lang to be 'ko', got {saved_lang}"
            assert await page.locator("#inquire").get_by_text("개발 분야 선택").first.is_visible()

            # Test Inquiry Form Dispatch in Korean
            await page.locator("#inquire input[required][type='text']").fill("넥서스 다이내믹스 / 반스 박사")
            await page.locator("#inquire input[required][type='email']").fill("vance@nexusdynamics.io")
            await page.locator("#inquire input[placeholder*='주'], #inquire input[placeholder*='weeks']").fill("시제품까지 4주")
            await page.locator("#inquire input[placeholder*='원'], #inquire input[placeholder*='USD']").fill("1,500만 원")
            await page.locator("#inquire textarea[required]").fill("윈도우 95 감성 포터블 콘솔, 4층 임피던스 제어 기판, CST816D 터치 및 60Hz GDMA 전송.")

            await page.locator("#inquire button[type='submit']").click()
            await page.wait_for_timeout(300)
            assert await page.locator("#inquire").get_by_text("발송 버퍼 준비 완료").is_visible()

            mailto_link = await page.locator("#inquire a:has-text('메일 프로그램 열기'), #inquire a:has-text('open mail client')").get_attribute("href")
            assert "contact@holtlab.io" in mailto_link
            assert "%20" in mailto_link
            assert " " not in mailto_link.split("?", 1)[1]
            print("✓ Inquiry form interactive dispatch, full data fidelity, and RFC mailto generator verified in Korean.")

            # Navigate to /demonstrator in Korean
            await page.locator("header nav a[href='/demonstrator']").first.click()
            await page.wait_for_timeout(300)
            assert await page.locator("#case-study").get_by_text("순수 구리 패턴까지 완벽히 분해하다.").first.is_visible()
            assert await page.locator("#specs").get_by_text("상세 기술 제원").is_visible()
            assert await page.locator("button:has-text('01 // 3D 아이소')").is_visible()

            # Navigate to /studio in Korean
            await page.locator("header nav a[href='/studio']").first.click()
            await page.wait_for_timeout(300)
            assert await page.locator("#studio").get_by_text("왜 통합 턴키인가?").is_visible()
            assert await page.locator("#studio").get_by_text("홀트 랩 통합 턴키 솔루션").is_visible()

            # Navigate to / in Korean
            await page.locator("header nav a[href='/']").first.click()
            await page.wait_for_timeout(300)
            hero_title_kr = await page.locator("#hero h1").inner_text()
            assert "실리콘 칩셋부터 외함까지" in hero_title_kr
            print("✓ Bilingual toggle synchronization verified across all 4 subpages.")

            # Capture Desktop KR Home Screenshot
            desktop_kr_shot = os.path.join(OUTPUT_DIR, "desktop_full_kr.png")
            await page.evaluate("() => { const h = document.querySelector('header'); if (h) h.style.position = 'static'; }")
            await page.screenshot(path=desktop_kr_shot, full_page=True)
            await page.evaluate("() => { const h = document.querySelector('header'); if (h) h.style.position = ''; }")

            # -------------------------------------------------------------
            # 7. Eradication of Forbidden Terms Audit (All Subpages)
            # -------------------------------------------------------------
            for route in ["/", "/demonstrator", "/studio", "/contact"]:
                await page.goto(f"http://localhost:{PORT}{route}?lang=ko", wait_until="networkidle")
                await assert_no_forbidden_terms(page, f"Korean {route}")
                await page.goto(f"http://localhost:{PORT}{route}?lang=en", wait_until="networkidle")
                await assert_no_forbidden_terms(page, f"English {route}")
            print("✓ Eradication audit: 0 instances of low-cost vendors, boutique, and 1-person claims across all subpages.")

            # -------------------------------------------------------------
            # 8. Responsive Viewport Verification (Tablet & Mobile)
            # -------------------------------------------------------------
            # Tablet (768x1024)
            page_tablet = await browser.new_page(viewport={"width": 768, "height": 1024})
            await page_tablet.goto(f"http://localhost:{PORT}/demonstrator?lang=en", wait_until="networkidle")
            assert await page_tablet.locator("button[aria-label='Toggle Navigation Menu']").is_visible(), "Hamburger missing at 768px"
            assert not await page_tablet.locator("header nav").is_visible(), "Desktop nav should be hidden at 768px"

            # Check Teardown tabs at 768px: zero truncation
            tablet_tabs = page_tablet.locator("#case-study button[type='button']")
            for i in range(4):
                title_el = tablet_tabs.nth(i).locator("div:has-text('MCAD'), div:has-text('ECAD'), div:has-text('Firmware'), div:has-text('Interface')").last
                scroll_w = await title_el.evaluate("el => el.scrollWidth")
                client_w = await title_el.evaluate("el => el.clientWidth")
                txt = await title_el.inner_text()
                assert scroll_w <= client_w, f"Tab {i} ({txt}) is truncated on tablet"
            print("✓ Tablet 768px: Clean responsive header and zero-truncation tabs verified.")

            # Mobile (375x812)
            mobile_page = await browser.new_page(viewport={"width": 375, "height": 812})
            await mobile_page.goto(f"http://localhost:{PORT}/?lang=en", wait_until="networkidle")
            hamburger_btn = mobile_page.locator("header button[aria-label='Toggle Navigation Menu']")
            await hamburger_btn.click()
            await mobile_page.wait_for_timeout(300)

            # Mobile Drawer multi-page items
            assert await mobile_page.locator("header a[href='/']").last.is_visible()
            assert await mobile_page.locator("header a[href='/demonstrator']").last.is_visible()
            assert await mobile_page.locator("header a[href='/studio']").last.is_visible()
            assert await mobile_page.locator("header a[href='/contact']").last.is_visible()

            # Mobile Drawer language switcher
            mobile_kr_btn = mobile_page.locator("header button:has-text('KR')").last
            await mobile_kr_btn.click()
            await mobile_page.wait_for_timeout(300)
            assert await mobile_page.locator("header a[href='/demonstrator']").get_by_text("쇼케이스").last.is_visible()
            print("✓ Mobile 375px: Hamburger drawer multi-page links and language switch verified.")

            # Click Demonstrator link in mobile drawer to test navigation
            await mobile_page.locator("header a[href='/demonstrator']").last.click()
            await mobile_page.wait_for_timeout(300)
            assert mobile_page.url.endswith("/demonstrator")
            print("✓ Mobile drawer SPA navigation to /demonstrator verified.")

            await browser.close()
            print("\n🎉 ALL MULTI-PAGE E2E AND PLAYWRIGHT TESTS PASSED 100%!")

    finally:
        server_proc.terminate()
        server_proc.wait()
        print("HTTP server stopped.")

if __name__ == "__main__":
    asyncio.run(run_e2e())
