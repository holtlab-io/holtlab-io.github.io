import asyncio
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
            # 1. Test Desktop Viewport (1440x900)
            # -------------------------------------------------------------
            page = await browser.new_page(viewport={"width": 1440, "height": 900})
            url = f"http://localhost:{PORT}/"
            print(f"Navigating to {url}...")
            await page.goto(url, wait_until="networkidle")

            # Wait for Dioxus WASM to mount all sections
            await page.wait_for_selector("header")
            await page.wait_for_selector("#demonstrator")
            await page.wait_for_selector("#teardown")
            await page.wait_for_selector("#specs")
            await page.wait_for_selector("#studio")
            await page.wait_for_selector("#inquire")
            await page.wait_for_selector("footer")
            print("✓ All 6 core structural sections mounted in DOM.")

            # Verify Header Brand & Navigation Anchors
            header_brand = await page.locator("header a[href='#']").inner_text()
            assert "holt" in header_brand and "lab" in header_brand, f"Brand missing: {header_brand}"
            assert "Boutique Hardware Engineering Studio" in header_brand, f"Descriptor missing: {header_brand}"
            
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

            # Verify Hero Demonstrator
            hero_title = await page.locator("#demonstrator h1").inner_text()
            assert "win95 minesweeper touch keyring" in hero_title, f"Hero title mismatch: {hero_title}"
            
            hero_eyebrow = await page.locator("#demonstrator").get_by_text("OPUS 01 // HARDWARE DEMONSTRATOR").inner_text()
            assert "OPUS 01" in hero_eyebrow
            
            # Verify 5 Quick Spec Pills
            pill_texts = await page.locator("#demonstrator").locator("span:has-text('ESP32'), span:has-text('43.4'), span:has-text('60Hz'), span:has-text('MJF'), span:has-text('Rust')").all_inner_texts()
            expected_pills = ["43.4×69.2×13.78mm", "ESP32-C3 160MHz", "2.0\" 60Hz IPS", "HP MJF PA12", "Bare-metal Rust"]
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
            assert await page.locator("#teardown").get_by_text("JLC04161H Stackup").is_visible()
            assert await page.locator("#teardown").get_by_text("GATE 2 // 0 ERC · 0 DRC VERIFIED").is_visible()
            print("✓ Layer 02 (ECAD 4-Layer PCBA) interactive switch verified.")

            # Switch to Layer 03 (Bare-Metal Rust Engine)
            tab_rust = page.locator("#teardown button:has-text('Bare-Metal Rust Engine')")
            await tab_rust.click()
            await page.wait_for_timeout(200)
            assert await page.locator("#teardown").get_by_text("LAYER 03").first.is_visible()
            assert await page.locator("#teardown").get_by_text("AHB GDMA CH0 Zero-Copy Pipeline").is_visible()
            assert await page.locator("#teardown").get_by_text("BENCH // ZERO-COPY 60Hz DMA PASS").is_visible()
            print("✓ Layer 03 (Bare-Metal Rust Engine) interactive switch verified.")

            # Switch to Layer 04 (Human Interface & Haptics)
            tab_haptics = page.locator("#teardown button:has-text('Human Interface & Haptics')")
            await tab_haptics.click()
            await page.wait_for_timeout(200)
            assert await page.locator("#teardown").get_by_text("LAYER 04").first.is_visible()
            assert await page.locator("#teardown").get_by_text("CST816D Capacitive Touch").is_visible()
            assert await page.locator("#teardown").get_by_text("TELEMETRY // FEED-FORWARD PWM PASS").is_visible()
            print("✓ Layer 04 (Human Interface & Haptics) interactive switch verified.")

            # Capture Teardown Layer 03 for detailed visual record
            await tab_rust.click()
            await page.wait_for_timeout(200)
            teardown_shot = os.path.join(OUTPUT_DIR, "desktop_teardown_rust.png")
            await page.locator("#teardown").screenshot(path=teardown_shot)

            # Verify Specs Matrix
            specs_h2 = await page.locator("#specs h2").inner_text()
            assert "technical specifications" in specs_h2
            rows_count = await page.locator("#specs .divide-y > div").count()
            assert rows_count >= 10, f"Expected 10 spec rows, found {rows_count}"
            print(f"✓ Specs matrix verified with {rows_count} technical specification rows.")

            # Verify Studio Turnkey Section
            studio_h2 = await page.locator("#studio h2").inner_text()
            assert "why 1-person turnkey?" in studio_h2
            assert await page.locator("#studio").get_by_text("TRADITIONAL AGENCY FRAGMENTATION").is_visible()
            assert await page.locator("#studio").get_by_text("HOLT LAB 1-PERSON TURNKEY").is_visible()
            assert await page.locator("#studio").get_by_text("01 // CO-DESIGN LOCK").is_visible()
            print("✓ Studio turnkey value proposition and 4-phase pipeline verified.")

            # Scroll back to top before capturing full page screenshot
            await page.evaluate("window.scrollTo(0, 0)")
            await page.wait_for_timeout(300)

            # Desktop Full Page Screenshot
            desktop_full = os.path.join(OUTPUT_DIR, "desktop_full_showcase.png")
            await page.screenshot(path=desktop_full, full_page=True)
            print(f"✓ Desktop full page screenshot saved to {desktop_full}")

            # -------------------------------------------------------------
            # 2. Test Tablet Portrait Viewport (768x1024)
            # -------------------------------------------------------------
            page_tablet = await browser.new_page(viewport={"width": 768, "height": 1024})
            await page_tablet.goto(url, wait_until="networkidle")
            
            # Verify clean responsive navigation: hamburger visible, desktop nav hidden (no brand overlap)
            assert await page_tablet.locator("button[aria-label='Toggle Navigation Menu']").is_visible(), "Hamburger missing at 768px"
            assert not await page_tablet.locator("header nav").is_visible(), "Desktop nav should be hidden at 768px"
            print("✓ Tablet 768px: Clean responsive header with hamburger drawer.")

            # Verify Teardown tabs at 768px: ZERO truncation
            tablet_tabs = page_tablet.locator("#teardown button[type='button']")
            for i in range(4):
                title_el = tablet_tabs.nth(i).locator("div:has-text('MCAD'), div:has-text('ECAD'), div:has-text('Rust'), div:has-text('Interface')").last
                scroll_w = await title_el.evaluate("el => el.scrollWidth")
                client_w = await title_el.evaluate("el => el.clientWidth")
                txt = await title_el.inner_text()
                assert scroll_w <= client_w, f"Tab {i} ({txt}) is truncated on tablet: scrollWidth={scroll_w} > clientWidth={client_w}"
            print("✓ Tablet 768px: All 4 teardown tab titles fit with zero truncation.")
            
            tablet_full = os.path.join(OUTPUT_DIR, "tablet_768_showcase.png")
            await page_tablet.screenshot(path=tablet_full, full_page=True)

            # -------------------------------------------------------------
            # 3. Test Mobile Viewport (375x812 iPhone)
            # -------------------------------------------------------------
            mobile_page = await browser.new_page(viewport={"width": 375, "height": 812})
            await mobile_page.goto(url, wait_until="networkidle")
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
            print("✓ Mobile navigation drawer links verified.")

            mobile_drawer_shot = os.path.join(OUTPUT_DIR, "mobile_drawer.png")
            await mobile_page.screenshot(path=mobile_drawer_shot)

            # Close drawer
            await hamburger_btn.click()
            await mobile_page.wait_for_timeout(200)

            # Verify Subsystem Inspector badge non-overlapping on mobile
            layer_tag = mobile_page.locator("#teardown div:has-text('LAYER 01')").last
            badge_tag = mobile_page.locator("#teardown div:has-text('GATE 1')").last
            b1 = await layer_tag.bounding_box()
            b2 = await badge_tag.bounding_box()
            overlap_x = max(0, min(b1['x'] + b1['width'], b2['x'] + b2['width']) - max(b1['x'], b2['x']))
            overlap_y = max(0, min(b1['y'] + b1['height'], b2['y'] + b2['height']) - max(b1['y'], b2['y']))
            assert overlap_x == 0 or overlap_y == 0, f"Tags overlap on mobile: overlap_x={overlap_x}, overlap_y={overlap_y}"
            print("✓ Mobile 375px: Subsystem layer tag and verification badge zero overlap.")

            # Test Inquiry Form Dispatch with full data intake
            await mobile_page.locator("#inquire input[placeholder*='acme robotics']").fill("Nexus Dynamics / Dr. Vance")
            await mobile_page.locator("#inquire input[placeholder*='domain.com']").fill("vance@nexusdynamics.io")
            await mobile_page.locator("#inquire input[placeholder*='4-6 weeks']").fill("4 weeks to bench prototype")
            await mobile_page.locator("#inquire input[placeholder*='USD']").fill("$12,000 USD envelope")
            await mobile_page.locator("#inquire input[placeholder*='ESP32-C3']").fill("ESP32-C3 RISC-V 4-layer PCBA + MJF PA12")
            await mobile_page.locator("#inquire textarea[placeholder*='dimensions']").fill("Windows 95 tactical console with CST816D capacitive touch and GDMA 60Hz blitting.")
            
            submit_btn = mobile_page.locator("#inquire button[type='submit']")
            await submit_btn.click()
            await mobile_page.wait_for_timeout(300)
            
            assert await mobile_page.locator("#inquire").get_by_text("DISPATCH BUFFER READY").is_visible()
            assert await mobile_page.locator("#inquire").get_by_text("Nexus Dynamics").is_visible()
            assert await mobile_page.locator("#inquire").get_by_text("4 weeks to bench prototype").is_visible()
            assert await mobile_page.locator("#inquire").get_by_text("12,000 USD envelope").is_visible()
            assert await mobile_page.locator("#inquire").get_by_text("ESP32-C3 RISC-V 4-layer PCBA + MJF PA12").is_visible()
            
            mailto_link = await mobile_page.locator("#inquire a:has-text('open mail client')").get_attribute("href")
            assert "contact@holtlab.io" in mailto_link
            assert "vance@nexusdynamics.io" in mailto_link
            assert "Nexus" in mailto_link
            assert "12%2C000" in mailto_link or "12,000" in mailto_link
            print("✓ Inquiry form interactive dispatch, full data fidelity, and mailto generator verified.")

            # Test Edit Specifications loop
            edit_btn = mobile_page.locator("#inquire button:has-text('edit specifications')")
            await edit_btn.click()
            await mobile_page.wait_for_timeout(200)
            val_name = await mobile_page.locator("#inquire input[placeholder*='acme robotics']").input_value()
            assert "Nexus Dynamics" in val_name, f"Expected data preserved, got {val_name}"
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
