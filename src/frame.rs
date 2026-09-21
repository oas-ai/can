//! CAN identifier와 frame payload의 공통 표현이다.

/// Classic CAN의 최대 payload 길이다.
pub const CLASSIC_CAN_MAX_DATA_LENGTH: usize = 8;

/// CAN FD의 최대 payload 길이다.
pub const CAN_FD_MAX_DATA_LENGTH: usize = 64;

/// 표준 또는 확장 CAN identifier다.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CanId {
    Standard(u16),
    Extended(u32),
}

impl CanId {
    /// 11-bit 표준 identifier를 검증해 생성한다.
    pub fn standard(value: u16) -> Result<Self, CanFrameError> {
        if value <= 0x7ff {
            Ok(Self::Standard(value))
        } else {
            Err(CanFrameError::InvalidStandardId(value))
        }
    }

    /// 29-bit 확장 identifier를 검증해 생성한다.
    pub fn extended(value: u32) -> Result<Self, CanFrameError> {
        if value <= 0x1fff_ffff {
            Ok(Self::Extended(value))
        } else {
            Err(CanFrameError::InvalidExtendedId(value))
        }
    }
}

/// CAN 또는 CAN FD frame이다.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CanFrame {
    pub id: CanId,
    pub data: Vec<u8>,
    pub is_fd: bool,
}

impl CanFrame {
    /// payload 길이를 검증해 frame을 생성한다.
    pub fn new(id: CanId, data: Vec<u8>, is_fd: bool) -> Result<Self, CanFrameError> {
        let maximum_length = if is_fd {
            CAN_FD_MAX_DATA_LENGTH
        } else {
            CLASSIC_CAN_MAX_DATA_LENGTH
        };

        if data.len() > maximum_length {
            return Err(CanFrameError::PayloadTooLarge {
                actual: data.len(),
                maximum: maximum_length,
            });
        }

        Ok(Self { id, data, is_fd })
    }
}

/// Frame 경계 검증 오류다.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CanFrameError {
    InvalidStandardId(u16),
    InvalidExtendedId(u32),
    PayloadTooLarge { actual: usize, maximum: usize },
}

#[cfg(test)]
mod tests {
    use super::{CanFrame, CanFrameError, CanId, CAN_FD_MAX_DATA_LENGTH};

    #[test]
    fn accepts_a_standard_classic_can_frame() {
        let frame = CanFrame::new(CanId::standard(0x123).unwrap(), vec![0; 8], false).unwrap();

        assert!(!frame.is_fd);
    }

    #[test]
    fn rejects_a_classic_can_payload_larger_than_eight_bytes() {
        let error = CanFrame::new(CanId::standard(0x123).unwrap(), vec![0; 9], false)
            .unwrap_err();

        assert_eq!(
            error,
            CanFrameError::PayloadTooLarge {
                actual: 9,
                maximum: 8,
            }
        );
    }

    #[test]
    fn accepts_a_maximum_sized_can_fd_frame() {
        let frame = CanFrame::new(
            CanId::extended(0x1fff_ffff).unwrap(),
            vec![0; CAN_FD_MAX_DATA_LENGTH],
            true,
        )
        .unwrap();

        assert!(frame.is_fd);
    }
}
