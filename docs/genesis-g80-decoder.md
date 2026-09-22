# Genesis G80 Legacy Decoder

`GenesisG80LegacyDecoder`는 `dbc` 저장소의 `GENESIS_G80_2017` provenance manifest에
고정된 Hyundai legacy CAN DBC 중 상태에 필요한 세 메시지만 직접 해석합니다.

| DBC message | 신호 | OAS 용도 |
| --- | --- | --- |
| `CLU11` | `CF_Clu_Vanz`, `CF_Clu_SPEED_UNIT` | 클러스터 속도와 단위 |
| `SAS11` | `SAS_Angle` | 조향각 |
| `TCS13` | `ACCEL_REF_ACC`, `DriverOverride` | 종가속도와 운전자 제동 상태 |

이는 전체 DBC parser가 아니라 승인된 schema의 좁은 read-only 구현이다. 알 수 없는
frame은 정상적으로 무시하고, 알려진 메시지의 payload 길이가 다르면 오류를 반환한다.
