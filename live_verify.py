import asyncio
import os
from playwright.async_api import async_playwright

LIVE_URL = "https://holtlab-io.github.io"
OUTPUT_DIR = "/home/wimvm/.tuner/profiles/default/workspace/projects/holt-lab.github.io/screenshots/live"

async def verify_live():
    os.makedirs(OUTPUT_DIR, exist_ok=True)
    async with async_playwright() as p:
        browser = await p.chromium.launch(headless=True)
        page = await browser.new_page(viewport={"width": 1440, "height": 900})

        print(f"Navigating to live production: {LIVE_URL}/?lang=en...")
        await page.goto(f"{LIVE_URL}/?lang=en", wait_until="networkidle")
        await page.wait_for_timeout(1000)

        # 1. Page 1: / (Studio Overview Home)
        await page.wait_for_selector("header")
        await page.wait_for_selector("#hero")
        await page.wait_for_selector("#capabilities")
        await page.wait_for_selector("footer")
        print("✓ Live: Studio Home core sections mounted in DOM.")

        title_en = await page.title()
        assert "Hardware & Embedded Engineering Studio" in title_en, f"Live title mismatch: {title_en}"
        print(f"✓ Live title: {title_en}")

        hero_title = await page.locator("#hero h1").inner_text()
        assert "From Silicon to Screen. We Engineer Physical Hardware." in hero_title, f"Live hero title mismatch: {hero_title}"
        print(f"✓ Live hero title: {hero_title}")

        hero_eyebrow = await page.locator("#hero").get_by_text("HARDWARE ENGINEERING & RAPID PROTOTYPING STUDIO").inner_text()
        assert "HARDWARE ENGINEERING" in hero_eyebrow
        print("✓ Live hero eyebrow verified.")

        # Capabilities 4 pillars on Home
        cap_h2 = await page.locator("#capabilities h2").inner_text()
        assert "4 pillars of unified turnkey hardware" in cap_h2
        assert await page.locator("#capabilities").get_by_text("Mechanical CAD & Enclosures").is_visible()
        print(f"✓ Live capabilities: {cap_h2}")

        # Capture Live Desktop EN Screenshot
        live_shot_en = os.path.join(OUTPUT_DIR, "live_desktop_en.png")
        await page.evaluate("() => { const h = document.querySelector('header'); if (h) h.style.position = 'static'; }")
        await page.screenshot(path=live_shot_en, full_page=True)
        await page.evaluate("() => { const h = document.querySelector('header'); if (h) h.style.position = ''; }")
        print(f"✓ Live desktop EN screenshot saved to {live_shot_en}")

        # 2. Page 2: /demonstrator (Deep Flagship Case Study)
        print("Navigating to live /demonstrator...")
        await page.locator("header nav a[href='/demonstrator']").first.click()
        await page.wait_for_timeout(1000)
        assert page.url.endswith("/demonstrator"), f"Expected /demonstrator, got {page.url}"

        # Interactive 4-view CAD Inspection Modes
        await page.locator("button:has-text('01 // 3D ISO')").click()
        await page.wait_for_timeout(300)
        assert await page.locator("text=PERSPECTIVE: 3D PROJECTION").first.is_visible()
        await page.locator("button:has-text('03 // INTERNAL SECTION')").click()
        await page.wait_for_timeout(300)
        assert await page.locator("text=ANATOMY: CROSS-SECTION").first.is_visible()
        print("✓ Live: /demonstrator CAD inspection modes verified.")

        # 4-Layer Anatomical Teardown Switch
        tab_ecad = page.locator("#case-study button:has-text('ECAD 4-Layer PCBA')")
        await tab_ecad.click()
        await page.wait_for_timeout(300)
        assert await page.locator("#case-study").get_by_text("LAYER 02").first.is_visible()
        print("✓ Live: /demonstrator interactive layer switch to ECAD verified.")

        # 10-Row Swiss Spec Matrix
        specs_rows = await page.locator("#specs .divide-y > div").count()
        assert specs_rows >= 10, f"Expected >= 10 spec rows, got {specs_rows}"
        print(f"✓ Live: /demonstrator specs matrix verified with {specs_rows} rows.")

        # 3. Page 3: /studio (Turnkey Architecture & Methodology)
        print("Navigating to live /studio...")
        await page.locator("header nav a[href='/studio']").first.click()
        await page.wait_for_timeout(1000)
        assert page.url.endswith("/studio"), f"Expected /studio, got {page.url}"
        assert await page.locator("#studio").get_by_text("TRADITIONAL AGENCY FRAGMENTATION").is_visible()
        assert await page.locator("#studio").get_by_text("HOLT LAB UNIFIED TURNKEY").is_visible()
        print("✓ Live: /studio methodology & comparison cards verified.")

        # 4. Page 4: /contact (Direct Client RFQ Intake)
        print("Navigating to live /contact...")
        await page.locator("header nav a[href='/contact']").first.click()
        await page.wait_for_timeout(1000)
        assert page.url.endswith("/contact"), f"Expected /contact, got {page.url}"
        assert await page.locator("#inquire").get_by_text("contact@holtlab.io").first.is_visible()
        print("✓ Live: /contact direct inquiry form verified.")

        # 5. Korean Toggle Across Subpages
        kr_btn = page.locator("header button:has-text('KR')").first
        await kr_btn.click()
        await page.wait_for_timeout(500)
        title_kr = await page.title()
        assert "하드웨어 & 임베디드" in title_kr
        print(f"✓ Live: Korean toggle verified: {title_kr}")

        # Navigate back to Home in Korean
        await page.locator("header nav a[href='/']").first.click()
        await page.wait_for_timeout(1000)
        hero_title_kr = await page.locator("#hero h1").inner_text()
        assert "실리콘 칩셋부터 외함까지" in hero_title_kr
        print("✓ Live: Multi-page navigation preserved in Korean.")

        live_shot_kr = os.path.join(OUTPUT_DIR, "live_desktop_kr.png")
        await page.evaluate("() => { const h = document.querySelector('header'); if (h) h.style.position = 'static'; }")
        await page.screenshot(path=live_shot_kr, full_page=True)
        await page.evaluate("() => { const h = document.querySelector('header'); if (h) h.style.position = ''; }")
        print(f"✓ Live desktop KR screenshot saved to {live_shot_kr}")

        await browser.close()
        print("\n🎉 LIVE PRODUCTION MULTI-PAGE VERIFICATION COMPLETED WITH 100% PASS!")

if __name__ == "__main__":
    asyncio.run(verify_live())
