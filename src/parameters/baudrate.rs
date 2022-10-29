use super::Error;

#[cfg(feature = "arg_enum")]
use clap::ArgEnum;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Default)]
#[cfg_attr(test, derive(proptest_derive::Arbitrary))]
#[cfg_attr(feature = "arg_enum", derive(ArgEnum))]
pub enum BaudRate {
    Bps1200,
    Bps2400,
    Bps4800,
    #[default]
    Bps9600,
    Bps19200,
    Bps38400,
    Bps57600,
    Bps115200,
}

impl TryFrom<u8> for BaudRate {
    type Error = super::Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Bps1200),
            1 => Ok(Self::Bps2400),
            2 => Ok(Self::Bps4800),
            3 => Ok(Self::Bps9600),
            4 => Ok(Self::Bps19200),
            5 => Ok(Self::Bps38400),
            6 => Ok(Self::Bps57600),
            7 => Ok(Self::Bps115200),
            _ => Err(Error::InvalidBaudrate { rate: value }),
        }
    }
}

impl From<BaudRate> for u8 {
    fn from(rate: BaudRate) -> Self {
        match rate {
            BaudRate::Bps1200 => 0,
            BaudRate::Bps2400 => 1,
            BaudRate::Bps4800 => 2,
            BaudRate::Bps9600 => 3,
            BaudRate::Bps19200 => 4,
            BaudRate::Bps38400 => 5,
            BaudRate::Bps57600 => 6,
            BaudRate::Bps115200 => 7,
        }
    }
}
