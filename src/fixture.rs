//! 실제 DBC 없이 decode pipeline을 검증하기 위한 fixture decoder다.

use std::collections::BTreeMap;
use std::convert::Infallible;

use crate::decode::{DecodeContext, DecodedCanMessage, FrameDecoder, SignalValue};
use crate::frame::{CanFrame, CanId};

/// Fixture message가 반환할 DBC-like metadata다.
#[derive(Debug, Clone, PartialEq)]
pub struct FixtureMessage {
    pub message_name: String,
    pub signals: BTreeMap<String, SignalValue>,
}

impl FixtureMessage {
    pub fn new(message_name: impl Into<String>) -> Self {
        Self {
            message_name: message_name.into(),
            signals: BTreeMap::new(),
        }
    }

    pub fn with_signal(mut self, name: impl Into<String>, value: SignalValue) -> Self {
        self.signals.insert(name.into(), value);
        self
    }
}

/// Frame ID로 고정된 synthetic decode 결과를 반환한다.
#[derive(Debug, Clone, Default)]
pub struct FixtureDecoder {
    messages: BTreeMap<CanId, FixtureMessage>,
}

impl FixtureDecoder {
    pub fn insert(&mut self, frame_id: CanId, message: FixtureMessage) {
        self.messages.insert(frame_id, message);
    }
}

impl FrameDecoder for FixtureDecoder {
    type Error = Infallible;

    fn decode(
        &self,
        frame: &CanFrame,
        context: DecodeContext,
    ) -> Result<Option<DecodedCanMessage>, Self::Error> {
        Ok(self.messages.get(&frame.id).map(|fixture| DecodedCanMessage {
            frame_id: frame.id,
            message_name: fixture.message_name.clone(),
            context,
            signals: fixture.signals.clone(),
        }))
    }
}

#[cfg(test)]
mod tests {
    use crate::decode::{DecodeContext, FrameDecoder, SignalValue};
    use crate::fixture::{FixtureDecoder, FixtureMessage};
    use crate::frame::{CanFrame, CanId};

    #[test]
    fn decodes_a_registered_synthetic_frame() {
        let frame_id = CanId::standard(0x123).unwrap();
        let mut decoder = FixtureDecoder::default();
        decoder.insert(
            frame_id,
            FixtureMessage::new("synthetic_status")
                .with_signal("speed_mps", SignalValue::Number(12.5)),
        );

        let frame = CanFrame::new(frame_id, vec![0; 8], false).unwrap();
        let decoded = decoder
            .decode(
                &frame,
                DecodeContext {
                    timestamp_ns: Some(100),
                    bus: 0,
                },
            )
            .unwrap()
            .unwrap();

        assert_eq!(decoded.message_name, "synthetic_status");
        assert_eq!(
            decoded.signals.get("speed_mps"),
            Some(&SignalValue::Number(12.5))
        );
    }
}
