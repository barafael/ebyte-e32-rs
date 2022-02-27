use crate::error::Error;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum BaudRate {
    Bps1200,
    Bps2400,
    Bps4800,
    Bps9600,
    Bps19200,
    Bps38400,
    Bps57600,
    Bps115200,
}

impl Default for BaudRate {
    fn default() -> Self {
        Self::Bps9600
    }
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
