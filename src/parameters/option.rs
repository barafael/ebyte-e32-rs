use crate::error::Error;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Option {
    transmission_mode: TransmissionMode,
    io_drive_mode: IoDriveMode,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum TransmissionMode {
    Transparent,
    Fixed,
}

impl TryFrom<u8> for TransmissionMode {
    type Error = super::Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Transparent),
            1 => Ok(Self::Fixed),
            _ => Err(Error::InvalidTransmissionMode { value }),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum IoDriveMode {
    PushPull,
    OpenCollector,
}

impl TryFrom<u8> for IoDriveMode {
    type Error = super::Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::OpenCollector),
            1 => Ok(Self::PushPull),
            _ => Err(Error::InvalidIoDriveMode { value }),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum WakeupTime {
    Ms250,
    Ms500,
    Ms750,
    Ms1000,
    Ms1250,
    Ms1500,
    Ms1750,
    Ms2000,
}

impl Default for WakeupTime {
    fn default() -> Self {
        Self::Ms250
    }
}

impl TryFrom<u8> for WakeupTime {
    type Error = super::Error;

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

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ForwardErrorCorrectionMode {
    Off,
    On,
}

impl Default for ForwardErrorCorrectionMode {
    fn default() -> Self {
        Self::On
    }
}

impl TryFrom<u8> for ForwardErrorCorrectionMode {
    type Error = super::Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Off),
            1 => Ok(Self::On),
            _ => Err(Error::InvalidFecMode { value }),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum TransmissionPower {
    Dbm30,
    Dbm27,
    Dbm24,
    Dbm21,
}

impl Default for TransmissionPower {
    fn default() -> Self {
        Self::Dbm30
    }
}

impl TryFrom<u8> for TransmissionPower {
    type Error = super::Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Dbm30),
            1 => Ok(Self::Dbm27),
            2 => Ok(Self::Dbm24),
            3 => Ok(Self::Dbm21),
            _ => Err(Error::InvalidTransmissionPower { value }),
        }
    }
}
