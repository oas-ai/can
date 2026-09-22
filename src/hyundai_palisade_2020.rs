//! Hyundai Palisade 2020 CAN의 read-only DBC subset decoder다.

use crate::decode::{DecodeContext, DecodedCanMessage, FrameDecoder, SignalValue};
use crate::frame::{CanFrame, CanId};

const CLU11: u16 = 1265;
const SAS11: u16 = 688;
const TCS13: u16 = 916;
const CGW1: u16 = 1345;
const LVR12: u16 = 871;
const WHL_SPD11: u16 = 902;
const SCC14: u16 = 905;
const GW_DDM_PE: u16 = 1313;

/// 승인된 Hyundai Palisade DBC subset을 해석하지 못한 오류다.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HyundaiPalisade2020DecodeError {
    UnexpectedFrameLength {
        message_name: &'static str,
        expected: usize,
        actual: usize,
    },
}

/// Hyundai Palisade 2020의 `hyundai_can.dbc` read-only subset decoder다.
#[derive(Debug, Default)]
pub struct HyundaiPalisade2020Decoder;

impl HyundaiPalisade2020Decoder {
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
    ) -> Result<u64, HyundaiPalisade2020DecodeError> {
        if frame.data.len() != expected {
            return Err(HyundaiPalisade2020DecodeError::UnexpectedFrameLength {
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
    ) -> Result<DecodedCanMessage, HyundaiPalisade2020DecodeError> {
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
    ) -> Result<DecodedCanMessage, HyundaiPalisade2020DecodeError> {
        let data = Self::data_as_u64(frame, "SAS11", 5)?;
        let angle = (data as i16) as f64 * 0.1;

        let mut message = DecodedCanMessage::new(frame.id, "SAS11", context);
        message.insert_signal("SAS_Angle", SignalValue::Number(angle));
        Ok(message)
    }

    fn decode_tcs13(
        frame: &CanFrame,
        context: DecodeContext,
    ) -> Result<DecodedCanMessage, HyundaiPalisade2020DecodeError> {
        let data = Self::data_as_u64(frame, "TCS13", 8)?;
        let acceleration = ((data >> 32) & 0x7ff) as f64 * 0.01 - 10.23;
        let driver_override = ((data >> 45) & 0x3) as f64;

        let mut message = DecodedCanMessage::new(frame.id, "TCS13", context);
        message.insert_signal("ACCEL_REF_ACC", SignalValue::Number(acceleration));
        message.insert_signal("DriverOverride", SignalValue::Number(driver_override));
        Ok(message)
    }

    fn decode_cgw1(
        frame: &CanFrame,
        context: DecodeContext,
    ) -> Result<DecodedCanMessage, HyundaiPalisade2020DecodeError> {
        let data = Self::data_as_u64(frame, "CGW1", 8)?;
        let mut message = DecodedCanMessage::new(frame.id, "CGW1", context);
        message.insert_signal(
            "CF_Gway_HeadLampLow",
            SignalValue::Number(((data >> 31) & 1) as f64),
        );
        for (signal, shift, mask) in [
            ("CF_Gway_DrvDrSw", 8, 0x03),
            ("CF_Gway_DrvSeatBeltSw", 10, 0x03),
            ("CF_Gway_AstSeatBeltSw", 14, 0x03),
            ("CF_Gway_WiperIntSw", 24, 0x01),
            ("CF_Gway_WiperLowSw", 25, 0x01),
            ("CF_Gway_WiperHighSw", 26, 0x01),
            ("CF_Gway_WiperAutoSw", 27, 0x01),
            ("CF_Gway_ALightStat", 37, 0x01),
            ("CF_Gway_LightSwState", 38, 0x03),
        ] {
            message.insert_signal(signal, SignalValue::Number(((data >> shift) & mask) as f64));
        }
        Ok(message)
    }

    fn decode_door_status(
        frame: &CanFrame,
        context: DecodeContext,
    ) -> Result<DecodedCanMessage, HyundaiPalisade2020DecodeError> {
        let data = Self::data_as_u64(frame, "GW_DDM_PE", 8)?;
        let mut message = DecodedCanMessage::new(frame.id, "GW_DDM_PE", context);
        for (signal, shift) in [
            ("C_DRVDoorStatus", 0),
            ("C_ASTDoorStatus", 2),
            ("C_RLDoorStatus", 4),
            ("C_RRDoorStatus", 6),
        ] {
            message.insert_signal(signal, SignalValue::Number(((data >> shift) & 0x03) as f64));
        }
        Ok(message)
    }

    fn decode_lvr12(
        frame: &CanFrame,
        context: DecodeContext,
    ) -> Result<DecodedCanMessage, HyundaiPalisade2020DecodeError> {
        let data = Self::data_as_u64(frame, "LVR12", 8)?;
        let mut message = DecodedCanMessage::new(frame.id, "LVR12", context);
        message.insert_signal(
            "CF_Lvr_Gear",
            SignalValue::Number(((data >> 32) & 0x0f) as f64),
        );
        Ok(message)
    }

    fn decode_wheel_speed(
        frame: &CanFrame,
        context: DecodeContext,
    ) -> Result<DecodedCanMessage, HyundaiPalisade2020DecodeError> {
        let data = Self::data_as_u64(frame, "WHL_SPD11", 8)?;
        let mut message = DecodedCanMessage::new(frame.id, "WHL_SPD11", context);
        for (signal, shift) in [
            ("WHL_SPD_FL", 0),
            ("WHL_SPD_FR", 16),
            ("WHL_SPD_RL", 32),
            ("WHL_SPD_RR", 48),
        ] {
            message.insert_signal(
                signal,
                SignalValue::Number(((data >> shift) & 0x3fff) as f64 * 0.03125),
            );
        }
        Ok(message)
    }

    fn decode_scc14(
        frame: &CanFrame,
        context: DecodeContext,
    ) -> Result<DecodedCanMessage, HyundaiPalisade2020DecodeError> {
        let data = Self::data_as_u64(frame, "SCC14", 8)?;
        let mut message = DecodedCanMessage::new(frame.id, "SCC14", context);
        message.insert_signal("ACCMode", SignalValue::Number(((data >> 32) & 0x07) as f64));
        Ok(message)
    }
}

impl FrameDecoder for HyundaiPalisade2020Decoder {
    type Error = HyundaiPalisade2020DecodeError;

    fn decode(
        &self,
        frame: &CanFrame,
        context: DecodeContext,
    ) -> Result<Option<DecodedCanMessage>, Self::Error> {
        let decoded = match Self::standard_id(frame) {
            Some(CLU11) => Some(Self::decode_clu11(frame, context)?),
            Some(SAS11) => Some(Self::decode_sas11(frame, context)?),
            Some(TCS13) => Some(Self::decode_tcs13(frame, context)?),
            Some(CGW1) => Some(Self::decode_cgw1(frame, context)?),
            Some(LVR12) => Some(Self::decode_lvr12(frame, context)?),
            Some(WHL_SPD11) => Some(Self::decode_wheel_speed(frame, context)?),
            Some(SCC14) => Some(Self::decode_scc14(frame, context)?),
            Some(GW_DDM_PE) => Some(Self::decode_door_status(frame, context)?),
            _ => None,
        };
        Ok(decoded)
    }
}

#[cfg(test)]
mod tests {
    use crate::decode::{DecodeContext, FrameDecoder, SignalValue};
    use crate::frame::{CanFrame, CanId};
    use crate::hyundai_palisade_2020::{
        HyundaiPalisade2020DecodeError, HyundaiPalisade2020Decoder,
    };

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
        let message = HyundaiPalisade2020Decoder
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
    fn decodes_steering_acceleration_and_driver_braking() {
        let steering =
            CanFrame::new(CanId::standard(688).unwrap(), vec![132, 3, 0, 0, 0], false).unwrap();
        let accelerating_and_braking = CanFrame::new(
            CanId::standard(916).unwrap(),
            vec![0, 0, 0, 0, 124, 68, 0, 0],
            false,
        )
        .unwrap();

        let steering_message = HyundaiPalisade2020Decoder
            .decode(&steering, context())
            .unwrap()
            .unwrap();
        let braking_message = HyundaiPalisade2020Decoder
            .decode(&accelerating_and_braking, context())
            .unwrap()
            .unwrap();

        assert_eq!(
            steering_message.signals.get("SAS_Angle"),
            Some(&SignalValue::Number(90.0))
        );
        assert_eq!(
            braking_message.signals.get("ACCEL_REF_ACC"),
            Some(&SignalValue::Number(1.25))
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
            HyundaiPalisade2020Decoder
                .decode(&frame, context())
                .unwrap_err(),
            HyundaiPalisade2020DecodeError::UnexpectedFrameLength {
                message_name: "SAS11",
                expected: 5,
                actual: 4,
            }
        );
    }

    #[test]
    fn decodes_low_beam_as_night_signal() {
        let frame = CanFrame::new(
            CanId::standard(1345).unwrap(),
            vec![0, 0, 0, 128, 0, 0, 0, 0],
            false,
        )
        .unwrap();
        let message = HyundaiPalisade2020Decoder
            .decode(&frame, context())
            .unwrap()
            .unwrap();
        assert_eq!(
            message.signals.get("CF_Gway_HeadLampLow"),
            Some(&SignalValue::Number(1.0))
        );
    }

    #[test]
    fn decodes_lvr12_gear() {
        let frame = CanFrame::new(
            CanId::standard(871).unwrap(),
            vec![0, 0, 0, 0, 5, 0, 0, 0],
            false,
        )
        .unwrap();
        let message = HyundaiPalisade2020Decoder
            .decode(&frame, context())
            .unwrap()
            .unwrap();

        assert_eq!(
            message.signals.get("CF_Lvr_Gear"),
            Some(&SignalValue::Number(5.0))
        );
    }

    #[test]
    fn decodes_wheel_speeds_and_cruise_mode() {
        let wheel_speeds = CanFrame::new(
            CanId::standard(902).unwrap(),
            vec![0, 32, 0, 16, 0, 8, 0, 4],
            false,
        )
        .unwrap();
        let cruise = CanFrame::new(
            CanId::standard(905).unwrap(),
            vec![0, 0, 0, 0, 1, 0, 0, 0],
            false,
        )
        .unwrap();

        let wheels = HyundaiPalisade2020Decoder
            .decode(&wheel_speeds, context())
            .unwrap()
            .unwrap();
        let cruise = HyundaiPalisade2020Decoder
            .decode(&cruise, context())
            .unwrap()
            .unwrap();

        assert_eq!(
            wheels.signals.get("WHL_SPD_FL"),
            Some(&SignalValue::Number(256.0))
        );
        assert_eq!(
            wheels.signals.get("WHL_SPD_RR"),
            Some(&SignalValue::Number(32.0))
        );
        assert_eq!(
            cruise.signals.get("ACCMode"),
            Some(&SignalValue::Number(1.0))
        );
    }
}
