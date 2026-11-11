//! ## Overview
//! Korean localized layer specifications and telemetry strings for the anatomical teardown section.
//! Defines exact specifications, materials, and engineering details in Korean.
//!
//! ## Search Tags
//! #i18n, #teardown, #korean, #telemetry, #specifications
//!
//! ## Submodules
//! None. This module provides atomic localized Korean records.

use super::{TeardownLayerData, TeardownSectionStrings};

/// Returns Korean section-level headings and exploded banner strings.
pub fn get_section_strings() -> TeardownSectionStrings {
    TeardownSectionStrings {
        eyebrow: "SUBSYSTEM TEARDOWN // 내부 계층 분석",
        title: "4계층 하드웨어 · 펌웨어 설계 해부",
        subtitle: "기구 외함, 4층 기판, 실시간 펌웨어, 휴먼 인터페이스 상세 설계 내역.",
        exploded_eyebrow: "내부 아키텍처 구조",
        exploded_title: "외함부터 기판과 펌웨어까지, 4계층 내부 설계 구조",
        callouts: [
            ("TOP", "PA12 전면 베젤 및 강화유리"),
            ("MID-1", "2.0\" IPS 및 CST816D 터치"),
            ("MID-2", "4층 PCBA 및 ESP32-C3"),
            ("BASE", "LiPo 배터리, 모터 및 후면 외함"),
        ],
        bottom_strip: [
            "파라메트릭 3D B-REP 솔리드 모델링",
            "실제 부품 3D STEP 조립 유격 검증",
            "HP MJF PA12 나일론 & 4층 SMT PCBA",
        ],
    }
}

fn layer_0_ko() -> TeardownLayerData {
    TeardownLayerData {
        id: 0,
        number: "01",
        title: "MCAD 기구 외함",
        subtitle: "HP 멀티 젯 퓨전 PA12 나일론 및 정밀 단차 베젤 구조",
        image_src: "/assets/showcase/mcad_drawing.png",
        image_alt: "MCAD 기구 설계 도면 및 와이어프레임",
        verification_badge: "3D 조립 유격 및 체결 검증",
        primary_spec: "43.40 × 69.20 × 13.78 mm 외형 규격",
        secondary_spec: "1.20 mm 공칭 벽두께 // 0.85 mm 단차 베젤",
        bus_or_material: "HP MJF PA12 + SLA 투명 라이트파이프",
        deep_summary: "파라메트릭 3D 솔리드 모델링을 통해 실제 조달 부품(디스플레이, 배터리, 스위치, 커넥터)의 3D STEP 형상과 체결 유격을 정밀하게 반영했습니다. ±1.50 mm 스트로크를 갖는 이탈 방지형 T-슬라이더 전원 스위치 캡과 일체형 스트랩 러그를 내장했습니다.",
        bullet_1: "코너 단차와 M2 접시머리 볼트 체결 보어가 적용된 일체형 후면 쉘",
        bullet_2: "디스플레이 밀착형 0.85 mm 단차 릴리프로 유리 돌출을 억제한 플러시 전면 베젤",
        bullet_3: "초슬림 LiPo 배터리 및 1027 코인 진동 모터를 안정적으로 고정하는 내부 크래들",
    }
}

fn layer_1_ko() -> TeardownLayerData {
    TeardownLayerData {
        id: 1,
        number: "02",
        title: "ECAD 4층 회로 기판",
        subtitle: "4층 제어 임피던스 적층 및 고속 신호 무결성 설계",
        image_src: "/assets/showcase/ecad_pcb.png",
        image_alt: "ECAD 4층 라우팅 PCB 어셈블리",
        verification_badge: "0 ERC · 0 DRC 회로 검증",
        primary_spec: "40.00 × 64.60 × 1.00 mm 4층 기판",
        secondary_spec: "50Ω 싱글엔디드 // 90Ω 차동 USB 임피던스",
        bus_or_material: "전면신호 / 내층1 GND / 내층2 +3V3 / 후면신호",
        deep_summary: "제어 임피던스 적층 구조를 적용한 4층 맞춤형 기판입니다. 4MB 플래시 내장 ESP32-C3 무선 모듈, TP4054 100mA 리튬폴리머 충전 IC, ME6211 3.3V LDO, DW01A/8205A 2.45V 과방전 하드웨어 보호 회로를 집약했으며, 모듈 내부 499Ω 직렬 저항에 따른 고속 SPI 신호 지연을 분석해 핀 배치를 최적화했습니다.",
        bullet_1: "연속 신호 귀환 경로와 저노이즈 특성을 확보하는 솔리드 내층 기준 평면 (GND/+3V3)",
        bullet_2: "전용 TVS 정전기 보호 소자가 적용된 하드웨어 USB-C 2.0 풀스피드 D+/D- 인터페이스",
        bullet_3: "RC 디바운스 리셋 회로 및 계측·브링업용 표면 실장 테스트 패드 배치",
    }
}

fn layer_2_ko() -> TeardownLayerData {
    TeardownLayerData {
        id: 2,
        number: "03",
        title: "임베디드 펌웨어",
        subtitle: "3-태스크 비동기 멀티태스킹 & AHB GDMA 그래픽 파이프라인",
        image_src: "/assets/showcase/firmware_logic.png",
        image_alt: "임베디드 펌웨어 아키텍처 맵",
        verification_badge: "제로카피 60Hz DMA 구동",
        primary_spec: "AHB GDMA CH0 제로카피 파이프라인",
        secondary_spec: "0.79 ms 타일 갱신 // 47.04 ms 전체 프레임",
        bus_or_material: "하드웨어 SPI2 @ 26.67 MHz (80MHz/3)",
        deep_summary: "동적 힙 메모리 할당(0 alloc)을 배제한 베어메탈 비동기 멀티태스킹 아키텍처로 장시간 구동 시에도 메모리 단편화나 런타임 오류 없이 안정적으로 동작합니다. 153.6 KB 정적 SRAM 프레임버퍼와 AHB GDMA 채널을 직결해 CPU 개입을 최소화하며 터치 입력, 상태 머신, 60Hz 화면 갱신을 동시에 처리합니다.",
        bullet_1: "제로 힙 할당: 디스플레이 해상도에 1:1 매핑된 153.6 KB 정적 SRAM 백버퍼",
        bullet_2: "평시 0Hz WFI 대기 상태에서 즉시 코어를 깨우는 GPIO6 하강 에지 터치 인터럽트",
        bullet_3: "20초 무입력 시 자동 밝기 디밍 및 60초 경과 시 디스플레이 SLPIN 절전 진입",
    }
}

fn layer_3_ko() -> TeardownLayerData {
    TeardownLayerData {
        id: 3,
        number: "04",
        title: "터치 인터페이스 · 햅틱",
        subtitle: "CST816D 정전식 터치 및 3.0V 정전압 보정 진동 피드백",
        image_src: "/assets/showcase/mcad_iso.png",
        image_alt: "휴먼 인터페이스 및 햅틱 시스템 아이소메트릭 뷰",
        verification_badge: "정전압 PWM 햅틱 제어",
        primary_spec: "2.0\" IPS 240×320 + CST816D 터치",
        secondary_spec: "AO3400A NMOS 구동 1027 코인 진동 모터",
        bus_or_material: "I2C 패스트모드 400kHz + 24kHz LEDC 백라이트",
        deep_summary: "단차 없는 전면 글래스와 즉각적인 정전식 터치 응답, 균일한 햅틱 클릭감을 결합했습니다. 400kHz I2C 인터페이스와 저지터 인터럽트로 터치 좌표를 수집하며, 배터리 잔량이나 USB 전원 연결 여부에 관계없이 일정한 3.0V 실효 진동 세기를 유지하도록 피드포워드 PWM 듀티 보정을 적용했습니다.",
        bullet_1: "소형 화면에서의 오터치를 줄이는 9×9 커서 클램핑 및 14px 데드존 트랙패드 UI",
        bullet_2: "전원 전압 실시간 계측 기반 피드포워드 PWM 보정으로 모터 과전압 및 클릭감 편차 방지",
        bullet_3: "가청 주파수 대역을 벗어난 24kHz 고주파 LEDC 백라이트 제어로 코일 노이즈 제거",
    }
}

/// Returns the 4 Korean teardown layers.
pub fn get_layers() -> [TeardownLayerData; 4] {
    [
        layer_0_ko(),
        layer_1_ko(),
        layer_2_ko(),
        layer_3_ko(),
    ]
}
