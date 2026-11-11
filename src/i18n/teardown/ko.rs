//! ## Overview
//! Korean localized layer specifications and telemetry strings for the anatomical teardown section.
//! Defines exact specifications, materials, and engineering achievements in Korean.
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
        eyebrow: "엔지니어링 쇼케이스 // 케이스 스터디 01",
        title: "win95 minesweeper touch keyring",
        subtitle: "43mm 포켓 풋프린트에 60Hz 터치 콘솔을 집약한 턴키 엔지니어링 실증.",
        exploded_eyebrow: "해부학적 구조",
        exploded_title: "순수 구리 패턴까지 완벽히 분해하다.",
        callouts: [
            ("TOP", "PA12 전면 베젤 및 강화유리"),
            ("MID-1", "2.0\" IPS 및 CST816D 터치"),
            ("MID-2", "4층 PCBA 및 ESP32-C3"),
            ("BASE", "LiPo 배터리, 모터 및 후면 외함"),
        ],
        bottom_strip: [
            "파라메트릭 CAD // OPENCASCADE B-REP",
            "23개 결합 검증 // 완벽한 무간섭 설계",
            "정밀 MJF 나일론 & 산업용 SMT PCBA",
        ],
    }
}

fn layer_0_ko() -> TeardownLayerData {
    TeardownLayerData {
        id: 0,
        number: "01",
        title: "MCAD 정밀 기구 외함",
        subtitle: "HP 멀티 젯 퓨전 PA12 나일론 및 정밀 무간섭 기구 결합",
        image_src: "assets/showcase/mcad_drawing.png",
        image_alt: "MCAD 기구 설계 도면 및 와이어프레임",
        verification_badge: "게이트 1 // 3D 무간섭 체적 검증 통과",
        primary_spec: "43.40 × 69.20 × 13.78 mm 외형 규격",
        secondary_spec: "1.20 mm 공칭 벽두께 // 0.85 mm 단차 베젤",
        bus_or_material: "HP MJF PA12 + SLA 8001 투명 라이트파이프",
        deep_summary: "build123d OpenCascade B-Rep 커널 기반 파라메트릭 기구 설계. 제조사 COTS 3D STEP 모델과 23개 쌍별 체적 검사를 수행하여 완벽한 무간섭 일체형 설계를 달성했습니다. ±1.50 mm 스트로크의 캡티브 T-슬라이더 스위치 캡 및 스트랩 홀 일체형 설계를 적용했습니다.",
        bullet_1: "코너 단차와 M2 접시머리 볼트 체결 보어가 적용된 일체형 후면 쉘",
        bullet_2: "디스플레이 밀착형 0.85 mm 단차 릴리프로 유리 단차가 없는 플러시 전면 베젤",
        bullet_3: "1.50 mm 초슬림 LiPo 배터리 및 1027 코인 햅틱 진동 모터 전용 크래들 내장",
    }
}

fn layer_1_ko() -> TeardownLayerData {
    TeardownLayerData {
        id: 1,
        number: "02",
        title: "ECAD 4층 회로 기판",
        subtitle: "산업용 4층 임피던스 제어 적층 및 499Ω 댐핑 RC 지연 보정",
        image_src: "assets/showcase/ecad_pcb.png",
        image_alt: "ECAD 4층 라우팅 PCB 어셈블리",
        verification_badge: "게이트 2 // 0 ERC · 0 DRC 검증 완료",
        primary_spec: "40.00 × 64.60 × 1.51 mm 4층 기판",
        secondary_spec: "50Ω 싱글엔디드 // 90Ω 차동 USB 임피던스",
        bus_or_material: "전면신호 / 내층1 GND / 내층2 +3V3 / 후면신호",
        deep_summary: "KiCad 9 기반 제어 임피던스 라우팅 설계. 4MB 플래시 ESP32-C3-WROOM-02-N4, TP4054 100mA 안전 LiPo 충전기, ME6211 3.3V 500mA LDO, DW01A/8205A 2.45V 하드웨어 보호 회로를 완벽 탑재했습니다. 모듈 내부 499Ω 댐핑 저항의 RC 신호 지연을 정밀 보정했습니다.",
        bullet_1: "연속 귀환 경로와 극저노이즈를 보장하는 전면 솔리드 내층 기준 평면 (GND/3V3)",
        bullet_2: "전용 TVS 정전기 방전 보호가 적용된 하드웨어 USB-C 2.0 풀스피드 D+/D-",
        bullet_3: "RC 디바운스 및 표면 실장 테스트 패드가 구비된 액티브 로우 리셋 회로",
    }
}

fn layer_2_ko() -> TeardownLayerData {
    TeardownLayerData {
        id: 2,
        number: "03",
        title: "고신뢰성 임베디드 펌웨어 엔진",
        subtitle: "esp-hal 1.2.2 + embassy 3-태스크 비동기 액터 아키텍처",
        image_src: "assets/showcase/firmware_logic.png",
        image_alt: "고신뢰성 임베디드 펌웨어 아키텍처 맵",
        verification_badge: "벤치 // 제로카피 60Hz DMA 검증 통과",
        primary_spec: "AHB GDMA CH0 제로카피 파이프라인",
        secondary_spec: "0.79 ms 타일 렌더 // 47.04 ms 전체 프레임",
        bus_or_material: "하드웨어 SPI2 @ 26.67 MHz (80MHz/3)",
        deep_summary: "esp-hal 및 embassy 비동기 런타임 기반 순수 #![no_std] 고신뢰성 임베디드 펌웨어. 153.6 KB SRAM 백버퍼를 통해 렌더링 중 힙 할당(0 alloc)을 완전히 배제하여 런타임 패닉과 메모리 충돌을 원천 차단했습니다. 정전식 터치 폴링, Windows 95 상태 머신 전환, 비동기 GDMA 전송을 3개 태스크로 동시 제어합니다.",
        bullet_1: "제로 힙 할당: 디스플레이 해상도에 1:1 매핑된 153.6 KB SRAM 직접 백버퍼",
        bullet_2: "0Hz WFI 절전 모드에서 코어를 즉각 기상시키는 하강 에지 터치 인터럽트 (GPIO6)",
        bullet_3: "20초 무입력 시 LEDC 자동 디밍 및 60초 경과 시 초절전 SLPIN 대기 모드 진입",
    }
}

fn layer_3_ko() -> TeardownLayerData {
    TeardownLayerData {
        id: 3,
        number: "04",
        title: "휴먼 인터페이스 및 햅틱 시스템",
        subtitle: "CST816D 정전식 터치 및 3.0V 정전압 공진 피드백",
        image_src: "assets/showcase/mcad_iso.png",
        image_alt: "휴먼 인터페이스 및 햅틱 시스템 아이소메트릭 뷰",
        verification_badge: "원격측정 // 피드포워드 PWM 제어 통과",
        primary_spec: "2.0\" IPS 240×320 + CST816D 터치",
        secondary_spec: "AO3400A NMOS 구동 1027 코인 진동 모터",
        bus_or_material: "I2C 패스트모드 400kHz + 24kHz LEDC 백라이트",
        deep_summary: "완벽한 단차 없는 글래스 전면과 즉각적인 반응의 정전식 터치, 정교한 기계적 클릭감. 400kHz I2C 패스트모드와 저지터 하강 에지 인터럽트로 터치 좌표를 샘플링합니다. AO3400A NMOS 기반 피드포워드 PWM 듀티 보정을 통해 배터리나 USB 전원 상태에 상관없이 균일한 3.0V RMS 햅틱 클릭을 제공합니다.",
        bullet_1: "9×9 클램핑 바운드 및 14px 데드존 필터링이 적용된 듀얼 개구부 트랙패드 UI",
        bullet_2: "피드포워드 PWM 전압 보정: D_max = min(1.0, 3.0 / V_sys)로 모터 과부하 및 편차 방지",
        bullet_3: "고주파 24kHz LEDC PWM 백라이트 디밍으로 가청 대역 인덕터 코일 노이즈 원천 차단",
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
