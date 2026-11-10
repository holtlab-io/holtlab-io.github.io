import asyncio
import os
from playwright.async_api import async_playwright

LIVE_URL = "https://holtlab-io.github.io/"
OUTPUT_DIR = "/home/wimvm/.tuner/profiles/default/workspace/projects/holt-lab.github.io/screenshots/live"

async def verify_live():
    os.makedirs(OUTPUT_DIR, exist_ok=True)
    async with async_playwright() as p:
        browser = await p.chromium.launch(headless=True)
        page = await browser.new_page(viewport={"width": 1440, "height": 900})

        print(f"Navigating to live production: {LIVE_URL}?lang=en...")
        await page.goto(f"{LIVE_URL}?lang=en", wait_until="networkidle")
        await page.wait_for_timeout(1000)

        # 1. Verify Core Sections
        await page.wait_for_selector("header")
        await page.wait_for_selector("#hero")
        await page.wait_for_selector("#capabilities")
        await page.wait_for_selector("#case-study")
        await page.wait_for_selector("#specs")
        await page.wait_for_selector("#studio")
        await page.wait_for_selector("#inquire")
        await page.wait_for_selector("footer")
        print("✓ Live: All 7 core structural sections mounted in DOM.")

        # 2. Verify Title and Hero in EN
        title_en = await page.title()
        assert "Hardware & Embedded Engineering Studio" in title_en, f"Live title mismatch: {title_en}"
        print(f"✓ Live title: {title_en}")

        hero_title = await page.locator("#hero h1").inner_text()
        assert "From Silicon to Screen. We Engineer Physical Hardware." in hero_title, f"Live hero title mismatch: {hero_title}"
        print(f"✓ Live hero title: {hero_title}")

        hero_eyebrow = await page.locator("#hero").get_by_text("HARDWARE ENGINEERING & RAPID PROTOTYPING STUDIO").inner_text()
        assert "HARDWARE ENGINEERING" in hero_eyebrow
        print("✓ Live hero eyebrow verified.")

        # 3. Verify Capabilities Section
        cap_h2 = await page.locator("#capabilities h2").inner_text()
        assert "4 pillars of unified turnkey hardware" in cap_h2
        assert await page.locator("#capabilities").get_by_text("Mechanical CAD & Enclosures").is_visible()
        print(f"✓ Live capabilities: {cap_h2}")

        # 4. Verify Case Study Section
        case_eyebrow = await page.locator("#case-study").get_by_text("FLAGSHIP DEMONSTRATOR // CASE STUDY 01").first.inner_text()
        assert "FLAGSHIP DEMONSTRATOR" in case_eyebrow.upper()
        case_title = await page.locator("#case-study h2").inner_text()
        assert "win95 minesweeper touch keyring" in case_title
        print(f"✓ Live case study: {case_title} ({case_eyebrow})")

        # 5. Interactive Layer Switch
        tab_ecad = page.locator("#case-study button:has-text('ECAD 4-Layer PCBA')")
        await tab_ecad.click()
        await page.wait_for_timeout(300)
        assert await page.locator("#case-study").get_by_text("LAYER 02").first.is_visible()
        print("✓ Live interactive layer switch to ECAD verified.")

        # Capture Live Desktop EN Screenshot
        live_shot_en = os.path.join(OUTPUT_DIR, "live_desktop_en.png")
        await page.evaluate("() => { const h = document.querySelector('header'); if (h) h.style.position = 'static'; }")
        await page.screenshot(path=live_shot_en, full_page=True)
        await page.evaluate("() => { const h = document.querySelector('header'); if (h) h.style.position = ''; }")
        print(f"✓ Live desktop EN screenshot saved to {live_shot_en}")

        # 6. Test Korean Toggle
        kr_btn = page.locator("header button:has-text('KR')").first
        await kr_btn.click()
        await page.wait_for_timeout(500)

        title_kr = await page.title()
        assert "하드웨어 & 임베디드" in title_kr
        hero_title_kr = await page.locator("#hero h1").inner_text()
        assert "실리콘 칩셋부터 외함까지" in hero_title_kr
        cap_h2_kr = await page.locator("#capabilities h2").inner_text()
        assert "4대 턴키 하드웨어 엔지니어링 역량" in cap_h2_kr
        case_eyebrow_kr = await page.locator("#case-study").get_by_text("플래그십 실증 사례 // 케이스 스터디 01").first.inner_text()
        assert "플래그십 실증 사례" in case_eyebrow_kr
        print("✓ Live Korean toggle verified across Hero, Capabilities, and Case Study.")

        live_shot_kr = os.path.join(OUTPUT_DIR, "live_desktop_kr.png")
        await page.evaluate("() => { const h = document.querySelector('header'); if (h) h.style.position = 'static'; }")
        await page.screenshot(path=live_shot_kr, full_page=True)
        await page.evaluate("() => { const h = document.querySelector('header'); if (h) h.style.position = ''; }")
        print(f"✓ Live desktop KR screenshot saved to {live_shot_kr}")

        await browser.close()
        print("\n🎉 LIVE PRODUCTION VERIFICATION COMPLETED WITH 100% PASS!")

if __name__ == "__main__":
    asyncio.run(verify_live())
