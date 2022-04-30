use crate::error::Error;
use smart_default::SmartDefault;

#[cfg(feature = "param_fromstr")]
use strum_macros::EnumString;

#[derive(Copy, Clone, Debug, PartialEq, Eq, SmartDefault)]
#[cfg_attr(test, derive(proptest_derive::Arbitrary))]
#[cfg_attr(feature = "param_fromstr", derive(EnumString))]
pub enum AirBaudRate {
    Bps300,
    Bps1200,
    #[default]
    Bps2400,
    Bps4800,
    Bps9600,
    Bps19200,
}

impl TryFrom<u8> for AirBaudRate {
    type Error = super::Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Bps300),
            1 => Ok(Self::Bps1200),
            2 => Ok(Self::Bps2400),
            3 => Ok(Self::Bps4800),
            4 => Ok(Self::Bps9600),
            5 | 6 | 7 => Ok(Self::Bps19200),
            _ => Err(Error::InvalidBaudrate { rate: value }),
        }
    }
}

impl From<AirBaudRate> for u8 {
    fn from(rate: AirBaudRate) -> Self {
        match rate {
            AirBaudRate::Bps300 => 0,
            AirBaudRate::Bps1200 => 1,
            AirBaudRate::Bps2400 => 2,
            AirBaudRate::Bps4800 => 3,
            AirBaudRate::Bps9600 => 4,
            AirBaudRate::Bps19200 => 5,
        }
    }
}
