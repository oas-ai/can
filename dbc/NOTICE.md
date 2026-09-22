# Hyundai Palisade 2020 read-only DBC subset

`hyundai_palisade_2020_read_only.dbc`는 commaai/opendbc의
`opendbc/dbc/generator/hyundai/hyundai_can.dbc`에서 OAS canonical state 및 DBC 기반
raw diagnostics에 사용하는 read-only 메시지만 추린 파생물입니다.

- Upstream revision: `ba800b77187208286f39c22f692ac7c69e9324f2`
- Upstream file SHA-256: `a3fb6e98bfdc0041914643f1422b1bb9290462981c40249a05cd7773e71a8bc3`
- License: MIT, Copyright (c) 2020 Comma.ai, Inc.
- Source: <https://github.com/commaai/opendbc/blob/ba800b77187208286f39c22f692ac7c69e9324f2/opendbc/dbc/generator/hyundai/hyundai_can.dbc>

이 파일과 decoder는 수신 전용이다. upstream의 차량 제어·송신 코드는 가져오지 않는다.
차량 연식·시장·트림에서 frame과 signal을 실차 로그로 확인하기 전에는 새 신호를
canonical state에 추가하지 않는다.
