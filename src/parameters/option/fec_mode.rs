use crate::parameters::error::Error;

#[cfg(feature = "arg_enum")]
use clap::ArgEnum;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Default)]
#[cfg_attr(test, derive(proptest_derive::Arbitrary))]
#[cfg_attr(feature = "arg_enum", derive(ArgEnum))]
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
