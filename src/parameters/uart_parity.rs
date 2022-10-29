use crate::parameters::error::Error;

#[cfg(feature = "value_enum")]
use clap::ValueEnum;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Default)]
#[cfg_attr(test, derive(proptest_derive::Arbitrary))]
#[cfg_attr(feature = "value_enum", derive(ValueEnum))]
pub enum Parity {
    #[default]
    None,
    Odd,
    Even,
}

impl TryFrom<u8> for Parity {
    type Error = Error;

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

#[cfg(feature = "value_enum")]
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parses_parity() {
        let none = Parity::from_str("None", true).unwrap();
        assert_eq!(none, Parity::None);

        let even = Parity::from_str("Even", true).unwrap();
        assert_eq!(even, Parity::Even);
    }
}
