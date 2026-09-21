# CAN Frame Contract

`CanFrame`은 transport와 DBC parser가 공유하는 최소 frame 표현입니다.

- Standard ID는 11-bit, Extended ID는 29-bit 범위를 생성 시 검증합니다.
- Classic CAN payload는 최대 8 byte, CAN FD payload는 최대 64 byte입니다.
- 이 타입은 송신 권한이나 차량 제어 의미를 부여하지 않습니다. 송신은 Gateway와 Safety 경계를 거쳐야 합니다.

DBC decoder의 출력은 [`decode::DecodedCanMessage`](../src/decode.rs)입니다. 이 타입은 DBC의 physical value를 전달할 뿐 OEM signal을 OAS public API로 만들지 않습니다. Manufacturer Adapter가 이를 Canonical `VehicleState`로 변환합니다.
