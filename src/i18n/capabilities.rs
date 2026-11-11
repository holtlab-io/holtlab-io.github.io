//! ## Overview
//! Localized data models and engineering descriptions for the 4 core studio capabilities.
//! Covers mechanical enclosures, custom multi-layer PCBA, embedded firmware, and IoT server/web integration.
//!
//! ## Search Tags
//! #i18n, #capabilities, #disciplines, #mcad, #ecad, #firmware, #iot
//!
//! ## Submodules
//! None. This module provides atomic presentation text models.

use super::Language;

/// Single discipline capability card data model.
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct CapabilityCard {
    /// Discipline numerical indicator (e.g. 01).
    pub number: &'static str,
    /// Technical domain badge tag.
    pub tag: &'static str,
    /// Discipline title heading.
    pub title: &'static str,
    /// Detailed capability description.
    pub desc: &'static str,
    /// Key engineering scope summary.
    pub metric: &'static str,
}

/// Localized string bundle for the studio engineering capabilities section.
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct CapabilitiesStrings {
    /// Top eyebrow badge text.
    pub eyebrow: &'static str,
    /// Section main title.
    pub title: &'static str,
    /// Engineering subtitle paragraph.
    pub subtitle: &'static str,
    /// 4 core engineering discipline cards.
    pub cards: [CapabilityCard; 4],
}

fn cap_en_01_02() -> [CapabilityCard; 2] {
    [
        CapabilityCard {
            number: "01",
            tag: "MCAD · 3D SOLID MODELING",
            title: "Mechanical CAD & Custom Enclosures",
            desc: "Custom enclosures and internal mounting structures modeled around actual vendor 3D STEP components. Pre-fabrication clearance and assembly checks reduce mechanical fit issues across MJF/SLA 3D printing, CNC machining, and injection molding.",
            metric: "3D Clearance Check · MJF / CNC / Molding",
        },
        CapabilityCard {
            number: "02",
            tag: "ECAD · CIRCUIT & MULTI-LAYER PCB",
            title: "Custom Electronics & Multi-Layer PCBA",
            desc: "Application-specific schematics covering sensor front-ends, Li-ion power management and protection, ESD safety, and wireless RF stages. Multi-layer controlled-impedance PCB layout paired with live BOM sourcing and SMT assembly coordination.",
            metric: "Power, RF & Sensors · Multi-Layer SMT PCBA",
        },
    ]
}

fn cap_en_03_04() -> [CapabilityCard; 2] {
    [
        CapabilityCard {
            number: "03",
            tag: "FIRMWARE · RTOS & BARE-METAL",
            title: "Embedded Firmware & Board Bring-Up",
            desc: "Deterministic bare-metal and RTOS firmware for sensor acquisition, DMA peripheral pipelines, and ultra-low-power battery operation. Co-engineered alongside the schematic to prevent pin-mux conflicts and streamline physical board bring-up.",
            metric: "RTOS / Bare-Metal · Low-Power & Bring-Up",
        },
        CapabilityCard {
            number: "04",
            tag: "CONNECTED IOT · BACKEND & WEB",
            title: "IoT Backend Server & Web Integration",
            desc: "End-to-end connectivity from device to cloud — custom backend API servers, MQTT/HTTP/WebSocket telemetry pipelines, device provisioning, OTA firmware updates, and responsive web dashboards for real-time monitoring and control.",
            metric: "Custom API Server · Web Dashboard · OTA",
        },
    ]
}

fn cap_ko_01_02() -> [CapabilityCard; 2] {
    [
        CapabilityCard {
            number: "01",
            tag: "MCAD · 3D 솔리드 모델링",
            title: "기구 설계 및 맞춤형 외함 엔지니어링",
            desc: "실제 조달할 전자 부품과 커넥터·배터리의 3D STEP 모델을 기준으로 외함과 내부 체결 구조를 설계합니다. 제작 전 부품 간 물리적 유격과 조립성을 시뮬레이션하며, 산업용 3D 프린팅(MJF·SLA), CNC 가공, 사출 성형 등 수량에 맞는 공정을 적용합니다.",
            metric: "3D 유격 검증 · MJF / CNC / 사출",
        },
        CapabilityCard {
            number: "02",
            tag: "ECAD · 회로 및 다층 PCB",
            title: "맞춤형 전자 회로 및 다층 기판(PCBA)",
            desc: "센서 인터페이스, 리튬 배터리 충·방전 및 하드웨어 보호 회로, 전원 레귤레이션, 무선 통신(BLE·Wi-Fi·LTE) 회로를 목적에 맞게 설계합니다. 노이즈 저감을 고려한 다층 기판 레이아웃부터 부품 수급(BOM) 확인 및 SMT 실장 발주까지 일괄 수행합니다.",
            metric: "전원·무선·센서 회로 · 다층 제어 임피던스 PCB",
        },
    ]
}

fn cap_ko_03_04() -> [CapabilityCard; 2] {
    [
        CapabilityCard {
            number: "03",
            tag: "FIRMWARE · RTOS & 실시간 제어",
            title: "고신뢰성 임베디드 펌웨어 및 보드 브링업",
            desc: "RTOS 및 베어메탈 환경에서 정밀 센서 수집, 고속 DMA 디스플레이·주변장치 제어, 초저전력 배터리 슬립 운용 등 하드웨어 성능을 끌어내는 펌웨어를 개발합니다. 회로 설계 단계부터 핀맵과 타이밍 제약을 함께 조율하여 안정적인 실기 보드 브링업을 완성합니다.",
            metric: "RTOS / 베어메탈 제어 · 초저전력 설계 및 브링업",
        },
        CapabilityCard {
            number: "04",
            tag: "IOT · 백엔드 & 웹 관제",
            title: "IoT 백엔드 서버 구축 및 클라우드·웹 연동",
            desc: "단독 동작 기기를 넘어 서버 연동이 필요한 제품을 위해 디바이스 통신 프로토콜(MQTT·HTTP·WebSocket·BLE), 맞춤형 백엔드 API 서버 및 데이터베이스 구축, 실시간 기기 상태 관제 웹 대시보드, 원격 펌웨어 업데이트(OTA) 파이프라인까지 통합 구축합니다.",
            metric: "맞춤형 API 서버 · 관제 웹 · OTA",
        },
    ]
}

/// Returns the studio engineering capabilities string bundle for the requested language.
pub fn get_capabilities_strings(lang: Language) -> CapabilitiesStrings {
    match lang {
        Language::En => {
            let p1 = cap_en_01_02();
            let p2 = cap_en_03_04();
            CapabilitiesStrings {
                eyebrow: "Core Capabilities · Engineering Domains",
                title: "Four Pillars of Full-Stack Hardware Engineering",
                subtitle: "Seamless execution from standalone physical devices to cloud-connected IoT systems.",
                cards: [p1[0], p1[1], p2[0], p2[1]],
            }
        }
        Language::Ko => {
            let p1 = cap_ko_01_02();
            let p2 = cap_ko_03_04();
            CapabilitiesStrings {
                eyebrow: "핵심 역량 · 통합 엔지니어링 도메인",
                title: "기구 · 회로 · 펌웨어 · 서버를 아우르는 4대 핵심 역량",
                subtitle: "하드웨어 단품 시제품부터 서버와 연동되는 커넥티드 IoT 시스템까지 분절 없이 설계합니다.",
                cards: [p1[0], p1[1], p2[0], p2[1]],
            }
        }
    }
}
