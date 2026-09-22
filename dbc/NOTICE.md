# Genesis G80 2017 read-only DBC subset

`genesis_g80_2017_read_only.dbc`는 commaai/opendbc의
`opendbc/dbc/generator/hyundai/hyundai_can.dbc`에서 OAS canonical state에 사용하는
read-only 메시지만 추린 파생물입니다.

- Upstream revision: `4ad6045b2cd19c0c9adc0c2cd79bf96c8bda6e5d`
- Upstream file SHA-256: `a3fb6e98bfdc0041914643f1422b1bb9290462981c40249a05cd7773e71a8bc3`
- License: MIT, Copyright (c) 2020 Comma.ai, Inc.
- Source: <https://github.com/commaai/opendbc/blob/4ad6045b2cd19c0c9adc0c2cd79bf96c8bda6e5d/opendbc/dbc/generator/hyundai/hyundai_can.dbc>

이 파일과 decoder는 수신 전용이다. upstream의 차량 제어·송신 코드는 가져오지 않는다.
차량 연식·시장·트림에서 frame과 signal을 실차 로그로 확인하기 전에는 새 신호를
canonical state에 추가하지 않는다.
