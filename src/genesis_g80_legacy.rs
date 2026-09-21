//! Genesis G80 2017 legacy CAN의 read-only DBC subset decoder다.

use crate::decode::{DecodeContext, DecodedCanMessage, FrameDecoder, SignalValue};
use crate::frame::{CanFrame, CanId};

const CLU11: u16 = 1265;
const SAS11: u16 = 688;
const TCS13: u16 = 916;

/// 승인된 Genesis DBC subset을 해석하지 못한 오류다.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GenesisG80LegacyDecodeError {
    UnexpectedFrameLength {
        message_name: &'static str,
        expected: usize,
        actual: usize,
    },
}

/// Genesis G80 2017의 `hyundai_can.dbc` read-only subset decoder다.
#[derive(Debug, Default)]
pub struct GenesisG80LegacyDecoder;

impl GenesisG80LegacyDecoder {
    fn standard_id(frame: &CanFrame) -> Option<u16> {
        match frame.id {
            CanId::Standard(id) => Some(id),
            CanId::Extended(_) => None,
        }
    }

    fn data_as_u64(
        frame: &CanFrame,
        message_name: &'static str,
        expected: usize,
    ) -> Result<u64, GenesisG80LegacyDecodeError> {
        if frame.data.len() != expected {
            return Err(GenesisG80LegacyDecodeError::UnexpectedFrameLength {
                message_name,
                expected,
                actual: frame.data.len(),
            });
        }

        let mut bytes = [0; 8];
        bytes[..expected].copy_from_slice(&frame.data);
        Ok(u64::from_le_bytes(bytes))
    }

    fn decode_clu11(
        frame: &CanFrame,
        context: DecodeContext,
    ) -> Result<DecodedCanMessage, GenesisG80LegacyDecodeError> {
        let data = Self::data_as_u64(frame, "CLU11", 4)?;
        let speed = ((data >> 8) & 0x1ff) as f64 * 0.5 + ((data >> 6) & 0x3) as f64 * 0.125;
        let unit = if (data >> 17) & 1 == 0 { "kph" } else { "mph" };

        let mut message = DecodedCanMessage::new(frame.id, "CLU11", context);
        message.insert_signal("CF_Clu_Vanz", SignalValue::Number(speed));
        message.insert_signal("CF_Clu_SPEED_UNIT", SignalValue::Enumeration(unit.into()));
        Ok(message)
    }

    fn decode_sas11(
        frame: &CanFrame,
        context: DecodeContext,
    ) -> Result<DecodedCanMessage, GenesisG80LegacyDecodeError> {
        let data = Self::data_as_u64(frame, "SAS11", 5)?;
        let angle = (data as i16) as f64 * 0.1;

        let mut message = DecodedCanMessage::new(frame.id, "SAS11", context);
        message.insert_signal("SAS_Angle", SignalValue::Number(angle));
        Ok(message)
    }

    fn decode_tcs13(
        frame: &CanFrame,
        context: DecodeContext,
    ) -> Result<DecodedCanMessage, GenesisG80LegacyDecodeError> {
        let data = Self::data_as_u64(frame, "TCS13", 8)?;
        let driver_override = ((data >> 45) & 0x3) as f64;

        let mut message = DecodedCanMessage::new(frame.id, "TCS13", context);
        message.insert_signal("DriverOverride", SignalValue::Number(driver_override));
        Ok(message)
    }
}

impl FrameDecoder for GenesisG80LegacyDecoder {
    type Error = GenesisG80LegacyDecodeError;

    fn decode(
        &self,
        frame: &CanFrame,
        context: DecodeContext,
    ) -> Result<Option<DecodedCanMessage>, Self::Error> {
        let decoded = match Self::standard_id(frame) {
            Some(CLU11) => Some(Self::decode_clu11(frame, context)?),
            Some(SAS11) => Some(Self::decode_sas11(frame, context)?),
            Some(TCS13) => Some(Self::decode_tcs13(frame, context)?),
            _ => None,
        };
        Ok(decoded)
    }
}

#[cfg(test)]
mod tests {
    use crate::decode::{DecodeContext, FrameDecoder, SignalValue};
    use crate::frame::{CanFrame, CanId};
    use crate::genesis_g80_legacy::{GenesisG80LegacyDecodeError, GenesisG80LegacyDecoder};

    fn context() -> DecodeContext {
        DecodeContext {
            timestamp_ns: Some(1_000),
            bus: 0,
        }
    }

    #[test]
    fn decodes_cluster_speed_in_kph() {
        let frame =
            CanFrame::new(CanId::standard(1265).unwrap(), vec![0, 160, 0, 0], false).unwrap();
        let message = GenesisG80LegacyDecoder
            .decode(&frame, context())
            .unwrap()
            .unwrap();

        assert_eq!(message.message_name, "CLU11");
        assert_eq!(
            message.signals.get("CF_Clu_Vanz"),
            Some(&SignalValue::Number(80.0))
        );
        assert_eq!(
            message.signals.get("CF_Clu_SPEED_UNIT"),
            Some(&SignalValue::Enumeration("kph".into()))
        );
    }

    #[test]
    fn decodes_steering_and_driver_braking() {
        let steering =
            CanFrame::new(CanId::standard(688).unwrap(), vec![132, 3, 0, 0, 0], false).unwrap();
        let braking = CanFrame::new(
            CanId::standard(916).unwrap(),
            vec![0, 0, 0, 0, 0, 64, 0, 0],
            false,
        )
        .unwrap();

        let steering_message = GenesisG80LegacyDecoder
            .decode(&steering, context())
            .unwrap()
            .unwrap();
        let braking_message = GenesisG80LegacyDecoder
            .decode(&braking, context())
            .unwrap()
            .unwrap();

        assert_eq!(
            steering_message.signals.get("SAS_Angle"),
            Some(&SignalValue::Number(90.0))
        );
        assert_eq!(
            braking_message.signals.get("DriverOverride"),
            Some(&SignalValue::Number(2.0))
        );
    }

    #[test]
    fn rejects_a_known_message_with_an_unexpected_length() {
        let frame = CanFrame::new(CanId::standard(688).unwrap(), vec![0; 4], false).unwrap();

        assert_eq!(
            GenesisG80LegacyDecoder.decode(&frame, context()).unwrap_err(),
            GenesisG80LegacyDecodeError::UnexpectedFrameLength {
                message_name: "SAS11",
                expected: 5,
                actual: 4,
            }
        );
    }
}
