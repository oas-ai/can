# Fixture Decoder

`FixtureDecoder`는 frame ID와 미리 등록한 decoded signal을 연결하는 test/simulation용 구현입니다. DBC를 parse하거나 frame payload를 해석하지 않습니다.

실제 DBC decoder가 도입되면 동일한 `FrameDecoder` contract를 구현해야 합니다. fixture decoder는 contract regression test에만 유지합니다.
