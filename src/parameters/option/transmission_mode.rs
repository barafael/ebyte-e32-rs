use crate::parameters::error::Error;

#[cfg(feature = "value_enum")]
use clap::ValueEnum;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Default)]
#[cfg_attr(test, derive(proptest_derive::Arbitrary))]
#[cfg_attr(feature = "value_enum", derive(ValueEnum))]
pub enum TransmissionMode {
    #[default]
    Transparent,
    Fixed,
}

impl TryFrom<u8> for TransmissionMode {
    type Error = Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Transparent),
            1 => Ok(Self::Fixed),
            _ => Err(Error::InvalidTransmissionMode { value }),
        }
    }
}

impl From<TransmissionMode> for u8 {
    fn from(mode: TransmissionMode) -> Self {
        match mode {
            TransmissionMode::Transparent => 0,
            TransmissionMode::Fixed => 1,
        }
    }
}
