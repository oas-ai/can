# Hyundai Palisade 2020 Legacy Decoder

`HyundaiPalisade2020Decoder`는 `dbc` 저장소의 `HYUNDAI_PALISADE_2020` provenance manifest에
고정된 Hyundai legacy CAN DBC 중 상태에 필요한 일곱 메시지만 직접 해석합니다.

| DBC message | 신호 | OAS 용도 |
| --- | --- | --- |
| `CLU11` | `CF_Clu_Vanz`, `CF_Clu_SPEED_UNIT` | 클러스터 속도와 단위 |
| `SAS11` | `SAS_Angle` | 조향각 |
| `TCS13` | `ACCEL_REF_ACC`, `DriverOverride` | 종가속도와 운전자 제동 상태 |
| `CGW1` (`0x541`) | `CF_Gway_HeadLampLow` | 저빔 기반 `night_mode` |
| `LVR12` (`0x367`) | `CF_Lvr_Gear` | 선택 레버 기어 상태 |
| `WHL_SPD11` (`0x386`) | `WHL_SPD_FL/FR/RL/RR` | 바퀴별 속도 |
| `SCC14` (`0x389`) | `ACCMode` | SCC 활성 상태 |

이는 전체 DBC parser가 아니라 승인된 schema의 좁은 read-only 구현이다. 알 수 없는
frame은 정상적으로 무시하고, 알려진 메시지의 payload 길이가 다르면 오류를 반환한다.

사용하는 DBC는 opendbc의 MIT 라이선스 `hyundai_can.dbc`를 특정 커밋으로 고정한
[read-only subset](../dbc/hyundai_palisade_2020_read_only.dbc)이다. 원본 revision·hash·고지는
[NOTICE](../dbc/NOTICE.md)에 있다.

## 실차 신호 검증

`CF_Gway_HeadLampLow`는 저빔 점등 상태만 나타낸다. 자동 조명 설정이나 주변 조도는
추론하지 않는다. 실차에서 매핑을 승인하기 전에는 정차·시동 ON 상태에서 다음을 기록한다.

| 조건 | 기대 `0x541` bit 31 | 기대 `night_mode` |
| --- | --- | --- |
| 라이트 OFF | `0` | `false` |
| 저빔 ON | `1` | `true` |
| AUTO 모드 | 실제 저빔 점등 여부와 동일 | 실제 저빔 점등 여부와 동일 |

Canable가 SocketCAN `can0`으로 연결된 뒤, 수신 전용으로 timestamp 포함 로그를 남긴다.

```sh
ip -details link show can0
candump -L -t a can0,541:7FF > palisade-cgw1-lamps.log
```

각 조건을 바꿀 때 시각과 관찰한 램프 상태를 별도로 기록하고, 로그의 payload 네 번째
byte의 최상위 bit가 표와 일치하는지 확인한다. 불일치하면 이 매핑을 배포하지 않고 해당
DBC와 차량 연식·트림을 다시 확인한다.

캡처는 물리 `can0`으로 재송신하지 않는다. 개발 환경에서는 `vcan0`에만 재생한다.

```sh
sudo modprobe vcan
sudo ip link add dev vcan0 type vcan
sudo ip link set vcan0 up
canplayer -I palisade-cgw1-lamps.log vcan0=can0
```

회귀 fixture에는 차량 식별 정보·위치·시간 등 민감한 원본 로그를 넣지 말고, 검증된
`0x541` frame과 필요한 최소 상태 frame만 남긴다.
