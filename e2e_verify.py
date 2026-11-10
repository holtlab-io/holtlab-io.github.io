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

async def run_e2e():
    os.makedirs(OUTPUT_DIR, exist_ok=True)

    # -------------------------------------------------------------
    # 0. Pre-Flight File Verification (SEO & Anti-FOUC Assets)
    # -------------------------------------------------------------
    robots_path = os.path.join(PUBLIC_DIR, "robots.txt")
    sitemap_path = os.path.join(PUBLIC_DIR, "sitemap.xml")
    index_path = os.path.join(PUBLIC_DIR, "index.html")

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
    assert "https://holtlab-io.github.io/?lang=ko" in sitemap_content, "sitemap missing Korean loc"
    assert "https://holtlab-io.github.io/?lang=en" in sitemap_content, "sitemap missing English loc"
    assert "hreflang=\"ko\"" in sitemap_content, "sitemap missing Korean hreflang"
    assert "hreflang=\"en\"" in sitemap_content, "sitemap missing English hreflang"
    print("✓ public/sitemap.xml verified with all 3 multilingual entries and hreflang links.")

    with open(index_path, "r", encoding="utf-8") as f:
        index_html = f.read()
    assert "assets/tailwind.css" in index_html, "tailwind stylesheet missing in index.html"
    assert "pretendard" in index_html, "pretendard stylesheet missing in index.html"
    assert "JetBrains+Mono" in index_html, "JetBrains Mono stylesheet missing in index.html"
    assert "background-color: #FAFAFA" in index_html, "anti-FOUC background style missing in index.html"
    print("✓ Anti-FOUC head preloads and background styles verified in index.html.")

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
            # 1. Test Desktop Viewport (1440x900) & SEO DOM Verification
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

            og_image = await page.locator("meta[property='og:image']").first.get_attribute("content")
            assert "banner.png" in og_image, f"og:image mismatch: {og_image}"

            tw_card = await page.locator("meta[name='twitter:card']").first.get_attribute("content")
            assert tw_card == "summary_large_image", f"twitter:card mismatch: {tw_card}"

            canonical = await page.locator("link[rel='canonical']").first.get_attribute("href")
            assert "holtlab-io.github.io" in canonical, f"canonical mismatch: {canonical}"

            ld_json_el = page.locator("script[type='application/ld+json']").first
            assert await ld_json_el.count() > 0, "JSON-LD structured data missing"
            ld_data = json.loads(await ld_json_el.inner_text())
            graph_types = [item.get("@type") for item in ld_data.get("@graph", [])]
            assert "Organization" in graph_types, "Organization schema missing in JSON-LD"
            assert "Product" in graph_types, "Product schema missing in JSON-LD"
            print("✓ Full SEO meta tags, OpenGraph, Twitter Cards, and JSON-LD schemas verified.")

            # Wait for Dioxus WASM to mount all sections
            await page.wait_for_selector("header")
            await page.wait_for_selector("#demonstrator")
            await page.wait_for_selector("#teardown")
            await page.wait_for_selector("#specs")
            await page.wait_for_selector("#studio")
            await page.wait_for_selector("#inquire")
            await page.wait_for_selector("footer")
            print("✓ All 6 core structural sections mounted in DOM.")

            # Verify Dynamic Title & html lang in English mode
            title_en = await page.title()
            assert "Hardware & Embedded Engineering Studio" in title_en, f"Initial title mismatch: {title_en}"
            html_lang_en = await page.locator("html").get_attribute("lang")
            assert html_lang_en == "en", f"Expected html lang='en', got {html_lang_en}"
            en_btn = page.locator("header button:has-text('EN')").first
            assert await en_btn.get_attribute("aria-pressed") == "true", "EN button should have aria-pressed=true"
            print("✓ Dynamic document title, html lang='en', and aria-pressed verified in English.")

            # Verify Header Brand & Navigation Anchors
            header_brand = await page.locator("header a[href='#']").inner_text()
            assert "holt" in header_brand and "lab" in header_brand, f"Brand missing: {header_brand}"
            assert "Hardware & Embedded Engineering Studio" in header_brand, f"Descriptor missing: {header_brand}"

            # Verify Desktop Navigation contains all 5 anchors
            nav_links = await page.locator("header nav a").all_inner_texts()
            print(f"Desktop nav links: {nav_links}")
            assert any("demonstrator" in l for l in nav_links), "demonstrator anchor missing"
            assert any("teardown" in l for l in nav_links), "teardown anchor missing"
            assert any("specs" in l for l in nav_links), "specs anchor missing"
            assert any("studio" in l for l in nav_links), "studio anchor missing"
            assert any("inquire" in l for l in nav_links), "inquire anchor missing"

            cta_text = await page.locator("header a:has-text('inquire // 의뢰하기')").inner_text()
            assert "inquire // 의뢰하기" in cta_text, f"CTA missing: {cta_text}"
            print("✓ Header brand lockup, all 5 navigation anchors, and CTA button verified.")

            # Verify Hero Demonstrator in English
            hero_title = await page.locator("#demonstrator h1").inner_text()
            assert "win95 minesweeper touch keyring" in hero_title, f"Hero title mismatch: {hero_title}"

            hero_eyebrow = await page.locator("#demonstrator").get_by_text("OPUS 01 // HARDWARE DEMONSTRATOR").inner_text()
            assert "OPUS 01" in hero_eyebrow

            # Verify 5 Quick Spec Pills
            pill_texts = await page.locator("#demonstrator").locator("span:has-text('ESP32'), span:has-text('43.4'), span:has-text('60Hz'), span:has-text('MJF'), span:has-text('Firmware')").all_inner_texts()
            expected_pills = ["43.4×69.2×13.78mm", "ESP32-C3 160MHz", "2.0\" 60Hz IPS", "HP MJF PA12", "Real-Time Firmware"]
            print(f"Pills found: {pill_texts}")
            for exp in expected_pills:
                assert any(exp in pt for pt in pill_texts), f"Pill {exp} missing in {pill_texts}"
            print("✓ Hero demonstrator title, eyebrow, and all 5 spec pills verified.")

            # Verify Teardown Section & Exploded Banner
            teardown_heading = await page.locator("#teardown h2").inner_text()
            assert "anatomical engineering teardown" in teardown_heading

            exploded_heading = await page.locator("#teardown h3").first.inner_text()
            assert "pulled apart to the bare copper." in exploded_heading
            print("✓ Exploded teardown banner verified.")

            # Test Interactive Subsystem Tabs:
            # Layer 01 default active
            assert await page.locator("#teardown").get_by_text("LAYER 01").first.is_visible()
            assert await page.locator("#teardown").get_by_text("MCAD Enclosure").first.is_visible()

            # Switch to Layer 02 (ECAD 4-Layer PCBA)
            tab_ecad = page.locator("#teardown button:has-text('ECAD 4-Layer PCBA')")
            await tab_ecad.click()
            await page.wait_for_timeout(200)
            assert await page.locator("#teardown").get_by_text("LAYER 02").first.is_visible()
            assert await page.locator("#teardown").get_by_text("Controlled-Impedance Stackup").is_visible()
            assert await page.locator("#teardown").get_by_text("GATE 2 // 0 ERC · 0 DRC VERIFIED").is_visible()
            print("✓ Layer 02 (ECAD 4-Layer PCBA) interactive switch verified.")

            # Switch to Layer 03 (High-Reliability Embedded Firmware Engine)
            tab_rust = page.locator("#teardown button:has-text('High-Reliability Embedded Firmware Engine')")
            await tab_rust.click()
            await page.wait_for_timeout(200)
            assert await page.locator("#teardown").get_by_text("LAYER 03").first.is_visible()
            assert await page.locator("#teardown").get_by_text("AHB GDMA CH0 Zero-Copy Pipeline").is_visible()
            assert await page.locator("#teardown").get_by_text("BENCH // ZERO-COPY 60Hz DMA PASS").is_visible()
            print("✓ Layer 03 (High-Reliability Embedded Firmware Engine) interactive switch verified.")

            # Switch to Layer 04 (Human Interface & Haptics)
            tab_haptics = page.locator("#teardown button:has-text('Human Interface & Haptics')")
            await tab_haptics.click()
            await page.wait_for_timeout(200)
            assert await page.locator("#teardown").get_by_text("LAYER 04").first.is_visible()
            assert await page.locator("#teardown").get_by_text("CST816D Capacitive Touch").is_visible()
            assert await page.locator("#teardown").get_by_text("TELEMETRY // FEED-FORWARD PWM PASS").is_visible()
            print("✓ Layer 04 (Human Interface & Haptics) interactive switch verified.")

            # Verify Specs Matrix
            specs_h2 = await page.locator("#specs h2").inner_text()
            assert "technical specifications" in specs_h2
            rows_count = await page.locator("#specs .divide-y > div").count()
            assert rows_count >= 10, f"Expected 10 spec rows, found {rows_count}"
            print(f"✓ Specs matrix verified with {rows_count} technical specification rows.")

            # Verify Studio Turnkey Section
            studio_h2 = await page.locator("#studio h2").inner_text()
            assert "why unified turnkey?" in studio_h2
            assert await page.locator("#studio").get_by_text("TRADITIONAL AGENCY FRAGMENTATION").is_visible()
            assert await page.locator("#studio").get_by_text("HOLT LAB UNIFIED TURNKEY").is_visible()
            assert await page.locator("#studio").get_by_text("01 // CO-DESIGN LOCK").is_visible()
            print("✓ Studio turnkey value proposition and 4-phase pipeline verified.")

            # Verify Eradication of Forbidden Terms in EN mode
            body_text_en = await page.inner_text("body")
            dom_html_en = await page.content()
            forbidden_terms = [
                "0.00 mm³", "0.000 mm³", "0.00mm³", "0.00 mm3", "0.000 mm3",
                "JLCPCB", "JLC3DP", "JLC04161H", "정밀공차",
                "Boutique", "boutique", "부티크", "BOUTIQUE",
                "1인", "1-person", "1-PERSON", "single architect", "단 한 명",
                "단 한명", "단일 리드", "1인 턴키", "1 person", "one-person", "single-threaded"
            ]
            for term in forbidden_terms:
                assert term not in body_text_en, f"Forbidden term '{term}' found in English body text"
                assert term not in dom_html_en, f"Forbidden term '{term}' found in English DOM HTML"
                # Case-insensitive check for ASCII phrases
                if term.isascii() and term.isalpha():
                    assert term.lower() not in body_text_en.lower(), f"Forbidden term '{term}' (case-insensitive) found in English body text"
                    assert term.lower() not in dom_html_en.lower(), f"Forbidden term '{term}' (case-insensitive) found in English DOM HTML"
            # Explicit case-insensitive check for solo terms
            for solo_phrase in ["1-person", "single architect", "one-person", "single-threaded"]:
                assert solo_phrase not in body_text_en.lower(), f"Forbidden solo phrase '{solo_phrase}' (case-insensitive) found in English body text"
                assert solo_phrase not in dom_html_en.lower(), f"Forbidden solo phrase '{solo_phrase}' (case-insensitive) found in English DOM HTML"
            print("✓ Eradication audit: 0 instances of low-cost vendors, boutique, boolean CAD metrics, and 1-person solo claims in EN.")

            # Capture Desktop EN Screenshot
            desktop_full_en = os.path.join(OUTPUT_DIR, "desktop_full_en.png")
            await page.screenshot(path=desktop_full_en, full_page=True)

            # -------------------------------------------------------------
            # 2. Test Language Switcher: Toggle to KR
            # -------------------------------------------------------------
            kr_toggle_btn = page.locator("header button:has-text('KR')").first
            await kr_toggle_btn.click()
            await page.wait_for_timeout(300)

            # Verify Document Title & html lang dynamically updated to Korean
            title_kr = await page.title()
            assert "하드웨어 & 임베디드" in title_kr, f"Korean title mismatch: {title_kr}"
            html_lang_kr = await page.locator("html").get_attribute("lang")
            assert html_lang_kr == "ko", f"Expected html lang='ko', got {html_lang_kr}"
            assert await kr_toggle_btn.get_attribute("aria-pressed") == "true", "KR button should have aria-pressed=true"
            print("✓ Dynamic document title, html lang='ko', and aria-pressed verified in Korean.")

            # Verify Hero updated to Korean
            hero_eyebrow_kr = await page.locator("#demonstrator").get_by_text("OPUS 01 // 하드웨어 실증 머신").first.inner_text()
            assert "하드웨어 실증 머신" in hero_eyebrow_kr

            assert await page.locator("#demonstrator").get_by_text("무간섭 일체형 기구 설계").first.is_visible()
            assert await page.locator("#demonstrator").get_by_text("초저지연 펌웨어", exact=True).is_visible()
            print("✓ Language toggle to KR: Hero updated dynamically to Korean.")

            # Verify Teardown updated to Korean
            teardown_h2_kr = await page.locator("#teardown h2").inner_text()
            assert "엔지니어링 해부학적 분해" in teardown_h2_kr
            assert await page.locator("#teardown").get_by_text("순수 구리 패턴까지 완벽히 분해하다.").first.is_visible()
            assert await page.locator("#teardown").get_by_text("MCAD 정밀 기구 외함").first.is_visible()
            print("✓ Language toggle to KR: Teardown updated dynamically to Korean.")

            # Verify Specs Matrix updated to Korean
            specs_h2_kr = await page.locator("#specs h2").inner_text()
            assert "상세 기술 제원" in specs_h2_kr
            assert await page.locator("#specs").get_by_text("외형 치수 및 무게").first.is_visible()
            print("✓ Language toggle to KR: Specs matrix updated dynamically to Korean.")

            # Verify Studio updated to Korean
            studio_h2_kr = await page.locator("#studio h2").inner_text()
            assert "왜 통합 턴키인가?" in studio_h2_kr
            assert await page.locator("#studio").get_by_text("홀트 랩 통합 턴키 솔루션").first.is_visible()
            assert await page.locator("#studio").get_by_text("기존 분절형 다중 외주 모델").first.is_visible()
            print("✓ Language toggle to KR: Studio turnkey updated dynamically to Korean.")

            # Verify Inquire updated to Korean
            inquire_h2_kr = await page.locator("#inquire h2").inner_text()
            assert "프로젝트 의뢰 및 기술 상담." in inquire_h2_kr
            assert await page.locator("#inquire").get_by_text("개발 분야 선택").first.is_visible()
            assert await page.locator("#inquire").get_by_text("턴키 하드웨어 개발").first.is_visible()
            print("✓ Language toggle to KR: Inquire section updated dynamically to Korean.")

            # Verify localStorage persistence of language preference
            saved_lang = await page.evaluate("localStorage.getItem('holtlab_lang')")
            assert saved_lang == "ko", f"Expected localStorage holtlab_lang to be 'ko', got {saved_lang}"
            print("✓ LocalStorage persistence verified: holtlab_lang = 'ko'.")

            # Verify Eradication of Forbidden Terms in KR mode
            body_text_kr = await page.inner_text("body")
            dom_html_kr = await page.content()
            for term in forbidden_terms:
                assert term not in body_text_kr, f"Forbidden term '{term}' found in Korean body text"
                assert term not in dom_html_kr, f"Forbidden term '{term}' found in Korean DOM HTML"
            # Explicit case-insensitive check for solo terms
            for solo_phrase in ["1-person", "single architect", "one-person", "single-threaded"]:
                assert solo_phrase not in body_text_kr.lower(), f"Forbidden solo phrase '{solo_phrase}' (case-insensitive) found in Korean body text"
                assert solo_phrase not in dom_html_kr.lower(), f"Forbidden solo phrase '{solo_phrase}' (case-insensitive) found in Korean DOM HTML"
            assert "공차" not in body_text_kr, "Forbidden term '공차' found in Korean body text"
            print("✓ Eradication audit: 0 instances of low-cost vendors, boutique, boolean CAD metrics, and 1-person solo claims in KR.")

            # Capture Desktop KR Screenshot
            desktop_full_kr = os.path.join(OUTPUT_DIR, "desktop_full_kr.png")
            await page.screenshot(path=desktop_full_kr, full_page=True)

            # Test reload remembers Korean preference
            await page.reload(wait_until="networkidle")
            await page.wait_for_selector("header")
            assert await page.locator("#demonstrator").get_by_text("하드웨어 실증 머신").is_visible()
            print("✓ Reload persistence verified: Page initialized in Korean from localStorage.")

            # Toggle back to EN
            en_toggle_btn = page.locator("header button:has-text('EN')").first
            await en_toggle_btn.click()
            await page.wait_for_timeout(300)
            assert await page.locator("#demonstrator").get_by_text("HARDWARE DEMONSTRATOR").is_visible()
            saved_lang_en = await page.evaluate("localStorage.getItem('holtlab_lang')")
            assert saved_lang_en == "en", f"Expected localStorage holtlab_lang to be 'en', got {saved_lang_en}"
            print("✓ Language toggle to EN: Successfully returned to English and saved to localStorage.")

            # Test URL query parameter precedence over existing localStorage
            # With localStorage currently set to 'en', navigate directly to ?lang=ko
            await page.goto(f"http://localhost:{PORT}/?lang=ko", wait_until="networkidle")
            await page.wait_for_selector("header")
            assert await page.locator("#demonstrator").get_by_text("하드웨어 실증 머신").is_visible(), "URL ?lang=ko failed to override localStorage 'en'"
            saved_after_url = await page.evaluate("localStorage.getItem('holtlab_lang')")
            assert saved_after_url == "ko", f"Expected localStorage to sync to 'ko' from URL parameter, got {saved_after_url}"
            # Reset back to EN for remaining tests
            await page.locator("header button:has-text('EN')").first.click()
            await page.wait_for_timeout(200)
            print("✓ URL parameter precedence over localStorage and auto-sync verified.")

            # -------------------------------------------------------------
            # 3. Test Tablet Portrait Viewport (768x1024)
            # -------------------------------------------------------------
            page_tablet = await browser.new_page(viewport={"width": 768, "height": 1024})
            await page_tablet.goto(f"http://localhost:{PORT}/?lang=en", wait_until="networkidle")

            # Verify responsive navigation: hamburger visible, desktop nav hidden
            assert await page_tablet.locator("button[aria-label='Toggle Navigation Menu']").is_visible(), "Hamburger missing at 768px"
            assert not await page_tablet.locator("header nav").is_visible(), "Desktop nav should be hidden at 768px"
            print("✓ Tablet 768px: Clean responsive header with hamburger drawer.")

            # Verify Teardown tabs at 768px: ZERO truncation
            tablet_tabs = page_tablet.locator("#teardown button[type='button']")
            for i in range(4):
                title_el = tablet_tabs.nth(i).locator("div:has-text('MCAD'), div:has-text('ECAD'), div:has-text('Firmware'), div:has-text('Interface')").last
                scroll_w = await title_el.evaluate("el => el.scrollWidth")
                client_w = await title_el.evaluate("el => el.clientWidth")
                txt = await title_el.inner_text()
                assert scroll_w <= client_w, f"Tab {i} ({txt}) is truncated on tablet: scrollWidth={scroll_w} > clientWidth={client_w}"
            print("✓ Tablet 768px: All 4 teardown tab titles fit with zero truncation.")

            tablet_full = os.path.join(OUTPUT_DIR, "tablet_768_showcase.png")
            await page_tablet.screenshot(path=tablet_full, full_page=True)

            # -------------------------------------------------------------
            # 4. Test Mobile Viewport (375x812 iPhone) & Mobile Language Switcher
            # -------------------------------------------------------------
            mobile_page = await browser.new_page(viewport={"width": 375, "height": 812})
            await mobile_page.goto(f"http://localhost:{PORT}/?lang=en", wait_until="networkidle")
            await mobile_page.wait_for_selector("header")

            # Test Mobile Hamburger Menu Drawer
            hamburger_btn = mobile_page.locator("header button[aria-label='Toggle Navigation Menu']")
            await hamburger_btn.click()
            await mobile_page.wait_for_timeout(300)

            # Verify Mobile Drawer items
            assert await mobile_page.locator("header a[href='#demonstrator']").last.is_visible()
            assert await mobile_page.locator("header a[href='#teardown']").last.is_visible()
            assert await mobile_page.locator("header a[href='#specs']").last.is_visible()
            assert await mobile_page.locator("header a[href='#studio']").last.is_visible()
            assert await mobile_page.locator("header a[href='#inquire']").last.is_visible()

            # Verify Mobile Language Toggle in Drawer
            mobile_kr_btn = mobile_page.locator("header button:has-text('KR')").last
            await mobile_kr_btn.click()
            await mobile_page.wait_for_timeout(300)

            # Drawer labels should now be in Korean
            assert await mobile_page.locator("header a[href='#demonstrator']").get_by_text("실증 기기").last.is_visible()
            print("✓ Mobile drawer language toggle verified.")

            mobile_drawer_shot = os.path.join(OUTPUT_DIR, "mobile_drawer_kr.png")
            await mobile_page.screenshot(path=mobile_drawer_shot)

            # Close drawer
            await hamburger_btn.click()
            await mobile_page.wait_for_timeout(200)

            # Verify Subsystem Inspector badge non-overlapping on mobile
            layer_tag = mobile_page.locator("#teardown div:has-text('LAYER 01')").last
            badge_tag = mobile_page.locator("#teardown div:has-text('게이트 1'), #teardown div:has-text('GATE 1')").last
            b1 = await layer_tag.bounding_box()
            b2 = await badge_tag.bounding_box()
            overlap_x = max(0, min(b1['x'] + b1['width'], b2['x'] + b2['width']) - max(b1['x'], b2['x']))
            overlap_y = max(0, min(b1['y'] + b1['height'], b2['y'] + b2['height']) - max(b1['y'], b2['y']))
            assert overlap_x == 0 or overlap_y == 0, f"Tags overlap on mobile: overlap_x={overlap_x}, overlap_y={overlap_y}"
            print("✓ Mobile 375px: Subsystem layer tag and verification badge zero overlap.")

            # Take a mobile screenshot of the new contact section before fill (temporarily hide sticky header)
            mobile_contact_el = mobile_page.locator("#inquire")
            mobile_contact_shot = os.path.join(OUTPUT_DIR, "mobile_contact_redesign.png")
            await mobile_page.evaluate("() => { const h = document.querySelector('header'); if (h) h.style.display = 'none'; }")
            await mobile_contact_el.screenshot(path=mobile_contact_shot)
            await mobile_page.evaluate("() => { const h = document.querySelector('header'); if (h) h.style.display = ''; }")
            print(f"✓ Mobile contact section redesign screenshot saved to {mobile_contact_shot}")

            # Test Inquiry Form Dispatch with full data intake in Korean mode
            await mobile_page.locator("#inquire input[required][type='text']").fill("넥서스 다이내믹스 / 반스 박사")
            await mobile_page.locator("#inquire input[required][type='email']").fill("vance@nexusdynamics.io")
            await mobile_page.locator("#inquire input[placeholder*='주'], #inquire input[placeholder*='weeks']").fill("작동 시제품까지 4주")
            await mobile_page.locator("#inquire input[placeholder*='원'], #inquire input[placeholder*='USD']").fill("1,500만 원 예산 범위")
            await mobile_page.locator("#inquire textarea[required]").fill("윈도우 95 감성 포터블 콘솔, 4층 임피던스 제어 기판, CST816D 정전식 터치 및 60Hz GDMA 전송.")

            submit_btn = mobile_page.locator("#inquire button[type='submit']")
            await submit_btn.click()
            await mobile_page.wait_for_timeout(300)

            assert await mobile_page.locator("#inquire").get_by_text("발송 버퍼 준비 완료").is_visible()
            assert await mobile_page.locator("#inquire").get_by_text("넥서스 다이내믹스").is_visible()
            assert await mobile_page.locator("#inquire").get_by_text("작동 시제품까지 4주").is_visible()
            assert await mobile_page.locator("#inquire").get_by_text("1,500만 원 예산 범위").is_visible()
            assert await mobile_page.locator("#inquire").get_by_text("윈도우 95 감성 포터블 콘솔").is_visible()

            mailto_link = await mobile_page.locator("#inquire a:has-text('메일 프로그램 열기'), #inquire a:has-text('open mail client')").get_attribute("href")
            assert "contact@holtlab.io" in mailto_link
            assert "vance@nexusdynamics.io" in mailto_link
            assert "subject=" in mailto_link
            assert "body=" in mailto_link
            # RFC 3986 / RFC 6068: query part must contain zero raw spaces and use percent-encoding
            query_str = mailto_link.split("?", 1)[1]
            assert " " not in query_str, f"Mailto query contains raw unencoded space: {query_str}"
            assert "%20" in query_str, "Mailto query missing percent-encoded spaces (%20)"
            print("✓ Inquiry form interactive dispatch, full data fidelity, and RFC percent-encoded mailto generator verified in Korean.")

            # Test Edit Specifications loop
            edit_btn = mobile_page.locator("#inquire button:has-text('수정하기'), #inquire button:has-text('edit specifications')")
            await edit_btn.click()
            await mobile_page.wait_for_timeout(200)
            val_name = await mobile_page.locator("#inquire input[required][type='text']").input_value()
            assert "넥서스 다이내믹스" in val_name, f"Expected data preserved, got {val_name}"
            print("✓ Inquiry form edit specifications loop verified: all input state preserved.")

            # Mobile Full Page Screenshot
            mobile_full = os.path.join(OUTPUT_DIR, "mobile_full_showcase.png")
            await mobile_page.screenshot(path=mobile_full, full_page=True)
            print(f"✓ Mobile full page screenshot saved to {mobile_full}")

            # Verify Footer
            github_href = await page.locator("footer a:has-text('github')").get_attribute("href")
            assert github_href == "https://github.com/holtlab-io", f"GitHub URL mismatch: {github_href}"
            youtube_href = await page.locator("footer a:has-text('youtube')").get_attribute("href")
            assert "youtube.com/@holt-lab" in youtube_href, f"YouTube URL mismatch: {youtube_href}"
            print("✓ Footer organization links verified.")

            await browser.close()
            print("\n🎉 ALL E2E AND PLAYWRIGHT TESTS PASSED 100%!")

    finally:
        server_proc.terminate()
        server_proc.wait()
        print("HTTP server stopped.")

if __name__ == "__main__":
    asyncio.run(run_e2e())
