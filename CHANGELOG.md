# Changelog

## Unreleased

- Add DBC-trusted Palisade body, door, seatbelt, and climate raw diagnostics signals.
- Add a read-only Hyundai Palisade 2020 decoder for cluster speed, steering angle, acceleration, driver braking, and CGW1 low-beam night mode.

## [0.1.0] - 2026-09-21

- Rust CAN library 초기 구조와 CI를 추가했습니다.
- CAN/CAN FD frame primitive와 identifier·payload 경계 검증을 추가했습니다.
- DBC decoder와 Adapter 사이의 transport-neutral decode contract를 추가했습니다.
- Synthetic decode pipeline 검증용 FixtureDecoder를 추가했습니다.
