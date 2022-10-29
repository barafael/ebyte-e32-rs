use crate::parameters::error::Error;

#[cfg(feature = "arg_enum")]
use clap::ArgEnum;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Default)]
#[cfg_attr(test, derive(proptest_derive::Arbitrary))]
#[cfg_attr(feature = "arg_enum", derive(ArgEnum))]
pub enum WakeupTime {
    #[default]
    Ms250,
    Ms500,
    Ms750,
    Ms1000,
    Ms1250,
    Ms1500,
    Ms1750,
    Ms2000,
}

impl TryFrom<u8> for WakeupTime {
    type Error = Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Ms250),
            1 => Ok(Self::Ms500),
            2 => Ok(Self::Ms750),
            3 => Ok(Self::Ms1000),
            4 => Ok(Self::Ms1250),
            5 => Ok(Self::Ms1500),
            6 => Ok(Self::Ms1750),
            7 => Ok(Self::Ms2000),
            _ => Err(Error::InvalidWakeupTime { value }),
        }
    }
}

impl From<WakeupTime> for u8 {
    fn from(time: WakeupTime) -> Self {
        match time {
            WakeupTime::Ms250 => 0,
            WakeupTime::Ms500 => 1,
            WakeupTime::Ms750 => 2,
            WakeupTime::Ms1000 => 3,
            WakeupTime::Ms1250 => 4,
            WakeupTime::Ms1500 => 5,
            WakeupTime::Ms1750 => 6,
            WakeupTime::Ms2000 => 7,
        }
    }
}
