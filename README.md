# can

OAS의 CAN/CAN FD frame, parser 및 transport를 제공하는 Rust library입니다.

이 저장소는 OEM별 DBC나 차량 Adapter를 포함하지 않습니다. Rust stable, Cargo, rustfmt, Clippy를 사용합니다.

`genesis_g80_legacy`는 승인된 Genesis G80 2017 DBC의 read-only subset(`CLU11`, `SAS11`,
`TCS13`)만 해석하는 예외입니다. 전체 DBC parser나 CAN 송신 기능을 제공하지 않습니다.
