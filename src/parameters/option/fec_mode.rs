use crate::error::Error;
use smart_default::SmartDefault;

#[derive(Copy, Clone, Debug, PartialEq, Eq, SmartDefault)]
#[cfg_attr(test, derive(proptest_derive::Arbitrary))]
pub enum ForwardErrorCorrectionMode {
    #[default]
    On,
    Off,
}

impl TryFrom<u8> for ForwardErrorCorrectionMode {
    type Error = super::Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Off),
            1 => Ok(Self::On),
            _ => Err(Error::InvalidFecMode { value }),
        }
    }
}

impl From<ForwardErrorCorrectionMode> for u8 {
    fn from(mode: ForwardErrorCorrectionMode) -> Self {
        match mode {
            ForwardErrorCorrectionMode::Off => 0,
            ForwardErrorCorrectionMode::On => 1,
        }
    }
}
