# Holt Lab Agent Guide & Standards (AGENTS.md)

This file defines the project-specific coding guidelines, industrial design architecture, and verification standards for the Holt Lab studio landing page (`holt-lab.github.io`).

---

## 1. Architectural Baseline: Agent-Native Core
This project strictly adheres to the compiler-enforced **Agent-Native** architecture.

👉 **Mandatory Baseline Reading**: Consult [AGENT_NATIVE.md](file://AGENT_NATIVE.md) for the authoritative specification on:
- **Core Philosophy**: Living LLM-Wiki (`mod.rs` = `index.md`), compile-checked intra-doc links, and living doctests.
- **Compiler Hard Limits**: Rules 1-4 enforced via `build.rs` and `build_linter.rs` (file headers, logical code limits, inline test limits, function/component physical limits).
- **Anti-Code-Golfing Principle**: Decompose into cohesive submodules; never compress variable names or eliminate idiomatic whitespace.
- **Safety Lints**: `missing_docs = "deny"`, `clippy::unwrap_used = "deny"`, `clippy::expect_used = "deny"` in `Cargo.toml`.
- **Documentation Language**: All Rust documentation (`//!`, `///`) and comments (`//`, `/* */`) must be written strictly in concise English.

---

## 2. System Architecture & Design System

Holt Lab is a boutique hardware and embedded engineering studio landing page built with Rust, Dioxus 0.7 (WebAssembly), and Tailwind CSS v4.

### Module Domains
The root crate index (`src/main.rs`) organizes the codebase into:
1. **`src/views/` (`src/views/mod.rs`)**:
   - High-level route views, primarily `home.rs`.
2. **`src/components/` (`src/components/mod.rs`)**:
   - Modular UI components:
     - `navbar.rs`: Sticky navigation bar with telemetry pill and mobile drawer.
     - `hero.rs`: Primary value proposition, headline, and hardware bench display.
     - `metrics.rs`: Key engineering metrics grid (Unified Turnkey, 1:1 Direct, 3~4 Weeks, 100% Audit).
     - `comparison.rs`: Architecture comparison between legacy fragmented outsourcing and Holt Lab turnkey.
     - `showcase/`: In-depth engineering case study (MCAD, ECAD, Firmware) for the Win95 Minesweeper Touch Keyring.
     - `services.rs`: 4 core capability cards (ECAD, MCAD, Firmware, Factory & Assembly).
     - `process.rs`: 4-stage engineering verification protocol (Gate 0 Lock to Gate 2 DFM).
     - `contact/`: Diagnostic interactive RFQ form and SLA / grant compliance assurances.
     - `footer.rs`: Brand footer, copyright, and channel links.

### Visual Aesthetic: Teenage Engineering / Dieter Rams
- **Canvas & Backgrounds**: Off-white / light matte aluminum / pure white (`#FAFAFA`, `#F4F4F5`, `#FFFFFF`).
- **Typography**: Deep charcoal / ink black (`#18181B`, `#09090B`), Swiss typography, generous whitespace.
- **Accent**: Single restrained matte vermilion orange (`#EA580C` / `#E64A19`) reserved for primary actions (e.g. Header CTA and Inquiry).
- **Clutter Elimination**: Zero blueprint CAD grids, zero `+` crosshairs, zero glowing neon indicators, zero fake decorative tags.
- **Showcase Bench**: Unified precision viewports that cleanly integrate both dark 3D renders and pure white 2D CAD drawings on a light canvas.

---

## 3. Verification Protocol
Always verify code changes before completing any task:

```bash
# 1. Check compiler linter rules and Agent-Native constraints
cargo check

# 2. Compile Tailwind CSS
npx @tailwindcss/cli -i input.css -o assets/tailwind.css
cp assets/tailwind.css public/assets/tailwind.css

# 3. Build WebAssembly release bundle
dx build --web --release
```
