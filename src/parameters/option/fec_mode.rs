use crate::parameters::error::Error;

#[cfg(feature = "value_enum")]
use clap::ValueEnum;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Default)]
#[cfg_attr(test, derive(proptest_derive::Arbitrary))]
#[cfg_attr(feature = "value_enum", derive(ValueEnum))]
pub enum ForwardErrorCorrectionMode {
    #[default]
    On,
    Off,
}

impl TryFrom<u8> for ForwardErrorCorrectionMode {
    type Error = Error;

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
