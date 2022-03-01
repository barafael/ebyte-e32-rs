use snafu::prelude::*;

#[derive(Debug, Snafu)]
pub enum Error {
    #[snafu(display("Failed to read model data"))]
    ReadModelData,

    #[snafu(display("Failed to read parameters"))]
    ReadParameters,

    #[snafu(display("Failed to set parameters"))]
    SetParameters,

    #[snafu(display("Failed to read serial port"))]
    SerialRead,

    #[snafu(display("Failed to write serial port"))]
    SerialWrite,

    #[snafu(display("Invalid UART parity {value}"))]
    InvalidUartParity { value: u8 },

    #[snafu(display("Invalid baud rate {rate}"))]
    InvalidBaudrate { rate: u8 },

    #[snafu(display("Invalid transmission mode {value}"))]
    InvalidTransmissionMode { value: u8 },

    #[snafu(display("Invalid IO drive mode {value}"))]
    InvalidIoDriveMode { value: u8 },

    #[snafu(display("Invalid wakeup time {value}"))]
    InvalidWakeupTime { value: u8 },

    #[snafu(display("Invalid FEC mode {value}"))]
    InvalidFecMode { value: u8 },

    #[snafu(display("Invalid transmission power {value}"))]
    InvalidTransmissionPower { value: u8 },
}
