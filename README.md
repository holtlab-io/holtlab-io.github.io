# Holt Lab — Official Studio Hub

> **"From Silicon to Enclosure to Screen."**  
> Hardware, Mechanical (MCAD), Electronics (ECAD) & Firmware Engineering Studio.

[![GitHub Pages Deployment](https://github.com/holtlab-io/holtlab-io.github.io/actions/workflows/deploy.yml/badge.svg)](https://github.com/holtlab-io/holtlab-io.github.io/actions/workflows/deploy.yml)
[![Live Website](https://img.shields.io/badge/Live_Site-holtlab--io.github.io-orange)](https://holtlab-io.github.io)

---

## ⚡ What is Holt Lab?

Holt Lab is an independent hardware & embedded engineering studio specializing in **Zero-Pingpong Turnkey Hardware Development**.  
We bridge the fatal gap between mechanical enclosures, high-density multi-layer PCBs, and low-latency embedded firmware.

- **Interference-Free Assembly Guarantee**: Co-designed in OpenCascade B-Rep 3D solid kernel and KiCad 9 3D STEP models.
- **Unified Turnkey Co-Design**: Eliminates cross-vendor ping-pong between PCB fabricators, mechanical molders, and firmware engineers.
- **Rapid PoC Turnaround**: 3–4 weeks from initial schematic to working physical prototypes in hand.
- **Government Grant Compliant**: 100% compliant electronic contracts, tax invoices, and milestone acceptance documentation for startup grants (예창패·청창사·초창패).

---

## 🛠️ Unified Engineering Stack

| Domain | Tools & Standards | Scope |
| :--- | :--- | :--- |
| **ECAD (Electronics)** | KiCad 9, FreeRouting, Precision SMT | 4~6 Layer Controlled Impedance, High-Speed SPI/RF, Battery PCM |
| **MCAD (Mechanical)** | build123d (OpenCascade B-Rep), Industrial MJF | HP MJF Nylon PA12, SLA Resin, CNC 6061 Aluminum, Interference-Free Fit |
| **Firmware & Systems** | Rust (`esp-hal`, `embassy`), C/C++, ESP-IDF | 26.67MHz Zero-Copy AHB GDMA, 60Hz UI, 3.3V Low-Power Sleep |
| **Web Infrastructure** | Dioxus 0.7 WebAssembly, Tailwind CSS | High-Performance Studio Showcase deployed on GitHub Pages |

---

## 🚀 Flagship Reference Case: Win95 Minesweeper Touch Keyring

- **Dimensions**: 43.40 × 69.20 × 13.78 mm (MJF Nylon PA12 Enclosure)
- **Circuitry**: 4-Layer Controlled-Impedance PCB (40.0 × 64.6 mm, Industrial Controlled-Impedance Stackup)
- **SoC**: ESP32-C3-WROOM-02-N4 (RISC-V 160MHz) + 2.0" ST7789T3 IPS (240×320) + CST816D Capacitive Touch
- **Power**: TW301525 Li-Po + DW01A/8205A 2.45V Hardware PCM Protection + TP4056
- **Performance**: 26.67MHz SPI Bus, 0.79ms Tile Update, FallingEdge Hardware IRQ

---

## 📬 Contact & Inquiries

- **Official Email**: [contact@holtlab.io](mailto:contact@holtlab.io)
- **YouTube Devlog**: [@holt-lab](https://youtube.com/@holt-lab)
- **GitHub**: [github.com/holtlab-io](https://github.com/holtlab-io)

---
*© 2026 Holt Lab. Built with Rust & Dioxus.*
