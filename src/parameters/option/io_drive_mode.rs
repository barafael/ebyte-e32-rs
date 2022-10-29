use crate::parameters::error::Error;

#[cfg(feature = "value_enum")]
use clap::ValueEnum;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Default)]
#[cfg_attr(test, derive(proptest_derive::Arbitrary))]
#[cfg_attr(feature = "value_enum", derive(ValueEnum))]
pub enum IoDriveMode {
    #[default]
    PushPull,
    OpenCollector,
}

impl TryFrom<u8> for IoDriveMode {
    type Error = Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::OpenCollector),
            1 => Ok(Self::PushPull),
            _ => Err(Error::InvalidIoDriveMode { value }),
        }
    }
}

impl From<IoDriveMode> for u8 {
    fn from(mode: IoDriveMode) -> Self {
        match mode {
            IoDriveMode::PushPull => 1,
            IoDriveMode::OpenCollector => 0,
        }
    }
}
