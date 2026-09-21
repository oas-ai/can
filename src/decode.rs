//! DBC decoder와 Manufacturer Adapter 사이의 transport-neutral 계약이다.

use std::collections::BTreeMap;

use crate::frame::{CanFrame, CanId};

/// 수신 frame의 관측 context다.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct DecodeContext {
    /// 관측 시각(Unix epoch ns)이다.
    pub timestamp_ns: Option<u64>,
    /// CAN bus 식별자다.
    pub bus: u8,
}

/// DBC physical value다. OEM signal의 원시 bit 표현은 노출하지 않는다.
#[derive(Debug, Clone, PartialEq)]
pub enum SignalValue {
    Number(f64),
    Boolean(bool),
    Enumeration(String),
}

/// 단일 CAN frame을 DBC로 해석한 결과다.
#[derive(Debug, Clone, PartialEq)]
pub struct DecodedCanMessage {
    pub frame_id: CanId,
    pub message_name: String,
    pub context: DecodeContext,
    pub signals: BTreeMap<String, SignalValue>,
}

impl DecodedCanMessage {
    /// 빈 decoded message를 생성한다.
    pub fn new(frame_id: CanId, message_name: impl Into<String>, context: DecodeContext) -> Self {
        Self {
            frame_id,
            message_name: message_name.into(),
            context,
            signals: BTreeMap::new(),
        }
    }

    /// DBC signal의 physical value를 기록한다.
    pub fn insert_signal(&mut self, name: impl Into<String>, value: SignalValue) {
        self.signals.insert(name.into(), value);
    }
}

/// CAN frame을 승인된 DBC 정의로 해석한다.
///
/// `Ok(None)`은 현재 DBC가 해당 frame을 정의하지 않은 정상 상황이다.
pub trait FrameDecoder {
    type Error;

    fn decode(
        &self,
        frame: &CanFrame,
        context: DecodeContext,
    ) -> Result<Option<DecodedCanMessage>, Self::Error>;
}

#[cfg(test)]
mod tests {
    use super::{DecodeContext, DecodedCanMessage, SignalValue};
    use crate::frame::CanId;

    #[test]
    fn stores_signals_in_deterministic_name_order() {
        let context = DecodeContext {
            timestamp_ns: Some(42),
            bus: 0,
        };
        let mut message = DecodedCanMessage::new(
            CanId::standard(0x123).unwrap(),
            "example_message",
            context,
        );

        message.insert_signal("speed", SignalValue::Number(12.5));
        message.insert_signal("enabled", SignalValue::Boolean(true));

        assert_eq!(message.context.timestamp_ns, Some(42));
        assert_eq!(message.signals.len(), 2);
        assert_eq!(
            message.signals.get("speed"),
            Some(&SignalValue::Number(12.5))
        );
    }
}
