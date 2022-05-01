use crate::parameters::error::Error;
use smart_default::SmartDefault;

#[cfg(feature = "arg_enum")]
use clap::ArgEnum;

#[derive(Copy, Clone, Debug, PartialEq, Eq, SmartDefault)]
#[cfg_attr(test, derive(proptest_derive::Arbitrary))]
#[cfg_attr(feature = "arg_enum", derive(ArgEnum))]
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
