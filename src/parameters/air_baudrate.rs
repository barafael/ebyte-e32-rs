use crate::error::Error;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum AirBaudRate {
    Bps300,
    Bps1200,
    Bps2400,
    Bps4800,
    Bps9600,
    Bps19200,
}

impl Default for AirBaudRate {
    fn default() -> Self {
        Self::Bps1200
    }
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
