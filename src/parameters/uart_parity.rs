use crate::error::Error;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Parity {
    None,
    Odd,
    Even,
}

impl Default for Parity {
    fn default() -> Self {
        Self::None
    }
}

impl TryFrom<u8> for Parity {
    type Error = super::Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 | 3 => Ok(Self::None),
            1 => Ok(Self::Odd),
            2 => Ok(Self::Even),
            _ => Err(Error::InvalidUartParity { value }),
        }
    }
}
