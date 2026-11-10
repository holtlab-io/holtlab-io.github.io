import asyncio
import os
import subprocess
import time
from playwright.async_api import async_playwright
from PIL import Image

PUBLIC_DIR = "/home/wimvm/.tuner/profiles/default/workspace/projects/holt-lab.github.io/target/dx/holtlab-landing/release/web/public"
OUTPUT_DIR = "/home/wimvm/.gemini/antigravity-cli/brain/c5c825f4-ebe2-47c6-b1e5-10e9ee9cf9c3"
REF_IMG = os.path.join(OUTPUT_DIR, "ref_teenage_engineering_store.png")
PORT = 8099

async def run_e2e():
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

            # Test Desktop View
            page = await browser.new_page(viewport={"width": 1440, "height": 900})
            url = f"http://localhost:{PORT}/"
            print(f"Navigating to {url}...")
            await page.goto(url, wait_until="networkidle")

            # Wait for Dioxus WASM to mount
            await page.wait_for_selector("header")
            await page.wait_for_selector("#catalog")
            print("Page loaded and WASM mounted successfully.")

            # Capture Desktop Full Screenshot
            desktop_path = os.path.join(OUTPUT_DIR, "holtlab_te_desktop.png")
            await page.screenshot(path=desktop_path)
            print(f"Desktop screenshot saved to {desktop_path}")

            # Test Interactive Modal: click on the first product card
            first_card = page.locator("#catalog .group").first
            await first_card.click()
            await page.wait_for_selector("text=SPECIFICATION //")
            print("Modal opened successfully!")

            modal_path = os.path.join(OUTPUT_DIR, "holtlab_te_modal.png")
            await page.screenshot(path=modal_path)
            print(f"Modal screenshot saved to {modal_path}")

            # Close modal
            close_btn = page.locator("button[aria-label='Close Modal']")
            await close_btn.click()
            await page.wait_for_timeout(300)
            print("Modal closed.")

            # Test Category Filter: click 'keyrings'
            keyrings_btn = page.locator("button:has-text('keyrings')").first
            await keyrings_btn.click()
            await page.wait_for_timeout(300)
            cards_count = await page.locator("#catalog .group").count()
            print(f"Cards count after filtering to keyrings: {cards_count}")

            # Reset filter: click 'all hardware'
            all_btn = page.locator("button:has-text('all hardware')").first
            await all_btn.click()
            await page.wait_for_timeout(300)
            cards_count_all = await page.locator("#catalog .group").count()
            print(f"Cards count after resetting to all: {cards_count_all}")
            assert cards_count_all == 8, f"Expected 8 cards, got {cards_count_all}"

            # Test Inquiry Form Submission
            name_input = page.locator("input[placeholder*='acme robotics']")
            await name_input.fill("Holt Robotics Corp")

            email_input = page.locator("input[placeholder='name@domain.com']")
            await email_input.fill("inquiry@acmerobotics.com")

            timeline_input = page.locator("input[placeholder*='4-6 weeks']")
            await timeline_input.fill("4 weeks to prototype")

            desc_input = page.locator("textarea[placeholder*='describe dimensions']")
            await desc_input.fill("Need turnkey 4-layer PCBA and MJF enclosure for custom handheld.")

            submit_btn = page.locator("button[type='submit']")
            await submit_btn.click()
            await page.wait_for_selector("text=DISPATCH BUFFER READY")
            print("Inquiry dispatch buffer confirmed!")

            # Test Mobile Viewport
            mobile_page = await browser.new_page(viewport={"width": 375, "height": 812})
            await mobile_page.goto(url, wait_until="networkidle")
            await mobile_page.wait_for_selector("header")
            mobile_path = os.path.join(OUTPUT_DIR, "holtlab_te_mobile.png")
            await mobile_page.screenshot(path=mobile_path)
            print(f"Mobile screenshot saved to {mobile_path}")

            # Open mobile drawer
            menu_btn = mobile_page.locator("button[aria-label='Toggle Navigation Menu']")
            await menu_btn.click()
            await mobile_page.wait_for_selector("text=all hardware")
            mobile_drawer_path = os.path.join(OUTPUT_DIR, "holtlab_te_mobile_drawer.png")
            await mobile_page.screenshot(path=mobile_drawer_path)
            print(f"Mobile drawer screenshot saved to {mobile_drawer_path}")

            await browser.close()

        # Stitch Side-by-Side Comparison
        if os.path.exists(REF_IMG) and os.path.exists(desktop_path):
            ref_img = Image.open(REF_IMG).convert("RGB")
            new_img = Image.open(desktop_path).convert("RGB")

            # Resize or crop both to 1440x900 for exact alignment
            ref_w, ref_h = ref_img.size
            new_w, new_h = new_img.size

            target_h = min(ref_h, new_h, 900)
            ref_cropped = ref_img.crop((0, 0, 1440, target_h))
            new_cropped = new_img.crop((0, 0, 1440, target_h))

            combined = Image.new("RGB", (2880, target_h))
            combined.paste(ref_cropped, (0, 0))
            combined.paste(new_cropped, (1440, 0))

            sbs_path = os.path.join(OUTPUT_DIR, "side_by_side_comparison.png")
            combined.save(sbs_path)
            print(f"Side-by-side comparison saved to {sbs_path}")

    finally:
        server_proc.terminate()
        print("Server terminated.")

if __name__ == "__main__":
    asyncio.run(run_e2e())
