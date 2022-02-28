use crate::error::Error;
use smart_default::SmartDefault;

#[derive(Copy, Clone, Debug, PartialEq, Eq, SmartDefault)]
#[cfg_attr(test, derive(proptest_derive::Arbitrary))]
pub enum Parity {
    #[default]
    None,
    Odd,
    Even,
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

impl From<Parity> for u8 {
    fn from(parity: Parity) -> Self {
        match parity {
            Parity::None => 0,
            Parity::Odd => 1,
            Parity::Even => 2,
        }
    }
}
